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

use it_memory_traits::MemoryView;

use std::marker::PhantomData;

/// A core trait for any backend. It serves two purposes:
/// * handles initialization of the library if needed
/// * provides access to all public types -- like `mod` but for trait impls.
pub trait WasmBackend: Clone + Send + Sync + 'static {
    /// A type that stores all the data, while most of the types are handles to data from `Store`.
    type Store: Store<Self>;
    /// A compiled, but not instantiated module.
    type Module: Module<Self>;
    /// An object that holds all the functions that are given to `Module` as imports.
    type Imports: Imports<Self>; // maybe rename to Linker?
    /// An instantiated module ready to be executed.
    type Instance: Instance<Self>;
    /// A temporary immutable handle to `Store`.
    type Context<'c>: Context<Self>;
    /// A temporary mutable handle to `Store`
    type ContextMut<'c>: ContextMut<Self>;
    /// A type that is used to pass context to imports.
    type ImportCallContext<'c>: ImportCallContext<Self>;
    /// A host function prepared to be used as import for instantiating a module, contained in `Store`.
    type HostFunction: HostFunction<Self> + FuncConstructor<Self>;
    /// An export function from a wasm instance, contained in `Store`
    type ExportFunction: ExportFunction<Self>;
    /// A wasm memory.
    type Memory: Memory<Self>;
    /// A view to the wasm memory.
    type MemoryView: MemoryView<DelayedContextLifetime<Self>>;

    /// Type that provides all WASI-related APIs.
    type Wasi: WasiImplementation<Self>;

    /// Creates a new wasm backend with default configuration. In future, a configuration
    /// may be passed as argument. The only option at the moment is an asynchronous backend.
    fn new_async() -> WasmBackendResult<Self>;
}

/// This struct is a helper, that allows passing `<WB as WasmBackend>::ContextMut` as template parameter,
/// but not specify a lifetime. Any local lifetime can be used instead.
pub struct DelayedContextLifetime<WB: WasmBackend> {
    _data: PhantomData<WB>,
}

impl<WB: WasmBackend> it_memory_traits::Store for DelayedContextLifetime<WB> {
    type ActualStore<'c> = <WB as WasmBackend>::ContextMut<'c>;
}
