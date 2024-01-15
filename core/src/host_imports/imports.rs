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

use crate::IType;
use crate::IValue;
use crate::MRecordTypes;
use crate::init_wasm_func;
use crate::call_wasm_func;
use crate::generic::HostImportDescriptor;

use marine_wasm_backend_traits::prelude::*;

use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;
use it_memory_traits::Memory as ITMemory;

use futures::future::BoxFuture;
use futures::FutureExt;

use std::sync::Arc;

pub(crate) fn create_host_import_func<WB: WasmBackend>(
    store: &mut <WB as WasmBackend>::Store,
    descriptor: HostImportDescriptor<WB>,
    record_types: Arc<MRecordTypes>,
) -> <WB as WasmBackend>::HostFunction {
    let raw_args = itypes_args_to_wtypes(&descriptor.argument_types);
    let raw_output =
        itypes_output_to_wtypes(&output_type_to_types(descriptor.output_type.as_ref()));

    let descriptor = Arc::new(descriptor);
    let func = create_host_import_closure(descriptor, record_types);

    <WB as WasmBackend>::HostFunction::new_with_caller_async(
        &mut store.as_context_mut(),
        FuncSig::new(raw_args, raw_output),
        func,
    )
}

async fn call_host_import<'args, WB: WasmBackend>(
    mut caller: <WB as WasmBackend>::ImportCallContext<'args>,
    inputs: &'args [WValue],
    descriptor: Arc<HostImportDescriptor<WB>>,
    record_types: Arc<MRecordTypes>,
) -> anyhow::Result<Vec<WValue>> {
    let HostImportDescriptor {
        host_exported_func,
        argument_types,
        error_handler,
        ..
    } = descriptor.as_ref();

    let memory = caller
        .memory(STANDARD_MEMORY_INDEX)
        .unwrap_or_else(|| panic!("Host import called directly, not from wasm"));

    let inputs = lift_inputs::<WB>(
        &mut caller,
        memory.clone(),
        record_types,
        inputs,
        argument_types,
    );
    let output = match inputs {
        Ok(ivalues) => host_exported_func(&mut caller, ivalues),
        Err(e) => {
            log::error!("error occurred while lifting values in host import: {}", e);
            error_handler
                .as_ref()
                .map_or_else(|| default_error_handler(&e), |h| h(&e))
        }
    };

    Ok(lower_outputs::<WB>(caller, memory, output).await)
}

fn lift_inputs<WB: WasmBackend>(
    caller: &mut <WB as WasmBackend>::ImportCallContext<'_>,
    memory: <WB as WasmBackend>::Memory,
    record_types: Arc<MRecordTypes>,
    inputs: &[WValue],
    argument_types: &[IType],
) -> HostImportResult<Vec<IValue>> {
    let memory_view = memory.view();
    let li_helper = LiHelper::new(record_types);
    let lifter = ILifter::new(memory_view, &li_helper);
    wvalues_to_ivalues(
        &mut caller.as_context_mut(),
        &lifter,
        inputs,
        argument_types,
    )
}

async fn lower_outputs<WB: WasmBackend>(
    mut caller: <WB as WasmBackend>::ImportCallContext<'_>,
    memory: <WB as WasmBackend>::Memory,
    output: Option<IValue>,
) -> Vec<WValue> {
    init_wasm_func!(
        allocate_func,
        caller,
        (i32, i32),
        i32,
        ALLOCATE_FUNC_NAME,
        2
    );

    let is_record = matches!(&output, Some(IValue::Record(_)));

    let memory_view = memory.view();
    let mut lo_helper = LoHelper::new(allocate_func.clone(), memory);
    let lowerer =
        ILowerer::<'_, _, _, DelayedContextLifetime<WB>>::new(memory_view, &mut lo_helper)
            .map_err(HostImportError::LowererError);
    let lowering_result = match lowerer {
        Ok(mut lowerer) => {
            ivalue_to_wvalues(&mut caller.as_context_mut(), &mut lowerer, output).await
        }
        Err(e) => Err(e),
    };

    let wvalues = match lowering_result {
        Ok(wvalues) => wvalues,
        Err(e) => {
            log::error!("host closure failed: {}", e);

            // returns 0 to a Wasm module in case of errors
            //init_wasm_func!(set_result_ptr_func, caller, i32, (), SET_PTR_FUNC_NAME, 4);
            //init_wasm_func!(set_result_size_func, caller, i32, (), SET_SIZE_FUNC_NAME, 4);

            let set_result_ptr_func: TypedFunc<WB, i32, ()> =
                match caller.get_func(SET_PTR_FUNC_NAME) {
                    Ok(func) => func,
                    Err(_) => return vec![WValue::I32(4)],
                };

            let set_result_size_func: TypedFunc<WB, i32, ()> =
                match caller.get_func(SET_SIZE_FUNC_NAME) {
                    Ok(func) => func,
                    Err(_) => return vec![WValue::I32(4)],
                };

            let mut store_ctx = caller.as_context_mut();
            {
                set_result_ptr_func(&mut store_ctx, 0).await.unwrap();
            }
            set_result_size_func(&mut store_ctx, 0).await.unwrap();

            return vec![WValue::I32(0)];
        }
    };

    // TODO: refactor this when multi-value is supported
    match wvalues.len() {
        // strings and arrays are passed back to the Wasm module by pointer and size
        // values used and consumed by set_result_ptr and set_result_size
        2 => {
            init_wasm_func!(set_result_ptr_func, caller, i32, (), SET_PTR_FUNC_NAME, 4);
            init_wasm_func!(set_result_size_func, caller, i32, (), SET_SIZE_FUNC_NAME, 4);

            let mut store_ctx = caller.as_context_mut();
            call_wasm_func!(
                set_result_ptr_func,
                &mut store_ctx,
                wvalues[0].to_u128() as _
            );
            call_wasm_func!(
                set_result_size_func,
                &mut store_ctx,
                wvalues[1].to_u128() as _
            );
            vec![]
        }

        // records lowerer returns only pointer which has to be used and consumed via set_result_ptr
        1 if is_record => {
            init_wasm_func!(set_result_ptr_func, caller, i32, (), SET_PTR_FUNC_NAME, 3);

            let mut store_ctx = caller.as_context_mut();
            call_wasm_func!(
                set_result_ptr_func,
                &mut store_ctx,
                wvalues[0].to_u128() as _
            );

            vec![]
        }

        // primitive values are passed as is
        1 => vec![wvalues[0].clone()],

        // when None is passed
        0 => vec![],

        // at now while multi-values aren't supported ivalue_to_wvalues returns only Vec with
        // 0, 1, 2 values
        _ => unimplemented!(),
    }
}

fn output_type_to_types(output_type: Option<&IType>) -> Vec<IType> {
    match output_type {
        Some(ty) => vec![ty.clone()],
        None => vec![],
    }
}

fn default_error_handler(err: &HostImportError) -> Option<crate::IValue> {
    panic!(
        "an error is occurred while lifting values to interface values: {}",
        err
    )
}

fn create_host_import_closure<WB: WasmBackend>(
    descriptor: Arc<HostImportDescriptor<WB>>,
    record_types: Arc<MRecordTypes>,
) -> impl for<'args> Fn(
    <WB as WasmBackend>::ImportCallContext<'args>,
    &'args [WValue],
) -> BoxFuture<'args, anyhow::Result<Vec<WValue>>>
       + Send
       + Sync {
    move |call_context, inputs| {
        call_host_import(
            call_context,
            inputs,
            descriptor.clone(),
            record_types.clone(),
        )
        .boxed()
    }
}
