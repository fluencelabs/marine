use crate::args::CallSchema;

use marine_rs_sdk::CallParameters;

use anyhow::anyhow;

use std::path::PathBuf;
use fluence_app_service::AppService;

pub(crate) fn run_single(
    config_path: PathBuf,
    func_name: &str,
    args: serde_json::Value,
    call_parameters: Option<CallParameters>,
) -> Result<(), anyhow::Error> {
    let mut app_service = load_app_service(config_path)?;
    let _result = call_app_service(&mut app_service, func_name, args, call_parameters)?;
    Ok(())
}

pub(crate) fn run_call_schema(
    config_path: PathBuf,
    call_schema: CallSchema,
) -> Result<(), anyhow::Error> {
    let mut app_service = load_app_service(config_path)?;
    for call_descritor in call_schema.calls {
        for _ in 0..call_descritor.repeat {
            let _result = call_app_service(
                &mut app_service,
                &call_descritor.func_name,
                call_descritor.args.clone(),
                call_descritor.call_parameters.clone(),
            )?;
        }
    }

    Ok(())
}

fn load_app_service(config_path: PathBuf) -> Result<AppService, anyhow::Error> {
    let mut config =
        fluence_app_service::TomlAppServiceConfig::load(&config_path).map_err(|e| {
            anyhow!(
                "Cannot load marine config from {}, error: {:?}",
                config_path.display(),
                e
            )
        })?;

    config.service_base_dir = Some(config_path.parent().unwrap().display().to_string());
    config.toml_marine_config.base_path = config_path.parent().unwrap().to_path_buf();

    AppService::new(config, "traced_service", <_>::default())
        .map_err(|e| anyhow!("Cannot create AppService: {:?}", e))
}

fn call_app_service(
    app_service: &mut AppService,
    func_name: &str,
    args: serde_json::Value,
    call_parameters: Option<CallParameters>,
) -> Result<serde_json::Value, anyhow::Error> {
    app_service
        .call(func_name, args.clone(), call_parameters.unwrap_or_default())
        .map_err(|e| {
            anyhow::anyhow!(
                "AppService call failed, func_name: {}, arguments: {}, error: {:?}",
                func_name,
                args,
                e
            )
        })
}
