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
    // /// The `v128` type.
    //V128(u128),
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
    // /// The `v128` type.
    // V128,
}

impl WValue {
    pub fn to_u128(&self) -> u128 {
        match *self {
            Self::I32(x) => x as u128,
            Self::I64(x) => x as u128,
            Self::F32(x) => f32::to_bits(x) as u128,
            Self::F64(x) => f64::to_bits(x) as u128,
            //Self::V128(x) => x,
        }
    }
}

impl std::fmt::Display for WType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
