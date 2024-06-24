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

use crate::MRecordTypes;
use crate::IRecordType;
use it_lilo::traits::RecordResolvable;
use it_lilo::traits::RecordResolvableError;

use std::sync::Arc;

pub(crate) struct LiHelper {
    record_types: Arc<MRecordTypes>,
}

impl LiHelper {
    pub(crate) fn new(record_types: Arc<MRecordTypes>) -> Self {
        Self { record_types }
    }
}

impl RecordResolvable for LiHelper {
    fn resolve_record(&self, record_type_id: u64) -> Result<&IRecordType, RecordResolvableError> {
        self.record_types
            .get(&record_type_id)
            .map(|r| r.as_ref())
            .ok_or(RecordResolvableError::RecordNotFound(record_type_id))
    }
}
