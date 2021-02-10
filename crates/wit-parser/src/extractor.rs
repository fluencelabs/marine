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

mod functions;
mod wit;

pub use functions::*;
pub use wit::*;

use crate::Result;
use std::path::{Path};

pub fn module_interface(module_path: &Path) -> Result<ServiceInterface> {
    use fce_wit_interfaces::FCEWITInterfaces;

    let wit_section_bytes = extract_wit_section_bytes(module_path)?;
    let wit = extract_wit_with_fn(&wit_section_bytes)?;
    let fce_interface = FCEWITInterfaces::new(wit);

    get_interface(&fce_interface)
}
