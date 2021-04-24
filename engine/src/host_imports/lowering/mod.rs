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

/// Contain functions intended to put (lower) IValues to Wasm memory
/// and pass it to a Wasm module as raw WValues (Wasm types).
mod lo_helper;
mod lower_ivalues;

pub(crate) use lo_helper::LoHelper;
pub(crate) use lower_ivalues::ivalue_to_wvalues;

use super::WValue;
use super::AllocateFunc;
use super::HostImportResult;
