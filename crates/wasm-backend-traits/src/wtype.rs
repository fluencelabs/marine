/*
 * Copyright 2023 Fluence Labs Limited
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

#[derive(Debug, Clone, PartialEq)]
pub enum WValue {
    /// The `i32` type.
    I32(i32),
    /// The `i64` type.
    I64(i64),
    /// The `f32` type.
    F32(f32),
    /// The `f64` type.
    F64(f64),
}

impl From<i32> for WValue {
    fn from(value: i32) -> Self {
        WValue::I32(value)
    }
}

impl From<i64> for WValue {
    fn from(value: i64) -> Self {
        WValue::I64(value)
    }
}

impl From<f32> for WValue {
    fn from(value: f32) -> Self {
        WValue::F32(value)
    }
}

impl From<f64> for WValue {
    fn from(value: f64) -> Self {
        WValue::F64(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WType {
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `v128` type, unsupported.
    V128,
    /// ExternRef type, unsupported.
    ExternRef,
    /// FuncRef type, unsupported.
    FuncRef,
}

impl WType {
    pub fn is_supported(&self) -> bool {
        !matches!(self, Self::ExternRef | Self::FuncRef | Self::V128)
    }
}

impl WValue {
    pub fn to_u128(&self) -> u128 {
        match *self {
            Self::I32(x) => x as u128,
            Self::I64(x) => x as u128,
            Self::F32(x) => f32::to_bits(x) as u128,
            Self::F64(x) => f64::to_bits(x) as u128,
        }
    }

    /// Converts any value to i32. Floats are interpreted as plain bytes.
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::I32(x) => x,
            Self::I64(x) => x as i32,
            Self::F32(x) => f32::to_bits(x) as i32,
            Self::F64(x) => f64::to_bits(x) as i32,
        }
    }
}

impl std::fmt::Display for WType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
