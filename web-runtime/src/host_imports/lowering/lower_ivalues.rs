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

use super::WValue;
use super::HostImportResult;
use crate::IValue;

use it_lilo::lowerer::*;
use it_lilo::traits::Allocatable;

pub(crate) fn ivalue_to_wvalues<A: Allocatable>(
    lowerer: &ILowerer<'_, A>,
    ivalue: Option<IValue>,
) -> HostImportResult<Vec<WValue>> {
    let result = match ivalue {
        Some(IValue::Boolean(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S8(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S16(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::U8(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U16(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::I32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::I64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::F32(v)) => vec![WValue::F32(v)],
        Some(IValue::F64(v)) => vec![WValue::F64(v)],
        Some(IValue::String(str)) => {
            let offset = lowerer.writer.write_bytes(str.as_bytes())?;

            vec![WValue::I32(offset as _), WValue::I32(str.len() as _)]
        }
        Some(IValue::ByteArray(array)) => {
            let offset = lowerer.writer.write_bytes(&array)?;

            vec![WValue::I32(offset as _), WValue::I32(array.len() as _)]
        }
        Some(IValue::Array(values)) => {
            let LoweredArray { offset, size } = array_lower_memory(lowerer, values)?;
            vec![WValue::I32(offset as _), WValue::I32(size as _)]
        }
        Some(IValue::Record(values)) => {
            let offset = record_lower_memory(lowerer, values)?;
            vec![WValue::I32(offset)]
        }
        None => vec![],
    };

    Ok(result)
}
