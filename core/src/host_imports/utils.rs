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
use crate::IType;

pub(super) fn itypes_args_to_wtypes(itypes: &[IType]) -> Vec<WType> {
    itypes
        .iter()
        .flat_map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) => vec![WType::I32, WType::I32],
            _ => vec![WType::I32],
        })
        .collect()
}

pub(super) fn itypes_output_to_wtypes(itypes: &[IType]) -> Vec<WType> {
    itypes
        .iter()
        .flat_map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) | IType::Record(_) => vec![],
            _ => vec![WType::I32],
        })
        .collect()
}

#[macro_export]
/// Initialize Wasm function in form of Box<RefCell<Option<Func<'static, args, rets>>>>.
/// This macro does not cache result.
macro_rules! init_wasm_func {
    ($func:ident, $ctx:expr, $args:ty, $rets:ty, $func_name:ident, $ret_error_code: expr) => {
        let $func: TypedFunc<WB, $args, $rets> = match { $ctx.get_func($func_name) } {
            Ok(func) => func,
            Err(_) => return vec![WValue::I32($ret_error_code)],
        };
    };
}

#[macro_export]
/// Call Wasm function that have Box<RefCell<Option<Func<'static, args, rets>>>> type.
macro_rules! call_wasm_func {
    ($func:expr, $store:expr, $($arg:expr),*) => {
        $func($store, ($($arg),*)).await.unwrap()
    };
}
