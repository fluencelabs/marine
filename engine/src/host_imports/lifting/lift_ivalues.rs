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

use super::WType;
use super::WValue;
use super::HostImportError;
use super::HostImportResult;
use crate::IValue;
use crate::RecordTypes;
use crate::IType;

use it_lilo_utils::memory_reader::MemoryReader;

use std::rc::Rc;

macro_rules! next_wvalue {
    ($wvalue_iter:ident, $wtype:ident) => {
        match $wvalue_iter
            .next()
            .ok_or_else(|| HostImportError::MismatchWValuesCount)?
        {
            WValue::$wtype(v) => *v,
            v => return Err(HostImportError::MismatchWValues(WType::$wtype, v.clone())),
        };
    };
}

macro_rules! simple_wvalue_to_ivalue {
    ($result:ident, $wvalue_iter:ident, $wtype:ident, $ivalue:ident) => {{
        let w = next_wvalue!($wvalue_iter, $wtype);
        $result.push(IValue::$ivalue(w as _))
    }};
}

pub(crate) fn wvalues_to_ivalues(
    reader: &MemoryReader<'_>,
    wvalues: &[WValue],
    itypes: &[IType],
    record_types: &Rc<RecordTypes>,
) -> HostImportResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(wvalues.len());
    let mut wvalue = wvalues.iter();

    for itype in itypes.iter() {
        match itype {
            IType::Boolean => {
                let w = next_wvalue!(wvalue, I32);
                result.push(IValue::Boolean(w != 0))
            }
            IType::S8 => simple_wvalue_to_ivalue!(result, wvalue, I32, S8),
            IType::S16 => simple_wvalue_to_ivalue!(result, wvalue, I32, S16),
            IType::S32 => simple_wvalue_to_ivalue!(result, wvalue, I32, S32),
            IType::S64 => simple_wvalue_to_ivalue!(result, wvalue, I64, S64),
            IType::U8 => simple_wvalue_to_ivalue!(result, wvalue, I32, U8),
            IType::U16 => simple_wvalue_to_ivalue!(result, wvalue, I32, U16),
            IType::U32 => simple_wvalue_to_ivalue!(result, wvalue, I32, U32),
            IType::U64 => simple_wvalue_to_ivalue!(result, wvalue, I64, U64),
            IType::I32 => simple_wvalue_to_ivalue!(result, wvalue, I32, I32),
            IType::I64 => simple_wvalue_to_ivalue!(result, wvalue, I64, I64),
            IType::F32 => simple_wvalue_to_ivalue!(result, wvalue, F32, F32),
            IType::F64 => simple_wvalue_to_ivalue!(result, wvalue, F64, F64),
            IType::String => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let raw_str = reader.read_raw_u8_array(offset as _, size as _)?;
                let str = String::from_utf8(raw_str)?;
                result.push(IValue::String(str));
            }
            IType::ByteArray => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let array = reader.read_raw_u8_array(offset as _, size as _)?;
                result.push(IValue::ByteArray(array));
            }
            IType::Array(ty) => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let array = super::lift_array(reader, ty, offset as _, size as _, record_types)?;
                result.push(IValue::Array(array));
            }
            IType::Record(record_type_id) => {
                let record_type = record_types
                    .get(record_type_id)
                    .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;
                let offset = next_wvalue!(wvalue, I32);

                let record = super::lift_record(reader, record_type, offset as _, record_types)?;
                result.push(record);
            }
        }
    }

    Ok(result)
}
