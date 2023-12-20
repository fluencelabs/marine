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
use marine::IValue;
use marine_wasmtime_backend::WasmtimeWasmBackend;

use pretty_assertions::assert_eq;

use std::path::PathBuf;

#[tokio::test]
pub async fn call_parameters() {
    let call_parameters_config_path = "../examples/call_parameters/Config.toml";

    let call_parameters_config_raw = std::fs::read(call_parameters_config_path)
        .expect("../examples/call_parameters/Config.toml should presence");

    let mut call_parameters_config: marine::TomlMarineConfig =
        toml::from_slice(&call_parameters_config_raw)
            .expect("call_parameters config should be well-formed");
    call_parameters_config.modules_dir =
        Some(PathBuf::from("../examples/call_parameters/artifacts"));

    let mut faas = Marine::with_raw_config(WasmtimeWasmBackend::default(), call_parameters_config)
        .await
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let init_peer_id = "init_peer_id";
    let service_id = "service_id";
    let service_creator_peer_id = "service_creator_peer_id";
    let host_id = "host_id";
    let particle_id = "particle_id";

    let tetraplet = marine_rs_sdk::SecurityTetraplet {
        function_name: "some_func_name".to_string(),
        json_path: "some_json_path".to_string(),
        ..Default::default()
    };

    let tetraplets = vec![vec![tetraplet]];

    let call_parameters = marine_rs_sdk::CallParameters {
        init_peer_id: init_peer_id.to_string(),
        service_id: service_id.to_string(),
        service_creator_peer_id: service_creator_peer_id.to_string(),
        host_id: host_id.to_string(),
        particle_id: particle_id.to_string(),
        tetraplets: tetraplets.clone(),
    };

    let result = faas
        .call_with_ivalues("call_parameters", "call_parameters", &[], call_parameters)
        .await
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::String(format!(
            "{}\n{}\n{}\n{}\n{}\n{:?}",
            init_peer_id, service_id, service_creator_peer_id, host_id, particle_id, tetraplets
        ))]
    );
}
