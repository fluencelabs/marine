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

mod raw_marine_config;
mod to_marine_config;
mod marine_config;
mod path_utils;

pub use marine_config::MarineModuleConfig;
pub use marine_config::MarineConfig;
pub use marine_config::MarineWASIConfig;
pub use marine_config::ModuleDescriptor;

pub use raw_marine_config::TomlMarineNamedModuleConfig;
pub use raw_marine_config::TomlWASIConfig;
pub use raw_marine_config::TomlMarineConfig;
pub use raw_marine_config::TomlMarineModuleConfig;

pub(crate) use to_marine_config::make_marine_config;
pub(crate) use path_utils::adjust_path;
