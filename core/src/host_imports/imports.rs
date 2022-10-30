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

use super::*;
use super::lifting::wvalues_to_ivalues;
use super::lifting::LiHelper;
use super::lowering::ivalue_to_wvalues;
use super::lowering::LoHelper;
use super::utils::itypes_args_to_wtypes;
use super::utils::itypes_output_to_wtypes;

use crate::MRecordTypes;
use crate::init_wasm_func_once;
use crate::call_wasm_func;
use crate::HostImportDescriptor;
//use crate::module::wit_prelude::WITMemoryView;

//use wasmer_core::Func;
//use wasmer_core::vm::Ctx;
//use wasmer_core::typed_func::DynamicFunc;
//use wasmer_core::types::Value as WValue;
//use wasmer_core::types::FuncSig;
use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;
//use it_memory_traits::Memory as ITMemory;

use std::cell::RefCell;
use std::rc::Rc;
use it_memory_traits::Memory;

use marine_wasm_backend_traits::{FuncSig, WasmBackend};
use marine_wasm_backend_traits::DynamicFunc;
use marine_wasm_backend_traits::ExportContext;
//use marine_wasm_backend_traits::FuncGetter;
use marine_wasm_backend_traits::errors::*;

pub(crate) fn create_host_import_func<WB: WasmBackend>(
    descriptor: HostImportDescriptor<WB>,
    record_types: Rc<MRecordTypes>,
) -> <WB as WasmBackend>::DynamicFunc {
    let allocate_func: AllocateFunc = Box::new(RefCell::new(None));
    let set_result_ptr_func: SetResultPtrFunc = Box::new(RefCell::new(None));
    let set_result_size_func: SetResultSizeFunc = Box::new(RefCell::new(None));

    let HostImportDescriptor {
        host_exported_func,
        argument_types,
        output_type,
        error_handler,
    } = descriptor;

    let output_type_to_types = |output_type| match output_type {
        Some(ty) => vec![ty],
        None => vec![],
    };

    let raw_args = itypes_args_to_wtypes(&argument_types);
    let raw_output = itypes_output_to_wtypes(&output_type_to_types(output_type));

    let func =
        move |ctx: &mut dyn ExportContext<WB>, inputs: &[WValue]| -> Vec<WValue> {
            let result = {
                let memory_index = 0;
                let memory_view = ctx.memory(memory_index).view();
                let li_helper = LiHelper::new(record_types.clone());
                let lifter = ILifter::new(memory_view, &li_helper);

                match wvalues_to_ivalues(&lifter, inputs, &argument_types) {
                    Ok(ivalues) => host_exported_func(ctx, ivalues),
                    Err(e) => {
                        log::error!("error occurred while lifting values in host import: {}", e);
                        error_handler
                            .as_ref()
                            .map_or_else(|| default_error_handler(&e), |h| h(&e))
                    }
                }
            };

            let ctx = ctx;
            init_wasm_func_once!(allocate_func, ctx, (i32, i32), i32, ALLOCATE_FUNC_NAME, 2);
            /*if allocate_func.borrow().is_none() {
                let raw_func = match unsafe {
                    ctx.get_export_func_by_name::<(i32, i32), i32>(ALLOCATE_FUNC_NAME)
                } {
                    Ok(func) => func,
                    Err(_) => return vec![WValue::I32(2)],
                };

                unsafe {
                    // assumed that this function will be used only in the context of closure
                    // linked to a corresponding Wasm import, so it is safe to make is static
                    // because all Wasm imports live in the Wasmer instances, which
                    // is itself static (i.e., lives until the end of the program)
                    let raw_func = std::mem::transmute::<Func<'_, _, _>, Func<'static, _, _>>(raw_func);

                    *allocate_func.borrow_mut() = Some(raw_func);
                }
            }*/

            let memory_index = 0;
            let memory_view = ctx.memory(memory_index).view();
            let lo_helper = LoHelper::new(&allocate_func, ctx.memory(memory_index));
            let t = ILowerer::new(memory_view, &lo_helper)
                .map_err(HostImportError::LowererError)
                .and_then(|lowerer| ivalue_to_wvalues(&lowerer, result));
            let wvalues = match t {
                Ok(wvalues) => wvalues,
                Err(e) => {
                    log::error!("host closure failed: {}", e);

                    // returns 0 to a Wasm module in case of errors
                    init_wasm_func_once!(set_result_ptr_func, ctx, i32, (), SET_PTR_FUNC_NAME, 4);
                    init_wasm_func_once!(set_result_size_func, ctx, i32, (), SET_SIZE_FUNC_NAME, 4);

                    call_wasm_func!(set_result_ptr_func, 0);
                    call_wasm_func!(set_result_size_func, 0);
                    return vec![WValue::I32(0)];
                }
            };

            // TODO: refactor this when multi-value is supported
            match wvalues.len() {
                // strings and arrays are passed back to the Wasm module by pointer and size
                2 => {
                    init_wasm_func_once!(set_result_ptr_func, ctx, i32, (), SET_PTR_FUNC_NAME, 4);
                    init_wasm_func_once!(set_result_size_func, ctx, i32, (), SET_SIZE_FUNC_NAME, 4);

                    call_wasm_func!(set_result_ptr_func, wvalues[0].to_u128() as _);
                    call_wasm_func!(set_result_size_func, wvalues[1].to_u128() as _);
                    vec![]
                }

                // records and primitive types are passed to the Wasm module by pointer
                // and value on the stack
                1 => {
                    init_wasm_func_once!(set_result_ptr_func, ctx, i32, (), SET_PTR_FUNC_NAME, 3);

                    call_wasm_func!(set_result_ptr_func, wvalues[0].to_u128() as _);
                    vec![wvalues[0].clone()]
                }

                // when None is passed
                0 => vec![],

                // at now while multi-values aren't supported ivalue_to_wvalues returns only Vec with
                // 0, 1, 2 values
                _ => unimplemented!(),
            }
        };

    <WB as WasmBackend>::DynamicFunc::new(FuncSig::new(raw_args, raw_output), func)
}

fn default_error_handler(err: &HostImportError) -> Option<crate::IValue> {
    panic!(
        "an error is occurred while lifting values to interface values: {}",
        err
    )
}
