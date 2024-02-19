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

use marine::Marine;

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

#[test]
pub fn call_parameters_v0() {
    let mut faas = Marine::with_raw_config(CONFIG_V0.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        json_path: "some_json_path".to_string(),
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
        .call_with_json(
            "call_parameters_v0",
            "call_parameters",
            json!([]),
            call_parameters,
        )
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = json!({
        "init_peer_id": particle.init_peer_id,
        "service_id": service_id,
        "service_creator_peer_id": service_creator_peer_id,
        "host_id": host_id,
        "particle_id": particle.id,
        "tetraplets": tetraplets,
    });

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}

#[test]
pub fn call_parameters_v1() {
    let mut faas = Marine::with_raw_config(CONFIG_V1.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        json_path: "some_json_path".to_string(),
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
        .call_with_json(
            "call_parameters_v1",
            "call_parameters",
            json!([]),
            call_parameters,
        )
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = json!({
        "init_peer_id": particle.init_peer_id,
        "service_id": service_id,
        "service_creator_peer_id": service_creator_peer_id,
        "host_id": host_id,
        "worker_id": worker_id,
        "particle_id": particle.id,
        "tetraplets": tetraplets,
    });

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}

#[test]
pub fn call_parameters_v2() {
    let mut faas = Marine::with_raw_config(CONFIG_V2.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let worker_id = "worker_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        json_path: "some_json_path".to_string(),
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
        .call_with_json(
            "call_parameters_v2",
            "call_parameters",
            json!([]),
            call_parameters.clone(),
        )
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    let expected = serde_json::to_value(call_parameters).unwrap();

    let result_json: serde_json::Value = serde_json::from_str(result.as_str().unwrap()).unwrap();
    assert_eq!(expected, result_json,);
}
