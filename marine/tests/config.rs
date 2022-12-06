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

use serde_json::{json, Value};
use marine::{Marine, MarineError, TomlMarineConfig};

#[test]
fn load_from_modules_dir() {
    let config_path = "tests/config_tests/ModulesDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
}

#[test]
fn load_from_specified_dir() {
    let config_path = "tests/config_tests/SpecifiedDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
}

#[test]
fn load_from_specified_path() {
    let config_path = "tests/config_tests/SpecifiedPathConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let _marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
}

#[test]
fn wasi_mapped_dirs() {
    let config_path = "tests/wasm_tests/wasi/Config.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let mut marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
    let file_data = std::fs::read("tests/wasm_tests/wasi/some_dir/some_file")
        .expect("file must exist for test to work");
    let result = marine
        .call_with_json(
            "wasi_effector",
            "read_from_mapped_dir",
            json!([]),
            <_>::default(),
        )
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

#[test]
fn wasi_mapped_dirs_conflicts_with_preopens() {
    let config_path = "tests/wasm_tests/wasi/PreopenMappedDuplicate.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let result = Marine::with_raw_config(raw_config);
    match result {
        Err(MarineError::InvalidConfig(_)) => (),
        Err(_) => panic!(
            "Expected InvalidConfig error telling about conflict with preopens and mapped dirs"
        ),
        Ok(_) => panic!("Expected error while loading this config"),
    };
}

#[test]
fn absolute_paths_in_wasi_prohibited() {
    let config_path = "tests/wasm_tests/wasi/AbsolutePaths.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let result = Marine::with_raw_config(raw_config);
    match result {
        Err(MarineError::InvalidConfig(_)) => (),
        Err(_) => panic!("Expected InvalidConfig error telling about absolute paths"),
        Ok(_) => panic!("Expected error while loading this config"),
    };
}

#[test]
fn parent_dir_in_wasi_paths_prohibited() {
    let config_path = "tests/wasm_tests/wasi/ParentDir.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let result = Marine::with_raw_config(raw_config);
    match result {
        Err(MarineError::InvalidConfig(_)) => (),
        Err(_) => panic!("Expected InvalidConfig error telling about .. in config"),
        Ok(_) => panic!("Expected error while loading this config"),
    };
}
