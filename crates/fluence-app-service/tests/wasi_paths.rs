use std::default::default;
use fluence_app_service::{AppService, AppServiceConfig, TomlAppServiceConfig};

#[test]
pub fn wasi_paths() {
    let mut config = TomlAppServiceConfig::load("some_path").expect("Config should be loaded");
    config.service_base_dir = Some("service_base_dir".to_string());

    AppService::new(config, "service_id", <_>::default()).expect("App service shoud be created");
}
