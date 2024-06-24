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

use super::WType;
use super::WValue;

use it_lilo::lifter::LiError;
use it_lilo::lowerer::LoError;
use it_lilo::traits::RecordResolvableError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum HostImportError {
    /// An error occurred when host functions tries to lift IValues from WValues
    /// and the latter has different type.
    #[error(
        "Expected {0} type, but found {1:?} value during interface values lifting from Wasm memory"
    )]
    MismatchWValues(WType, WValue),

    /// An error occurred when a host functions tries to lift IValues from WValues
    /// and the latter is not enough for that.
    #[error("Not enough WValue arguments are provided from the Wasm side")]
    MismatchWValuesCount,

    #[error(transparent)]
    LifterError(#[from] LiError),

    #[error(transparent)]
    LowererError(#[from] LoError),

    #[error(transparent)]
    RecordNotFound(#[from] RecordResolvableError),

    #[error(transparent)]
    InvalidUTF8String(#[from] std::string::FromUtf8Error),
}
