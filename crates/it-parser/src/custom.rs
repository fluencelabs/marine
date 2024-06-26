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
