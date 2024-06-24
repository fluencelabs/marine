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

/// Contain functions intended to create (lift) IValues from raw WValues (Wasm types).

mod li_helper;
mod lift_ivalues;

pub(crate) use li_helper::LiHelper;
pub(crate) use lift_ivalues::wvalues_to_ivalues;

use super::WValue;
use super::WType;
use super::HostImportError;
use super::HostImportResult;
