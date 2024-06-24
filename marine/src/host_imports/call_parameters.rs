/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::SecurityTetraplet;

use marine_wasm_backend_traits::WasmBackend;
use marine_core::generic::HostImportDescriptor;

use wasmer_it::IValue;
use wasmer_it::IType;

use parking_lot::Mutex;
use serde::Serialize;

use std::ops::Deref;
use std::sync::Arc;

/// Create the import intended for handling get_call_parameters SDK api.

pub(crate) fn create_call_parameters_import<WB: WasmBackend, CP: Serialize + Send + 'static>(
    call_parameters: Arc<Mutex<CP>>, // TODO try to avoid using mutex
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

pub(crate) fn call_parameters_v3_to_v0(
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

pub(crate) fn call_parameters_v3_to_v1(
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

pub(crate) fn call_parameters_v3_to_v2(
    call_parameters: marine_rs_sdk::CallParameters,
) -> marine_call_parameters_v2::CallParameters {
    let marine_rs_sdk::CallParameters {
        particle,
        service_id,
        service_creator_peer_id,
        host_id,
        tetraplets,
        worker_id,
    } = call_parameters;

    marine_call_parameters_v2::CallParameters {
        particle: marine_call_parameters_v2::ParticleParameters {
            id: particle.id,
            init_peer_id: particle.init_peer_id,
            signature: particle.signature,
            timestamp: particle.timestamp,
            ttl: particle.ttl,
            script: particle.script,
            token: particle.token,
        },
        service_id,
        service_creator_peer_id,
        host_id,
        worker_id,
        tetraplets: to_v2_sdk_tetraplets(tetraplets),
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
        lens,
    } = tetraplet;

    marine_call_parameters_v0::SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        json_path: lens,
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
        lens,
    } = tetraplet;

    marine_call_parameters_v1::SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        json_path: lens,
    }
}

fn to_v2_sdk_tetraplets(
    tetraplets: Vec<Vec<SecurityTetraplet>>,
) -> Vec<Vec<marine_call_parameters_v2::SecurityTetraplet>> {
    tetraplets
        .into_iter()
        .map(|tetraplets| tetraplets.into_iter().map(to_v2_sdk_tetraplet).collect())
        .collect()
}

fn to_v2_sdk_tetraplet(
    tetraplet: SecurityTetraplet,
) -> marine_call_parameters_v2::SecurityTetraplet {
    let SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        lens,
    } = tetraplet;

    marine_call_parameters_v2::SecurityTetraplet {
        peer_pk,
        service_id,
        function_name,
        lambda: lens,
    }
}
