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

mod it;

pub use functions::*;
pub use it::*;

use crate::Result;
use crate::ITParserError;

use marine_it_interfaces::MITInterfaces;
use std::path::Path;

pub fn module_interface<P>(module_path: P) -> Result<ServiceInterface>
where
    P: AsRef<Path>,
{
    create_mit_with(module_path, |it| get_interface(&it))
}

pub fn module_raw_interface<P>(module_path: P) -> Result<MModuleInterface>
where
    P: AsRef<Path>,
{
    create_mit_with(module_path, |it| get_raw_interface(&it))
}

fn create_mit_with<P, T>(
    module_path: P,
    transformer: impl FnOnce(MITInterfaces<'_>) -> Result<T>,
) -> Result<T>
where
    P: AsRef<Path>,
{
    let module = walrus::ModuleConfig::new()
        .parse_file(module_path)
        .map_err(ITParserError::CorruptedWasmFile)?;
    let raw_custom_section = extract_custom_section(&module)?;
    let custom_section_bytes = raw_custom_section.as_ref();
    let it = extract_it_from_bytes(custom_section_bytes)?;

    let mit = MITInterfaces::new(it);

    transformer(mit)
}
