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

use marine_it_interfaces::MITInterfacesError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ITInterfaceError {
    #[error("type with idx = {0} isn't a function type")]
    ITTypeNotFunction(u32),

    #[error("record type with type id {0} not found")]
    NotFoundRecordTypeId(u64),

    #[error("mailformed module: a record contains more recursion level then allowed")]
    TooManyRecursionLevels,

    #[error(transparent)]
    MITInterfacesError(#[from] MITInterfacesError),
}
