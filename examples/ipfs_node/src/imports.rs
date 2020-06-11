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

use wasmer_core::vm::Ctx;
use wasmer_core::typed_func::DynamicFunc;
use wasmer_core::typed_func::WasmTypeList;
use wasmer_core::types::Value;
use wasmer_core::types::Type;
use wasmer_core::types::FuncSig;
use wasmer_runtime::Func;
use wasmer_runtime::error::ResolveError;
use wasmer_core::backend::SigRegistry;
use wasmer_runtime::types::LocalOrImport;
use wasmer_core::module::ExportIndex;

use std::collections::HashMap;
use cmd_lib::run_fun;

const ALLOCATE_FUNC_NAME: &str = "allocate";
const SET_PTR_FUNC_NAME: &str = "set_result_ptr";
const SET_SIZE_FUNC_NAME: &str = "set_result_size";

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => println!("ipfs node logger: incorrect UTF8 string's been supplied to logger"),
    }
}

// based on Wasmer: https://github.com/wasmerio/wasmer/blob/081f6250e69b98b9f95a8f62ad6d8386534f3279/lib/runtime-core/src/instance.rs#L863
unsafe fn get_export_func_by_name<'a, Args, Rets>(
    ctx: &'a mut Ctx,
    name: &str,
) -> Result<Func<'a, Args, Rets>, ResolveError>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    let module_inner = &(*ctx.module);

    let export_index =
        module_inner
            .info
            .exports
            .get(name)
            .ok_or_else(|| ResolveError::ExportNotFound {
                name: name.to_string(),
            })?;

    let export_func_index = match export_index {
        ExportIndex::Func(func_index) => func_index,
        _ => {
            return Err(ResolveError::ExportWrongType {
                name: name.to_string(),
            })
        }
    };

    let export_func_signature_idx = *module_inner
        .info
        .func_assoc
        .get(*export_func_index)
        .expect("broken invariant, incorrect func index");

    let export_func_signature = &module_inner.info.signatures[export_func_signature_idx];
    let export_func_signature_ref = SigRegistry.lookup_signature_ref(export_func_signature);

    if export_func_signature_ref.params() != Args::types()
        || export_func_signature_ref.returns() != Rets::types()
    {
        return Err(ResolveError::Signature {
            expected: (*export_func_signature).clone(),
            found: Args::types().to_vec(),
        });
    }

    let func_wasm_inner = module_inner
        .runnable_module
        .get_trampoline(&module_inner.info, export_func_signature_idx)
        .unwrap();

    let export_func_ptr = match export_func_index.local_or_import(&module_inner.info) {
        LocalOrImport::Local(local_func_index) => module_inner
            .runnable_module
            .get_func(&module_inner.info, local_func_index)
            .unwrap(),
        _ => {
            return Err(ResolveError::ExportNotFound {
                name: name.to_string(),
            })
        }
    };

    let typed_func: Func<Args, Rets, wasmer_core::typed_func::Wasm> =
        Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, ctx as _);

    Ok(typed_func)
}

#[allow(dead_code)]
fn to_full_path<S>(cmd: S, mapped_dirs: &HashMap<String, String>) -> String
where
    S: Into<String>,
{
    use std::str::pattern::Pattern;

    fn find_start_at<'a, P: Pattern<'a>>(slice: &'a str, at: usize, pat: P) -> Option<usize> {
        slice[at..].find(pat).map(|i| at + i)
    }

    let cmd = cmd.into();

    if cmd.is_empty() || mapped_dirs.is_empty() {
        return cmd;
    }

    // assume that string is started with /
    let from_dir = if let Some(found_pos) = find_start_at(&cmd, 1, '/') {
        // it is safe because we are splitting on the found position
        cmd.split_at(found_pos)
    } else {
        (cmd.as_str(), "")
    };

    match mapped_dirs.get(from_dir.0) {
        Some(to_dir) => {
            let ret = format!("{}/{}", to_dir, from_dir.1);
            println!("ret is {}", ret);
            ret
        }
        None => cmd,
    }
}

fn write_to_mem(context: &mut Ctx, address: usize, value: &[u8]) {
    let memory = context.memory(0);

    for (byte_id, cell) in memory.view::<u8>()[address as usize..(address + value.len())]
        .iter()
        .enumerate()
    {
        cell.set(value[byte_id]);
    }
}

pub(super) fn create_host_import_func<S>(host_cmd: S) -> DynamicFunc<'static>
where
    S: Into<String>,
{
    /*
    let mut allocate_func: Option<Func<'static, i32, i32>> = None;
    let mut set_result_ptr: Option<Func<'static, i32, ()>> = None;
    let mut set_result_size: Option<Func<'static, i32, ()>> = None;
     */

    let host_cmd = host_cmd.into();

    let func = move |ctx: &mut Ctx, inputs: &[Value]| -> Vec<Value> {
        use wasmer_core::memory::ptr::{Array, WasmPtr};

        let array_ptr = inputs[0].to_u128() as i32;
        let array_size = inputs[1].to_u128() as i32;

        let wasm_ptr = WasmPtr::<u8, Array>::new(array_ptr as _);
        let result = match wasm_ptr.get_utf8_string(ctx.memory(0), array_size as _) {
            Some(arg_value) => {
                // let arg_value = " add -Q /Users/mike/dev/work/fluence/wasm/tmp/ipfs_rpc_file";
                let output = run_fun!("{} {}", host_cmd, arg_value).unwrap();
                output
            }
            None => return vec![Value::I32(1)],
        };

        unsafe {
            let mem_address = match get_export_func_by_name::<i32, i32>(ctx, ALLOCATE_FUNC_NAME) {
                Ok(func) => func.call(result.len() as i32).unwrap(),
                Err(_) => return vec![Value::I32(2)],
            };

            write_to_mem(ctx, mem_address as usize, result.as_bytes());

            match get_export_func_by_name::<i32, ()>(ctx, SET_PTR_FUNC_NAME) {
                Ok(func) => func.call(mem_address as i32).unwrap(),
                Err(_) => return vec![Value::I32(3)],
            };

            match get_export_func_by_name::<i32, ()>(ctx, SET_SIZE_FUNC_NAME) {
                Ok(func) => func.call(result.len() as i32).unwrap(),
                Err(_) => return vec![Value::I32(4)],
            };

            vec![Value::I32(0)]
        }
    };

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(vec![Type::I32, Type::I32], vec![Type::I32])),
        func,
    )
}
