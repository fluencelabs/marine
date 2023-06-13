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

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::DerefMut;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Value as JValue;
use wasm_bindgen::prelude::*;

use marine::generic::Marine;
use marine::generic::MarineConfig;
use marine::generic::MarineModuleConfig;
use marine::generic::ModuleDescriptor;
use marine::MarineWASIConfig;
use marine_js_backend::JsWasmBackend;

use crate::global_state::MARINE;
use crate::logger::marine_logger;

#[derive(Serialize, Deserialize)]
pub struct WasiConfig {
    pub envs: HashMap<String, String>,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleConfig {
    pub name: String,
    pub wasm_bytes: Vec<u8>,
    pub wasi_config: Option<WasiConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub modules: Vec<ModuleConfig>,
}

impl From<&ServiceConfig> for MarineConfig<JsWasmBackend> {
    fn from(val: &ServiceConfig) -> Self {
        let create_module_config = |wasi_config: Option<&WasiConfig>| {
            let wasi_config = wasi_config.map(|config: &WasiConfig| MarineWASIConfig {
                envs: config.envs.clone(),
                preopened_files: <_>::default(),
                mapped_dirs: <_>::default(),
            });

            MarineModuleConfig {
                mem_pages_count: None,
                max_heap_size: None,
                logger_enabled: true,
                host_imports: Default::default(),
                wasi: wasi_config,
                logging_mask: 0,
            }
        };

        let module_descriptors = val
            .modules
            .iter()
            .map(|module_config| ModuleDescriptor {
                load_from: None,
                file_name: module_config.name.clone(),
                import_name: module_config.name.clone(),
                config: create_module_config(module_config.wasi_config.as_ref()),
            })
            .collect::<Vec<ModuleDescriptor<JsWasmBackend>>>();

        MarineConfig {
            modules_dir: None,
            modules_config: module_descriptors,
            default_modules_config: None,
        }
    }
}

/// Registers a module inside web-runtime.
///
/// # Arguments
///
/// * `name` - name of module to register
/// * `wasm_bytes` - wasm file bytes
///
/// # Return value
///
/// JSON object with field "error". If error is empty, module is registered.
/// otherwise, it contains error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn register_module(config: JsValue, log_fn: js_sys::Function) -> Result<(), JsError> {
    let config: ServiceConfig = serde_wasm_bindgen::from_value(config)?;
    log::debug!("register_module start");

    let marine_config: MarineConfig<JsWasmBackend> = (&config).into();

    let modules = config
        .modules
        .into_iter()
        .map(|config| (config.name, config.wasm_bytes))
        .collect::<HashMap<String, Vec<u8>>>();

    let module_names = modules
        .keys()
        .map(Clone::clone)
        .collect::<HashSet<String>>();

    marine_logger().enable_service_logging(log_fn, module_names);

    let new_marine = Marine::<JsWasmBackend>::with_modules(modules, marine_config)?;

    MARINE.with(|marine| marine.replace(Some(new_marine)));

    log::debug!("register_module success");
    Ok(())
}

///  Calls a function from a module.
///
/// # Arguments
///
/// * module_name - name of registered module
/// * function_name - name of the function to call
/// * args - JSON array of function arguments
///
/// # Return value
///
/// JSON object with fields "error" and "result". If "error" is empty string,
/// "result" contains a function return value. Otherwise, "error" contains error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn call_module(module_name: &str, function_name: &str, args: &str) -> Result<String, JsError> {
    MARINE.with(|marine| {
        let args: JValue = serde_json::from_str(args)?;
        marine
            .borrow_mut()
            .deref_mut()
            .as_mut()
            .ok_or_else(|| JsError::new("marine is not initialized"))
            .and_then(|mut marine| {
                let result =
                    marine.call_with_json(module_name, function_name, args, <_>::default())?;
                serde_json::ser::to_string(&result).map_err(|e| JsError::new(&e.to_string()))
            })
    })
}
