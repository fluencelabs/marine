/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use fce::HostImportDescriptor;
use wasmer_core::vm::Ctx;
use wasmer_wit::IValue;
use wasmer_wit::IType;

pub(crate) fn create_host_import(host_cmd: String) -> HostImportDescriptor {
    let host_cmd_closure = move |_ctx: &mut Ctx, args: Vec<IValue>| {
        let arg = match &args[0] {
            IValue::String(str) => str,
            // this closure will be linked to import function with signature from supplied
            // HostImportDescriptor. So it should be invoked only with string as an arg.
            _ => unreachable!(),
        };

        let result = match cmd_lib::run_fun!("{} {}", host_cmd, arg) {
            Ok(result) => result,
            Err(e) => {
                log::error!("error occurred `{} {}`: {:?} ", host_cmd, arg, e);
                String::new()
            }
        };

        Some(IValue::String(result))
    };

    HostImportDescriptor {
        host_exported_func: Box::new(host_cmd_closure),
        argument_types: vec![IType::String],
        output_type: Some(IType::String),
        error_handler: None,
    }
}
