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

mod errors;
mod manifest_extractor;
mod module_manifest;
#[cfg(test)]
mod tests;

pub use errors::ManifestError;
pub use manifest_extractor::extract_from_path;
pub use manifest_extractor::extract_from_module;
pub use manifest_extractor::extract_from_compiled_module;
pub use manifest_extractor::extract_from_bytes;
pub use module_manifest::ModuleManifest;
