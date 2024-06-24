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

use marine::generic::MarineConfig;

use std::path::PathBuf;
use marine_wasm_backend_traits::WasmBackend;

/// Describes behaviour of the Fluence AppService.
#[derive(Default)]
pub struct AppServiceConfig<WB: WasmBackend> {
    /// Used for preparing filesystem on the service initialization stage.
    pub service_working_dir: PathBuf,
    pub marine_config: MarineConfig<WB>,
}
