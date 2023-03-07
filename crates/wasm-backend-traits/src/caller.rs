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

use crate::AsContextMut;
use crate::FuncGetter;
use crate::WasmBackend;

/// `Caller` is a structure used to pass context to imports.
/// It serves as a handle to `Store`, and also provides access to `Memory` and export functions
/// from the caller instance, if there is one.
pub trait Caller<WB: WasmBackend>:
    FuncGetter<WB, (i32, i32), i32>
    + FuncGetter<WB, (i32, i32), ()>
    + FuncGetter<WB, i32, i32>
    + FuncGetter<WB, i32, ()>
    + FuncGetter<WB, (), i32>
    + FuncGetter<WB, (), ()>
    + AsContextMut<WB>
{
    /// Gets the `Memory` from the caller instance.
    /// Returns `None` if function was called directly from host.
    fn memory(&mut self, memory_index: u32) -> Option<<WB as WasmBackend>::Memory>;
}
