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

use marine_wasm_backend_traits::WasmBackend;

use marine_core::HostImportDescriptor;
use wasmer_it::IValue;
use wasmer_it::IType;

use parking_lot::Mutex;

use std::ops::Deref;
use std::sync::Arc;

/// Create the import intended for handling get_call_parameters SDK api.

pub(crate) fn create_call_parameters_import<WB: WasmBackend>(
    call_parameters: Arc<Mutex<marine_rs_sdk::CallParameters>>, // TODO try to avoid using mutex
) -> HostImportDescriptor<WB> {
    let call_parameters_closure = move |_ctx: &mut <WB as WasmBackend>::Caller<'_>,
                                        _args: Vec<IValue>| {
        let result = {
            // a separate code block to unlock the mutex ASAP and to avoid double locking
            crate::to_interface_value(call_parameters.lock().deref())
                .unwrap_or_else(|_| panic!("CallParameters should be convertible to IValue"))
        };

        Some(result)
    };

    HostImportDescriptor {
        host_exported_func: Box::new(call_parameters_closure),
        argument_types: vec![],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}
