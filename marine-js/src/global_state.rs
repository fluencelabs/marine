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

use marine_js_backend::JsWasmBackend;
use marine::generic::Marine;

use std::cell::RefCell;


// two variables required because public api functions borrow_mut MODULES,
// and deep internal functions borrow_mut INSTANCE
// this is a bad design, and it will be refactored while moving wasm compilation inside marine-web
thread_local!(pub(crate) static MARINE: RefCell<Option<Marine<JsWasmBackend>>> = RefCell::new(None));
