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

use crate::WasmBackend;

/// `Store` is an object that stores modules, instances, functions memories and so on.
/// `Store` is grow-only: once something added, it will not be removed until Store is destroyed.
/// Some of the implementations can limit allocated resources.
/// For example, Wasmtime cannot have more than 10000 instances in one `Store`.
///
/// Most of the functions in this crate require a handle to `Store` to work.
pub trait Store<WB: WasmBackend>: AsContextMut<WB> {
    fn new(backend: &WB) -> Self;

    // TODO: create general/backend-specific core config?
    fn set_memory_limit(&mut self, memory_limit: u64);
}

/// A temporary immutable handle to store
pub trait Context<WB: WasmBackend>: AsContext<WB> {}

/// A temporary mutable handle to store
pub trait ContextMut<WB: WasmBackend>: AsContextMut<WB> {}

pub trait AsContext<WB: WasmBackend> {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_>;
}

pub trait AsContextMut<WB: WasmBackend>: AsContext<WB> {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_>;
}
