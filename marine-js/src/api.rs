/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::global_state::MARINE;
use crate::logger::marine_logger;

use marine::generic::Marine;
use marine::generic::MarineConfig;
use marine::generic::MarineModuleConfig;
use marine::generic::ModuleDescriptor;
use marine::MarineWASIConfig;
use marine_js_backend::JsWasmBackend;
use marine_wasm_backend_traits::WasmBackend;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Value as JValue;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ApiWasiConfig {
    pub envs: HashMap<String, String>,
    pub mapped_dirs: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiModuleConfig {
    pub logger_enabled: bool,
    pub wasi: Option<ApiWasiConfig>,
    pub logging_mask: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ApiModuleDescriptor {
    pub import_name: String,
    pub config: Option<ApiModuleConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiServiceConfig {
    pub modules_config: Vec<ApiModuleDescriptor>,
    pub default_modules_config: Option<ApiModuleConfig>,
}

impl From<ApiWasiConfig> for MarineWASIConfig {
    fn from(value: ApiWasiConfig) -> Self {
        let mapped_dirs = value
            .mapped_dirs
            .map(|mapped_dirs| {
                mapped_dirs
                    .iter()
                    .map(|(guest, host)| (guest.clone(), host.into()))
                    .collect::<HashMap<String, PathBuf>>()
            })
            .unwrap_or_default();

        Self {
            envs: value.envs,
            mapped_dirs,
        }
    }
}

impl From<ApiModuleConfig> for MarineModuleConfig<JsWasmBackend> {
    fn from(value: ApiModuleConfig) -> Self {
        Self {
            logger_enabled: value.logger_enabled,
            host_imports: Default::default(),
            wasi: value.wasi.map(Into::into),
            logging_mask: value.logging_mask,
        }
    }
}

impl From<ApiModuleDescriptor> for ModuleDescriptor<JsWasmBackend> {
    fn from(value: ApiModuleDescriptor) -> Self {
        Self {
            load_from: None,
            file_name: value.import_name.clone(),
            import_name: value.import_name,
            config: value.config.map(Into::into).unwrap_or_default(),
        }
    }
}

impl From<ApiServiceConfig> for MarineConfig<JsWasmBackend> {
    fn from(value: ApiServiceConfig) -> Self {
        let modules_config = value
            .modules_config
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ModuleDescriptor<JsWasmBackend>>>();

        MarineConfig {
            modules_dir: None,
            total_memory_limit: None,
            modules_config,
            default_modules_config: value.default_modules_config.map(Into::into),
        }
    }
}

/// Registers a module inside web-runtime.
///
/// # Arguments
///
/// * `config` - description of wasm modules with names, wasm bytes and wasi parameters
/// * `log_fn` - function to direct logs from wasm modules
///
/// # Return value
///
/// Nothing. An error is signaled via exception.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub async fn register_module(
    config: JsValue,
    modules: js_sys::Object,
    log_fn: js_sys::Function,
) -> Result<(), JsError> {
    let mut marine = MARINE
        .lock()
        .map_err(|e| JsError::new(&format!("MARINE.lock() failed in register_module: {:?}", e)))?;

    let mut config: ApiServiceConfig = serde_wasm_bindgen::from_value(config)?;
    let modules = extract_modules(modules)?;

    let marine_config: MarineConfig<JsWasmBackend> = config.into();
    let module_names = modules.keys().cloned().collect::<HashSet<String>>();

    marine_logger().enable_service_logging(log_fn, module_names);

    let backend = JsWasmBackend::new_async()?;

    let new_marine = Marine::<JsWasmBackend>::with_modules(backend, modules, marine_config).await?;

    marine.replace(new_marine);

    Ok(())
}

fn extract_modules(modules: js_sys::Object) -> Result<HashMap<String, Vec<u8>>, JsError> {
    let mut modules_map = HashMap::<String, Vec<u8>>::new();
    for key in js_sys::Object::keys(&modules) {
        if !key.is_string() {
            return Err(JsError::new("modules object has non-string key"));
        }

        let property =
            js_sys::Reflect::get(&modules, &key).map_err(|e| JsError::new(&format!("{:?}", e)))?;
        let module_bytes: js_sys::Uint8Array = property.try_into()?;
        let module_name = key
            .as_string()
            .ok_or_else(|| JsError::new("cannot convert modules object property to string"))?;
        let module_bytes = module_bytes.to_vec();
        modules_map.insert(module_name, module_bytes);
    }

    Ok(modules_map)
}

///  Calls a function from a module.
///
/// # Arguments
///
/// * module_name - name of registered module
/// * function_name - name of the function to call
/// * args - JSON array of function arguments
/// * call_parameters - an object representing call paramters, with the structure defined by fluence network
/// # Return value
///
/// JSON array of values. An error is signaled via exception.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub async fn call_module(
    module_name: &str,
    function_name: &str,
    args: &str,
    call_parameters: JsValue,
) -> Result<String, JsError> {
    let call_parameters = serde_wasm_bindgen::from_value(call_parameters)?;

    let mut marine = MARINE
        .lock()
        .map_err(|e| JsError::new(&format!("MARINE.lock() failed in call_module: {:?}", e)))?;

    let args: JValue = serde_json::from_str(args)?;

    let marine = marine
        .as_mut()
        .ok_or_else(|| JsError::new("marine is not initialized"))?;

    let result = marine
        .call_with_json_async(module_name, function_name, args, call_parameters)
        .await?;
    serde_json::ser::to_string(&result).map_err(|e| JsError::new(&e.to_string()))
}
