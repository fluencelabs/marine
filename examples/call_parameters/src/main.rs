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

#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::marine;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::module_manifest;

#[cfg(target_arch = "wasm32")]
module_manifest!();

pub fn main() {}

#[marine]
#[cfg(target_arch = "wasm32")]
pub fn call_parameters() -> String {
    let cp = marine_rs_sdk::get_call_parameters();
    format!(
        "{}\n{}\n{}\n{}\n{}\n{:?}",
        cp.init_peer_id,
        cp.service_id,
        cp.service_creator_peer_id,
        cp.host_id,
        cp.particle_id,
        cp.tetraplets
    )
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    use marine_rs_sdk_test::CallParameters;
    

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn empty_string(call_parameters: marine_test_env::call_parameters::ModuleInterface) {
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

        let cp = CallParameters {
            init_peer_id: init_peer_id.to_string(),
            service_id: service_id.to_string(),
            service_creator_peer_id: service_creator_peer_id.to_string(),
            host_id: host_id.to_string(),
            particle_id: particle_id.to_string(),
            tetraplets: tetraplets.clone(),
        };

        let actual = call_parameters.call_parameters_cp(cp);
        let expected = format!(
            "{}\n{}\n{}\n{}\n{}\n{:?}",
            init_peer_id, service_id, service_creator_peer_id, host_id, particle_id, tetraplets
        );
        assert_eq!(actual, expected);
    }
}
