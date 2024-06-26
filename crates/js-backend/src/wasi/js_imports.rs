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

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/wasi_bindings.js")]
extern "C" {
    pub fn create_wasi(env: JsValue) -> JsValue;
    pub fn generate_wasi_imports(module: &JsValue, wasi: &JsValue) -> JsValue;
    pub fn bind_to_instance(wasi: &JsValue, memory: &JsValue);
}
