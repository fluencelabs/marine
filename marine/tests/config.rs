use serde_json::{json, Value};
use marine::{IValue, Marine, TomlMarineConfig};

#[test]
fn load_from_modules_dir() {
    let config_path = "tests/config_tests/ModulesDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
}

#[test]
fn load_from_specified_dir() {
    let config_path = "tests/config_tests/SpecifiedDirConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
}

#[test]
fn load_from_specified_path() {
    let config_path = "tests/config_tests/SpecifiedPathConfig.toml";
    let raw_config = TomlMarineConfig::load(config_path).expect("Config must be loaded");
    let marine = Marine::with_raw_config(raw_config).expect("Marine should load all modules");
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
