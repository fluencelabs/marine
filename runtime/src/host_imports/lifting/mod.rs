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

/// Contain functions intended to create (lift) IValues from raw WValues (Wasm types).

mod li_helper;
mod lift_ivalues;

pub(crate) use li_helper::LiHelper;
pub(crate) use lift_ivalues::wvalues_to_ivalues;

use super::WValue;
use super::WType;
use super::HostImportError;
use super::HostImportResult;
