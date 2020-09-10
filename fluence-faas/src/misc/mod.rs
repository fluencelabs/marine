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

mod config;
mod imports;
mod json_to_ivalues;
mod modules_load_strategy;
mod utils;

pub use config::RawModulesConfig;
pub use config::RawModuleConfig;
pub use config::ModulesConfig;
pub use config::ModuleConfig;
pub use config::WASIConfig;

pub(crate) use config::load_config;
pub(crate) use json_to_ivalues::json_array_to_ivalues;
pub(crate) use json_to_ivalues::json_map_to_ivalues;
pub(crate) use modules_load_strategy::ModulesLoadStrategy;
pub(crate) use utils::make_fce_config;
