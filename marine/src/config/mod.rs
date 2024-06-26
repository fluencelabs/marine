/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod raw_marine_config;
mod to_marine_config;
mod marine_config;
mod path_utils;

pub use marine_config::ConfigContext;
pub use marine_config::WithContext;
pub use marine_config::MarineModuleConfig;
pub use marine_config::MarineConfig;
pub use marine_config::MarineWASIConfig;
pub use marine_config::ModuleDescriptor;

pub use raw_marine_config::TomlMarineNamedModuleConfig;
pub use raw_marine_config::TomlWASIConfig;
pub use raw_marine_config::TomlMarineConfig;
pub use raw_marine_config::TomlMarineModuleConfig;

// reexport toml types, so users don't have to directly depend on the same version of toml crate
pub use toml::Value as TomlValue;
pub use toml::value::Table as TomlValueTable;

pub(crate) use to_marine_config::make_marine_config;
pub(crate) use path_utils::as_relative_to_base;
