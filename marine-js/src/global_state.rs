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

use marine_js_backend::JsWasmBackend;
use marine::generic::Marine;

use once_cell::sync::Lazy;

use std::cell::RefCell;
use std::sync::Mutex;

thread_local!(pub(crate) static MARINE_OLD: RefCell<Option<Marine<JsWasmBackend>>> = RefCell::new(None));
pub(crate) static MARINE: Lazy<Mutex<Option<Marine<JsWasmBackend>>>> =
    Lazy::new(|| Mutex::new(None));
