/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine_wasm_backend_traits::WasmBackend;

use marine_core::generic::HostImportDescriptor;
use marine_rs_sdk::MountedBinaryResult;

use wasmer_it::IValue;
use wasmer_it::IType;

use std::path::Path;
use std::path::PathBuf;

pub(crate) fn create_mounted_binary_import<WB: WasmBackend>(
    mounted_binary_path: PathBuf,
) -> HostImportDescriptor<WB> {
    let host_cmd_closure = move |_ctx: &mut <WB as WasmBackend>::ImportCallContext<'_>,
                                 raw_args: Vec<IValue>| {
        let result =
            mounted_binary_import_impl(&mounted_binary_path, raw_args).unwrap_or_else(Into::into);

        let raw_result = crate::to_interface_value(&result).unwrap();

        Some(raw_result)
    };

    HostImportDescriptor {
        host_exported_func: Box::new(host_cmd_closure),
        argument_types: vec![IType::Array(Box::new(IType::String))],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}

fn mounted_binary_import_impl(
    mounted_binary_path: &Path,
    raw_args: Vec<IValue>,
) -> Result<MountedBinaryResult, MountedBinaryResult> {
    let args = parse_args(raw_args)?;

    let result = std::process::Command::new(mounted_binary_path)
        .args(&args)
        .output();

    let result = match result {
        Ok(output) => {
            const TERMINATED_BY_SIGNAL_CODE: i32 = 100000;
            let ret_code = output.status.code().unwrap_or(TERMINATED_BY_SIGNAL_CODE);

            MountedBinaryResult {
                ret_code,
                error: String::new(),
                stdout: output.stdout,
                stderr: output.stderr,
            }
        }
        Err(e) => {
            const COMMAND_ERROR_CODE: i32 = 100001;
            let error = format!("{}", e);

            log::error!(
                "error occurred on `{} {:?}`: {} ",
                mounted_binary_path.display(),
                args,
                e
            );

            MountedBinaryResult {
                ret_code: COMMAND_ERROR_CODE,
                error,
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        }
    };

    Ok(result)
}

fn parse_args(mut raw_args: Vec<IValue>) -> Result<Vec<String>, MountedBinaryResult> {
    if raw_args.len() != 1 {
        return Err(MountedBinaryResult::from_error(100002, "internal error is encountered while passing arguments to a mounted binary closure, probably you use a not suitable version of rust-sdk"));
    }

    let args = match raw_args.remove(0) {
        IValue::Array(array) => {
            let mut args = Vec::with_capacity(array.len());
            for value in array {
                match value {
                    IValue::String(str) => args.push(str),
                    _ => return Err(MountedBinaryResult::from_error(100004, "internal error is encountered while passing arguments to a mounted binary closure, probably you use a not suitable version of rust-sdk")),
                }
            }

            args
        }
        _ => {
            return Err(MountedBinaryResult::from_error(100003, "internal error is encountered while passing arguments to a mounted binary closure, probably you use a not suitable version of rust-sdk"));
        }
    };

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::mounted_binary_import_impl;

    #[test]
    fn call_non_existent_binary() {
        let path = std::path::Path::new("____non_existent_path____");
        let actual = mounted_binary_import_impl(path, vec![]).unwrap_err();

        assert_eq!(actual.ret_code, 100002);
    }
}
