/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine::Marine;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use marine_wasm_backend_traits::WasmBackend;

use pretty_assertions::assert_eq;
use once_cell::sync::Lazy;

use serde_json::json;

static CONFIG_V0: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/call_parameters_v0/Config.toml")
        .expect("toml faas config should be created")
});

static CONFIG_V1: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/call_parameters_v1/Config.toml")
        .expect("toml faas config should be created")
});

static CONFIG_V2: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/call_parameters_v2/Config.toml")
        .expect("toml faas config should be created")
});

static CONFIG_V3: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/call_parameters_v3/Config.toml")
        .expect("toml faas config should be created")
});

#[tokio::test]
pub async fn call_parameters_v0() {
    let mut faas =
        Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), CONFIG_V0.clone())
            .await
            .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        lens: "some_lens".to_string(),
        ..Default::default()
    };

    let tetraplets = vec![vec![tetraplet]];

    let particle = marine_rs_sdk::ParticleParameters {
        id: particle_id.to_string(),
        init_peer_id: init_peer_id.to_string(),
        timestamp: 0,
        ttl: 0,
        script: "(null)".to_string(),
        signature: vec![],
        token: "token".to_string(),
    };
    let call_parameters = marine_rs_sdk::CallParameters {
        particle: particle.clone(),
        service_id: service_id.to_string(),
        service_creator_peer_id: service_creator_peer_id.to_string(),
        host_id: host_id.to_string(),
        worker_id: worker_id.to_string(),
        tetraplets: tetraplets.clone(),
    };

    let result = faas
        .call_with_json_async(
            "call_parameters_v0",
            "call_parameters",
            json!([]),
            call_parameters,
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = json!({
        "init_peer_id": particle.init_peer_id,
        "service_id": service_id,
        "service_creator_peer_id": service_creator_peer_id,
        "host_id": host_id,
        "particle_id": particle.id,
        "tetraplets": [[{
            "peer_pk": "",
            "service_id": "",
            "function_name": "some_func_name",
            "json_path": "some_lens",
        }]],
    });

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}

#[tokio::test]
pub async fn call_parameters_v1() {
    let mut faas =
        Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), CONFIG_V1.clone())
            .await
            .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        lens: "some_lens".to_string(),
        ..Default::default()
    };

    let tetraplets = vec![vec![tetraplet]];

    let particle = marine_rs_sdk::ParticleParameters {
        id: particle_id.to_string(),
        init_peer_id: init_peer_id.to_string(),
        timestamp: 0,
        ttl: 0,
        script: "(null)".to_string(),
        signature: vec![],
        token: "token".to_string(),
    };
    let call_parameters = marine_rs_sdk::CallParameters {
        particle: particle.clone(),
        service_id: service_id.to_string(),
        service_creator_peer_id: service_creator_peer_id.to_string(),
        host_id: host_id.to_string(),
        worker_id: worker_id.to_string(),
        tetraplets: tetraplets.clone(),
    };

    let result = faas
        .call_with_json_async(
            "call_parameters_v1",
            "call_parameters",
            json!([]),
            call_parameters,
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = json!({
        "init_peer_id": particle.init_peer_id,
        "service_id": service_id,
        "service_creator_peer_id": service_creator_peer_id,
        "host_id": host_id,
        "worker_id": worker_id,
        "particle_id": particle.id,
        "tetraplets": [[{
            "peer_pk": "",
            "service_id": "",
            "function_name": "some_func_name",
            "json_path": "some_lens",
        }]],
    });

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}

#[tokio::test]
pub async fn call_parameters_v2() {
    let mut faas =
        Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), CONFIG_V2.clone())
            .await
            .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        lens: "some_lens".to_string(),
        ..Default::default()
    };

    let tetraplets = vec![vec![tetraplet]];

    let particle = marine_rs_sdk::ParticleParameters {
        id: particle_id.to_string(),
        init_peer_id: init_peer_id.to_string(),
        timestamp: 0,
        ttl: 0,
        script: "(null)".to_string(),
        signature: vec![],
        token: "token".to_string(),
    };
    let call_parameters = marine_rs_sdk::CallParameters {
        particle: particle.clone(),
        service_id: service_id.to_string(),
        service_creator_peer_id: service_creator_peer_id.to_string(),
        host_id: host_id.to_string(),
        worker_id: worker_id.to_string(),
        tetraplets: tetraplets.clone(),
    };

    let result = faas
        .call_with_json_async(
            "call_parameters_v2",
            "call_parameters",
            json!([]),
            call_parameters,
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = json!({
        "particle": particle,
        "service_id": service_id,
        "service_creator_peer_id": service_creator_peer_id,
        "host_id": host_id,
        "worker_id": worker_id,
        "tetraplets": [[{
            "peer_pk": "",
            "service_id": "",
            "function_name": "some_func_name",
            "lambda": "some_lens",
        }]],
    });

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}

#[tokio::test]
pub async fn call_parameters_v3() {
    let mut faas =
        Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), CONFIG_V3.clone())
            .await
            .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        lens: "some_lens".to_string(),
        ..Default::default()
    };

    let tetraplets = vec![vec![tetraplet]];

    let particle = marine_rs_sdk::ParticleParameters {
        id: particle_id.to_string(),
        init_peer_id: init_peer_id.to_string(),
        timestamp: 0,
        ttl: 0,
        script: "(null)".to_string(),
        signature: vec![],
        token: "token".to_string(),
    };
    let call_parameters = marine_rs_sdk::CallParameters {
        particle: particle.clone(),
        service_id: service_id.to_string(),
        service_creator_peer_id: service_creator_peer_id.to_string(),
        host_id: host_id.to_string(),
        worker_id: worker_id.to_string(),
        tetraplets: tetraplets.clone(),
    };

    let result = faas
        .call_with_json_async(
            "call_parameters_v3",
            "call_parameters",
            json!([]),
            call_parameters.clone(),
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = serde_json::to_value(call_parameters).unwrap();

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}
