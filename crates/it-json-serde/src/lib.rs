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
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
mod ivalues_to_json;
mod json_to_ivalues;
mod errors;

pub type JsonResult<T> = Result<T, ITJsonSeDeError>;
pub use errors::ITJsonSeDeError;
pub use ivalues_to_json::ivalues_to_json;
pub use json_to_ivalues::json_to_ivalues;

use std::collections::HashMap;
use std::sync::Arc;

pub(crate) use wasmer_it::IValue;
pub(crate) use wasmer_it::IType;
pub(crate) use wasmer_it::IRecordType;
pub(crate) type MRecordTypes = HashMap<u64, Arc<IRecordType>>;
