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
#![warn(rust_2018_idioms)]
#![feature(get_mut_unchecked)]
#![feature(new_uninit)]
#![feature(stmt_expr_attributes)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

pub(crate) mod marine_js;
mod engine;
mod errors;
mod misc;
mod module;
mod faas;
mod global_state;

pub(crate) use engine::MModuleInterface;
pub(crate) use engine::Marine;
pub(crate) use errors::MError;
pub(crate) use module::IValue;
pub(crate) use module::IRecordType;
pub(crate) use module::IFunctionArg;
pub(crate) use module::IType;
pub(crate) use module::MRecordTypes;

pub(crate) type MResult<T> = std::result::Result<T, MError>;

// contains public API functions exported to JS
mod api;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsValue;
#[wasm_bindgen( start)]
pub fn main() -> Result<(), JsValue> {
    // prints human-readable stracktrace on panics, useful when investigating problems
    console_error_panic_hook::set_once();
    Ok(())
}
