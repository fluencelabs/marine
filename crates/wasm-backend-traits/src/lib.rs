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

pub mod errors;
pub mod exports;
pub mod imports;
pub mod store;
pub mod wasi;
pub mod wtype;
pub mod module;
pub mod instance;
pub mod caller;
pub mod function;
pub mod macros;

/// Helper functions for backend implementations.
pub mod impl_utils;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::exports::*;
    pub use crate::imports::*;
    pub use crate::store::*;
    pub use crate::wasi::*;
    pub use crate::wtype::*;
    pub use crate::module::*;
    pub use crate::instance::*;
    pub use crate::caller::*;
    pub use crate::function::*;
    pub use crate::WasmBackend;
    pub use crate::DelayedContextLifetime;
}

pub use prelude::*;

pub use macros::*;

use std::marker::PhantomData;
use it_memory_traits::MemoryView;

pub trait WasmBackend: Clone + Default + 'static {
    type Store: Store<Self>;
    type Module: Module<Self>;
    type Imports: Imports<Self>; // maybe rename to Linker?
    type Instance: Instance<Self>;
    type Context<'c>: Context<Self>;
    type ContextMut<'c>: ContextMut<Self>;
    type Caller<'c>: Caller<Self>;

    type Function: Function<Self> + FuncConstructor<Self>;
    type Memory: Memory<Self>;
    type MemoryView: MemoryView<DelayedContextLifetime<Self>>;

    type Wasi: WasiImplementation<Self>;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> CompilationResult<Self::Module>;

    fn new() -> Self;
}

pub struct DelayedContextLifetime<WB: WasmBackend> {
    _data: PhantomData<WB>,
}

impl<WB: WasmBackend> it_memory_traits::Store for DelayedContextLifetime<WB> {
    type ActualStore<'c> = <WB as WasmBackend>::ContextMut<'c>;
}
