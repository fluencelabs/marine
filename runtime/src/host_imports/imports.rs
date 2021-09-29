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

use wasmer_core::Func;
use wasmer_core::vm::Ctx;
use wasmer_core::typed_func::DynamicFunc;
use wasmer_core::types::Value as WValue;
use wasmer_core::types::FuncSig;
use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;

use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

pub(crate) fn create_host_import_func(
    descriptor: HostImportDescriptor,
    record_types: Rc<MRecordTypes>,
) -> DynamicFunc<'static> {
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

    let func = move |ctx: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
        let memory_index = 0;
        let view = ctx.memory(memory_index).view::<u8>();
        let memory = view.deref();

        let li_helper = LiHelper::new(record_types.clone());
        let lifter = ILifter::new(memory, &li_helper);

        let result = match wvalues_to_ivalues(&lifter, inputs, &argument_types) {
            Ok(ivalues) => host_exported_func(ctx, ivalues),
            Err(e) => {
                log::error!("error occurred while lifting values in host import: {}", e);
                error_handler
                    .as_ref()
                    .map_or_else(|| default_error_handler(&e), |h| h(&e))
            }
        };

        init_wasm_func_once!(allocate_func, ctx, (i32, i32), i32, ALLOCATE_FUNC_NAME, 2);

        let lo_helper = LoHelper::new(ctx, &allocate_func);
        let t = ILowerer::new(&lo_helper)
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

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(raw_args, raw_output)),
        func,
    )
}

fn default_error_handler(err: &HostImportError) -> Option<crate::IValue> {
    panic!(
        "an error is occurred while lifting values to interface values: {}",
        err
    )
}
