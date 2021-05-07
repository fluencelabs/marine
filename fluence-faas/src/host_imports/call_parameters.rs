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

use marine::HostImportDescriptor;
use wasmer_core::vm::Ctx;
use wasmer_wit::IValue;
use wasmer_wit::IType;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

/// Create the import intended for handling get_call_parameters SDK api.
pub(crate) fn create_call_parameters_import(
    call_parameters: Rc<RefCell<fluence::CallParameters>>,
) -> HostImportDescriptor {
    let call_parameters_closure = move |_ctx: &mut Ctx, _args: Vec<IValue>| {
        let result = crate::to_interface_value(call_parameters.borrow().deref()).unwrap();
        Some(result)
    };

    HostImportDescriptor {
        host_exported_func: Box::new(call_parameters_closure),
        argument_types: vec![],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}
