use marine_rs_sdk_test::generate_marine_test_env;
use marine_rs_sdk_test::ServiceDescription;
fn main() {
    let services = vec![
        ("greeting".to_string(), ServiceDescription {
            config_path: "Config.toml".to_string(),
            modules_dir: Some("artifacts".to_string()),
        })
    ];

    if cfg!(not(target_arch = "wasm32")) {
        generate_marine_test_env(services, "marine_test_env.rs", file!());
    }

    println!("cargo:rerun-if-changed=src/main.rs");
}
