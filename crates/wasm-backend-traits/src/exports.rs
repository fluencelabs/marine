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

pub static STANDARD_MEMORY_EXPORT_NAME: &str = "memory";
pub static STANDARD_MEMORY_INDEX: u32 = 0;

use crate::DelayedContextLifetime;
use crate::WasmBackend;

/// Contains Wasm exports necessary for internal usage.
#[derive(Clone)]
pub enum Export<WB: WasmBackend> {
    Memory(<WB as WasmBackend>::Memory),
    Function(<WB as WasmBackend>::ExportFunction),
    Other,
}

// TODO: add read/write/etc methods to the `Memory` trait,
// and then make a generic implementation of interface-types traits
/// A wasm memory handle.
/// As it is only a handle to an object in `Store`, cloning is cheap.
pub trait Memory<WB: WasmBackend>:
    it_memory_traits::Memory<<WB as WasmBackend>::MemoryView, DelayedContextLifetime<WB>>
    + Clone
    + Send
    + Sync
    + 'static
{
    /// Get the size of the allocated memory in bytes.
    fn size(&self, store: &mut <WB as WasmBackend>::ContextMut<'_>) -> usize;
}
