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

use super::WType;
use crate::IType;

use wasmer_core::backend::SigRegistry;
use wasmer_core::module::ExportIndex;
use wasmer_core::vm::Ctx;
use wasmer_core::typed_func::WasmTypeList;
use wasmer_runtime::Func;
use wasmer_runtime::error::ResolveError;
use wasmer_runtime::types::LocalOrImport;

// based on Wasmer: https://github.com/wasmerio/wasmer/blob/081f6250e69b98b9f95a8f62ad6d8386534f3279/lib/runtime-core/src/instance.rs#L863
/// Extract export function from Wasmer instance by name.
pub(super) unsafe fn get_export_func_by_name<'a, Args, Rets>(
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

    let typed_func: Func<'_, Args, Rets, wasmer_core::typed_func::Wasm> =
        Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, ctx as _);

    Ok(typed_func)
}

pub(super) fn write_to_wasm_mem(context: &mut Ctx, address: usize, value: &[u8]) {
    let memory = context.memory(0);

    memory.view::<u8>()[address..(address + value.len())]
        .iter()
        .zip(value.iter())
        .for_each(|(cell, byte)| cell.set(*byte));
}

pub(super) fn itypes_args_to_wtypes(itypes: &[IType]) -> Vec<WType> {
    itypes
        .iter()
        .map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) => vec![WType::I32, WType::I32],
            _ => vec![WType::I32],
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub(super) fn itypes_output_to_wtypes(itypes: &[IType]) -> Vec<WType> {
    itypes
        .iter()
        .map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) | IType::Record(_) => vec![],
            _ => vec![WType::I32],
        })
        .flatten()
        .collect::<Vec<_>>()
}

#[macro_export] // https://github.com/rust-lang/rust/issues/57966#issuecomment-461077932
/// Initialize Wasm function in form of Box<RefCell<Option<Func<'static, args, rets>>>> only once.
macro_rules! init_wasm_func_once {
    ($func:ident, $ctx:ident, $args:ty, $rets:ty, $func_name:ident, $ret_error_code: expr) => {
        if $func.borrow().is_none() {
            let raw_func = match unsafe {
                super::utils::get_export_func_by_name::<$args, $rets>($ctx, $func_name)
            } {
                Ok(func) => func,
                Err(_) => return vec![WValue::I32($ret_error_code)],
            };

            unsafe {
                // assumed that this function will be used only in the context of closure
                // linked to a corresponding Wasm import, so it is safe to make is static
                let raw_func = std::mem::transmute::<Func<'_, _, _>, Func<'static, _, _>>(raw_func);

                *$func.borrow_mut() = Some(raw_func);
            }
        }
    };
}

#[macro_export]
/// Call Wasm function that have Box<RefCell<Option<Func<'static, args, rets>>>> type.
macro_rules! call_wasm_func {
    ($func:ident, $arg:expr) => {
        $func.borrow().as_ref().unwrap().call($arg).unwrap()
    };
}
