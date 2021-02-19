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

mod raw_toml_config;
mod toml_config;

pub use toml_config::FaaSModuleConfig;
pub use toml_config::FaaSConfig;
pub use toml_config::FaaSWASIConfig;
pub use toml_config::ModuleDescriptor;

pub use raw_toml_config::TomlFaaSNamedModuleConfig;
pub use raw_toml_config::TomlWASIConfig;
pub use raw_toml_config::TomlFaaSConfig;
pub use raw_toml_config::TomlFaaSModuleConfig;

pub use raw_toml_config::from_toml_faas_config;
pub use raw_toml_config::from_toml_module_config;
pub use raw_toml_config::from_toml_wasi_config;
