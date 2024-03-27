/*
 * Copyright 2022 Fluence Labs Limited
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

extern crate core;

use marine::Marine;
use marine::TomlMarineConfig;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use marine_wasm_backend_traits::WasmBackend;

use serde_json::json;
use serde_json::Value;

#[tokio::test]
async fn load_from_modules_dir() {
    let config_path = "tests/config_tests/ModulesDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), raw_config)
        .await
        .expect("Marine should load all modules");
}

#[tokio::test]
async fn load_from_specified_dir() {
    let config_path = "tests/config_tests/SpecifiedDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), raw_config)
        .await
        .expect("Marine should load all modules");
}

#[tokio::test]
async fn load_from_specified_path() {
    let config_path = "tests/config_tests/SpecifiedPathConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), raw_config)
        .await
        .expect("Marine should load all modules");
}

#[tokio::test]
async fn wasi_mapped_dirs() {
    let config_path = "tests/wasm_tests/wasi/Config.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let mut marine = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), raw_config)
        .await
        .expect("Marine should load all modules");
    let file_data = std::fs::read("tests/wasm_tests/wasi/some_dir/some_file")
        .expect("file must exist for test to work");
    let result = marine
        .call_with_json_async(
            "wasi_effector",
            "read_from_mapped_dir",
            json!([]),
            <_>::default(),
        )
        .await
        .expect("function should execute successfully");
    if let Value::Array(data) = result {
        let data = data
            .into_iter()
            .map(|value| {
                value
                    .as_u64()
                    .expect("test is wrong: function returned invalid data type")
                    as u8
            })
            .collect::<Vec<u8>>();

        assert_eq!(data, file_data);
    } else {
        panic!("test is wrong: function returned invalid data type");
    }
}

#[tokio::test]
async fn mapping_from_absolute_path_in_wasi_allowed() {
    let config_path = "tests/wasm_tests/wasi/MapFromAbsolutePath.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _result = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), raw_config)
        .await
        .expect("Module should be loaded successfully");
}
