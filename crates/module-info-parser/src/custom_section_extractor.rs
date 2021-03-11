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

use crate::Result;

use walrus::IdsToIndices;
use walrus::Module;

use std::borrow::Cow;

pub(crate) fn extract_custom_sections_by_name<'w>(
    wasm_module: &'w Module,
    section_name: &str,
) -> Result<Vec<Cow<'w, [u8]>>> {
    let default_ids = IdsToIndices::default();

    let sections = wasm_module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == section_name)
        .map(|s| s.1.data(&default_ids))
        .collect::<Vec<_>>();

    Ok(sections)
}
