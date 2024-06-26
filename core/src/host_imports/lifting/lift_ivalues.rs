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

use super::WType;
use super::WValue;
use super::HostImportError;
use super::HostImportResult;
use crate::IValue;
use crate::IType;

use it_lilo::lifter::*;
use it_lilo::traits::RecordResolvable;

macro_rules! next_wvalue {
    ($wvalue_iter:ident, $wtype:ident) => {
        match $wvalue_iter
            .next()
            .ok_or_else(|| HostImportError::MismatchWValuesCount)?
        {
            WValue::$wtype(v) => *v,
            v => return Err(HostImportError::MismatchWValues(WType::$wtype, v.clone())),
        }
    };
}

macro_rules! simple_wvalue_to_ivalue {
    ($result:ident, $wvalue_iter:ident, $wtype:ident, $ivalue:ident) => {{
        let w = next_wvalue!($wvalue_iter, $wtype);
        $result.push(IValue::$ivalue(w as _))
    }};
}

pub(crate) fn wvalues_to_ivalues<
    R: RecordResolvable,
    MV: MemoryView<S>,
    S: it_memory_traits::Store,
>(
    store: &mut <S as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, S>,
    wvalues: &[WValue],
    itypes: &[IType],
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

                let raw_str = lifter
                    .reader
                    .read_raw_u8_array(store, offset as _, size as _)?;
                let str = String::from_utf8(raw_str)?;
                result.push(IValue::String(str));
            }
            IType::ByteArray => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let array = lifter
                    .reader
                    .read_raw_u8_array(store, offset as _, size as _)?;
                result.push(IValue::ByteArray(array));
            }
            IType::Array(ty) => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let array = array_lift_memory(store, lifter, ty, offset as _, size as _)?;
                result.push(array);
            }
            IType::Record(record_type_id) => {
                let record_type = lifter.resolver.resolve_record(*record_type_id)?;
                let offset = next_wvalue!(wvalue, I32);

                let record = record_lift_memory(store, lifter, record_type, offset as _)?;
                result.push(record);
            }
        }
    }

    Ok(result)
}
