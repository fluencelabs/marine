use marine::{Marine, TomlMarineConfig};

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
