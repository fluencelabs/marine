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

    #[error("{0}")]
    MITInterfacesError(#[from] MITInterfacesError),
}
