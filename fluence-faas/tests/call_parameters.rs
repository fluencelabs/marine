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

use fluence_faas::FluenceFaaS;
use fluence_faas::IValue;

use pretty_assertions::assert_eq;

#[test]
pub fn call_parameters() {
    let call_parameters_config_path = "../examples/call_parameters/Config.toml";

    let call_parameters_config_raw = std::fs::read(call_parameters_config_path)
        .expect("../examples/call_parameters/Config.toml should presence");

    let mut call_parameters_config: fluence_faas::TomlFaaSConfig =
        toml::from_slice(&call_parameters_config_raw)
            .expect("call_parameters config should be well-formed");
    call_parameters_config.modules_dir =
        Some(String::from("../examples/call_parameters/artifacts"));

    let mut faas = FluenceFaaS::with_raw_config(call_parameters_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let call_id = "0x1337";
    let user_name = "root";
    let application_id = "0x31337";

    let result = faas
        .call_with_ivalues(
            "call_parameters",
            "call_parameters",
            &[],
            fluence_sdk_main::CallParameters {
                call_id: call_id.to_string(),
                user_name: user_name.to_string(),
                application_id: application_id.to_string(),
                tetraplets: vec![],
            },
        )
        .unwrap_or_else(|e| panic!("can't invoke call_parameters: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::String(format!(
            "{}\n{}\n{}",
            call_id, user_name, application_id
        ))]
    );
}
