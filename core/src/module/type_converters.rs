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

use super::IType;
use super::IValue;

use marine_wasm_backend_traits::WType;
use marine_wasm_backend_traits::WValue;

pub(super) fn wtype_to_itype(ty: &WType) -> IType {
    match ty {
        WType::I32 => IType::I32,
        WType::I64 => IType::I64,
        WType::F32 => IType::F32,
        WType::F64 => IType::F64,
        ty => {
            eprintln!("trying to convert {:?}", ty);
            unimplemented!()
        }
    }
}

pub(super) fn itype_to_wtype(ty: &IType) -> WType {
    match ty {
        IType::S8 => WType::I32,
        IType::S16 => WType::I32,
        IType::S32 => WType::I32,
        IType::S64 => WType::I64,
        IType::U8 => WType::I32,
        IType::U16 => WType::I32,
        IType::U32 => WType::I32,
        IType::U64 => WType::I64,
        IType::I32 => WType::I32,
        IType::I64 => WType::I64,
        IType::F32 => WType::F32,
        IType::F64 => WType::F64,
        ty => {
            eprintln!("trying to convert {:?}", ty);
            unimplemented!()
        }
    }
}

pub(super) fn ival_to_wval(value: &IValue) -> WValue {
    match value {
        IValue::I32(v) => WValue::I32(*v),
        IValue::I64(v) => WValue::I64(*v),
        IValue::F32(v) => WValue::F32(*v),
        IValue::F64(v) => WValue::F64(*v),
        _ => unimplemented!(),
    }
}

pub(super) fn wval_to_ival(value: &WValue) -> IValue {
    match value {
        WValue::I32(v) => IValue::I32(*v),
        WValue::I64(v) => IValue::I64(*v),
        WValue::F32(v) => IValue::F32(*v),
        WValue::F64(v) => IValue::F64(*v),
    }
}
