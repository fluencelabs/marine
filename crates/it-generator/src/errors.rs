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

use thiserror::Error as ThisError;
use serde_json::Error as SerdeDeserializationError;

#[derive(Debug, ThisError)]
pub enum ITGeneratorError {
    /// An error related to serde deserialization.
    #[error("Embedded by rust-sdk metadata couldn't be parsed by serde: {0:?}")]
    DeserializationError(#[from] SerdeDeserializationError),

    /// Various errors related to records
    #[error("{0}")]
    CorruptedRecord(String),

    /// Various errors occurred during the parsing/emitting a Wasm file.
    #[error("I/O error occurred: {0}")]
    IOError(String),
}
