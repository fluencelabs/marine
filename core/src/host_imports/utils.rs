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

#[macro_export] // https://github.com/rust-lang/rust/issues/57966#issuecomment-461077932
/// Initialize Wasm function in form of Box<RefCell<Option<Func<'static, args, rets>>>>.
/// This macro does not cache result.
macro_rules! init_wasm_func {
    ($func:ident, $ctx:ident, $args:ty, $rets:ty, $func_name:ident, $ret_error_code: expr) => {
        let mut $func: Box<
            dyn FnMut(&mut <WB as WasmBackend>::ContextMut<'_>, $args) -> RuntimeResult<$rets>
                + Send
                + Sync,
        > = match { $ctx.get_func($func_name) } {
            Ok(func) => func,
            Err(_) => return vec![WValue::I32($ret_error_code)],
        };
    };
}

#[macro_export]
/// Call Wasm function that have Box<RefCell<Option<Func<'static, args, rets>>>> type.
macro_rules! call_wasm_func {
    ($func:expr, $store:expr, $($arg:expr),*) => {
        $func.as_mut()($store, ($($arg),*)).unwrap()
    };
}
