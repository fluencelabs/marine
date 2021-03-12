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

use walrus::CustomSection;
use walrus::IdsToIndices;

use std::borrow::Cow;

pub const IT_SECTION_NAME: &str = "interface-types";

#[derive(Debug, Clone)]
pub(super) struct ITCustomSection(pub Vec<u8>);

impl CustomSection for ITCustomSection {
    fn name(&self) -> &str {
        IT_SECTION_NAME
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }
}
