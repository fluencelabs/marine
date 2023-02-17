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

use std::ops::Deref;
use std::sync::{Arc, Mutex};

/// Create the import intended for handling get_call_parameters SDK api.

pub(crate) fn create_call_parameters_import<WB: WasmBackend>(
    call_parameters: Arc<Mutex<marine_rs_sdk::CallParameters>>, // todo show mike // todo try to move inside caller's state
) -> HostImportDescriptor<WB> {
    let call_parameters_closure = move |_ctx: &mut <WB as WasmBackend>::Caller<'_>,
                                        _args: Vec<IValue>| {
        // TODO: BE EXTREMELY CAUTIOUS ABOUT .lock().unwrap(), INVESTIGATE IT/DISCUSS WITÏH MIKE
        let result =
            { crate::to_interface_value(call_parameters.lock().unwrap().deref()).unwrap() };
        Some(result)
    };

    HostImportDescriptor {
        host_exported_func: Box::new(call_parameters_closure),
        argument_types: vec![],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}
