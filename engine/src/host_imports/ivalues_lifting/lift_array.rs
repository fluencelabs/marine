use crate::IValue;

use wasmer_core::memory::ptr::{Array, WasmPtr};
use wasmer_core::vm::Ctx;
use wasmer_wit::IRecordType;
use wasmer_wit::NEVec;

pub(super) fn read_u8_array(ctx: &Ctx, offset: usize, size: usize) -> IValue {

}