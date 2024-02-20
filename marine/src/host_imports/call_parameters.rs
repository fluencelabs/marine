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

use crate::SecurityTetraplet;

use marine_wasm_backend_traits::WasmBackend;
use marine_core::generic::HostImportDescriptor;

use wasmer_it::IValue;
use wasmer_it::IType;

use parking_lot::Mutex;

use std::ops::Deref;
use std::sync::Arc;

/// Create the import intended for handling get_call_parameters SDK api.

pub(crate) fn create_call_parameters_import_v2<WB: WasmBackend>(
    call_parameters: Arc<Mutex<marine_rs_sdk::CallParameters>>, // TODO try to avoid using mutex
) -> HostImportDescriptor<WB> {
    let call_parameters_closure = move |_ctx: &mut <WB as WasmBackend>::ImportCallContext<'_>,
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

pub(crate) fn create_call_parameters_import_v1<WB: WasmBackend>(
    call_parameters: Arc<Mutex<marine_call_parameters_v1::CallParameters>>, // TODO try to avoid using mutex
) -> HostImportDescriptor<WB> {
    let call_parameters_closure = move |_ctx: &mut <WB as WasmBackend>::ImportCallContext<'_>,
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

pub(crate) fn create_call_parameters_import_v0<WB: WasmBackend>(
    call_parameters: Arc<Mutex<marine_call_parameters_v0::CallParameters>>, // TODO try to avoid using mutex
) -> HostImportDescriptor<WB> {
    let call_parameters_closure = move |_ctx: &mut <WB as WasmBackend>::ImportCallContext<'_>,
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

pub(crate) fn call_parameters_v2_to_v0(
    call_parameters: marine_rs_sdk::CallParameters,
) -> marine_call_parameters_v0::CallParameters {
    let marine_rs_sdk::CallParameters {
        particle,
        service_id,
        service_creator_peer_id,
        host_id,
        tetraplets,
        ..
    } = call_parameters;

    marine_call_parameters_v0::CallParameters {
        init_peer_id: particle.init_peer_id,
        service_id,
        service_creator_peer_id,
        host_id,
        particle_id: particle.id,
        tetraplets: to_v0_sdk_tetraplets(tetraplets),
    }
}

pub(crate) fn call_parameters_v2_to_v1(
    call_parameters: marine_rs_sdk::CallParameters,
) -> marine_call_parameters_v1::CallParameters {
    let marine_rs_sdk::CallParameters {
        particle,
        service_id,
        service_creator_peer_id,
        host_id,
        tetraplets,
        worker_id,
    } = call_parameters;

    marine_call_parameters_v1::CallParameters {
        init_peer_id: particle.init_peer_id,
        service_id,
        service_creator_peer_id,
        host_id,
        worker_id,
        particle_id: particle.id,
        tetraplets: to_v1_sdk_tetraplets(tetraplets),
    }
}

fn to_v0_sdk_tetraplets(
    tetraplets: Vec<Vec<SecurityTetraplet>>,
) -> Vec<Vec<marine_call_parameters_v0::SecurityTetraplet>> {
    tetraplets
        .into_iter()
        .map(|tetraplets| tetraplets.into_iter().map(to_v0_sdk_tetraplet).collect())
        .collect()
}

fn to_v0_sdk_tetraplet(
    tetraplet: SecurityTetraplet,
) -> marine_call_parameters_v0::SecurityTetraplet {
    let SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        lambda,
    } = tetraplet;

    marine_call_parameters_v0::SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        json_path: lambda,
    }
}

fn to_v1_sdk_tetraplets(
    tetraplets: Vec<Vec<SecurityTetraplet>>,
) -> Vec<Vec<marine_call_parameters_v1::SecurityTetraplet>> {
    tetraplets
        .into_iter()
        .map(|tetraplets| tetraplets.into_iter().map(to_v1_sdk_tetraplet).collect())
        .collect()
}

fn to_v1_sdk_tetraplet(
    tetraplet: SecurityTetraplet,
) -> marine_call_parameters_v1::SecurityTetraplet {
    let SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        lambda,
    } = tetraplet;

    marine_call_parameters_v1::SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        json_path: lambda,
    }
}
