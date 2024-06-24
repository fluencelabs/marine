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

use super::WValue;
use super::HostImportResult;
use crate::IValue;

use it_lilo::lowerer::*;
use it_lilo::traits::Allocatable;
use it_memory_traits::MemoryView;

pub(crate) async fn ivalue_to_wvalues<
    A: Allocatable<MV, Store>,
    MV: MemoryView<Store>,
    Store: it_memory_traits::Store,
>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lowerer: &mut ILowerer<'_, A, MV, Store>,
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
            let offset = lowerer.writer.write_bytes(store, str.as_bytes()).await?;

            vec![WValue::I32(offset as _), WValue::I32(str.len() as _)]
        }
        Some(IValue::ByteArray(array)) => {
            let offset = lowerer.writer.write_bytes(store, &array).await?;

            vec![WValue::I32(offset as _), WValue::I32(array.len() as _)]
        }
        Some(IValue::Array(values)) => {
            let LoweredArray { offset, size } = array_lower_memory(store, lowerer, values).await?;
            vec![WValue::I32(offset as _), WValue::I32(size as _)]
        }
        Some(IValue::Record(values)) => {
            let offset = record_lower_memory(store, lowerer, values).await?;
            vec![WValue::I32(offset as i32)]
        }
        None => vec![],
    };

    Ok(result)
}
