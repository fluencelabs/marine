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

mod utils;

use marine::Marine;
use marine::IType;

use pretty_assertions::assert_eq;
use once_cell::sync::Lazy;
use serde_json::json;

use std::sync::Arc;

static ARG_CONFIG: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/memory_limiting/Config.toml")
        .expect("toml faas config should be created")
});

const MODULE_NAME: &str = "arguments_passing_pure";

#[test]
pub fn triggered_on_instantiation() {
    let faas = Marine::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

}
#[test]
pub fn triggered_by_single_module() {
    let faas = Marine::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

}
#[test]
pub fn not_triggered_near_limit() {
    let faas = Marine::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

}

#[test]
pub fn triggered_by_two_modules() {
    let faas = Marine::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

}
