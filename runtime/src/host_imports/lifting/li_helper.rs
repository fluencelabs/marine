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

use crate::MRecordTypes;
use crate::IRecordType;
use it_lilo::traits::RecordResolvable;
use it_lilo::traits::RecordResolvableError;

use std::rc::Rc;

pub(crate) struct LiHelper {
    record_types: Rc<MRecordTypes>,
}

impl LiHelper {
    pub(crate) fn new(record_types: Rc<MRecordTypes>) -> Self {
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
