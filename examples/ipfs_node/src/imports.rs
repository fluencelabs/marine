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

const ALLOCATE_FUNC_NAME: &'static str = "allocate";

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => println!("ipfs node logger: incorrect UTF8 string's been supplied to logger"),
    }
}

// rewrited from Wasmer: https://github.com/wasmerio/wasmer/blob/081f6250e69b98b9f95a8f62ad6d8386534f3279/lib/runtime-core/src/instance.rs#L863
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

pub(super) fn create_host_import_func(host_cmd: String) -> DynamicFunc<'static> {
    let allocate_func: Option<Func<'static, i32, i32>> = None;
    let set_result_ptr: Option<Func<'static, i32, ()>> = None;
    let set_result_size: Option<Func<'static, i32, ()>> = None;

    let func = move |ctx: &mut Ctx, inputs: &[Value]| -> Vec<Value> {
        use wasmer_core::memory::ptr::{Array, WasmPtr};

        println!("inputs size is {}", inputs.len());

        // TODO: refactor this
        let array_ptr = inputs[0].to_u128() as i32;
        let array_size = inputs[1].to_u128() as i32;
        println!("ptr is {}, size is {}", array_ptr, array_size);

        let wasm_ptr = WasmPtr::<u8, Array>::new(array_ptr as _);
        let result = match wasm_ptr.get_utf8_string(ctx.memory(0), array_size as _) {
            Some(arg_value) => {
                let output = std::process::Command::new(host_cmd.clone())
                    .arg(arg_value)
                    .output()
                    .unwrap();
                output.stdout
            }
            None => b"host callback: incorrect UTF8 string's been supplied to import".to_vec(),
        };

        println!("from host import function: result is {:?}", result);

        unsafe {
            if let mut allocate_func = None {
                let func = match get_export_func_by_name::<i32, i32>(ctx, ALLOCATE_FUNC_NAME) {
                    Ok(func) => func,
                    Err(_) => return vec![Value::I32(0)],
                };
                allocate_func = Some(func);
            }

            if let mut set_result_ptr = None {
                let func = match get_export_func_by_name::<i32, ()>(ctx, ALLOCATE_FUNC_NAME) {
                    Ok(func) => func,
                    Err(_) => return vec![Value::I32(0)],
                };
                set_result_ptr = Some(func);
            }

            if let mut set_result_size = None {
                let func = match get_export_func_by_name::<i32, ()>(ctx, ALLOCATE_FUNC_NAME) {
                    Ok(func) => func,
                    Err(_) => return vec![Value::I32(0)],
                };
                set_result_size = Some(func);
            }
            let mem_address = allocate_func
                .clone()
                .unwrap()
                .call(result.len() as i32)
                .unwrap();
            let _ = set_result_ptr.clone().unwrap().call(mem_address as i32);
            let _ = set_result_size.clone().unwrap().call(result.len() as i32);

            vec![Value::I32(1)]
        }
    };

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(vec![Type::I32, Type::I32], vec![Type::I32])),
        func,
    )
}
