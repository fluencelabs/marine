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

/// Contain functions intended to put (lower) IValues to Wasm memory
/// and pass it to a Wasm module as raw WValues (Wasm types).
mod lo_helper;
mod lower_ivalues;

pub(crate) use lo_helper::LoHelper;
pub(crate) use lower_ivalues::ivalue_to_wvalues;

use super::WValue;
use super::AllocateFunc;
use super::HostImportResult;
