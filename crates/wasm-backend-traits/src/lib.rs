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

pub use errors::*;
pub use exports::*;
pub use imports::*;
pub use store::*;
pub use wasi::*;
pub use wtype::*;
pub use module::*;
pub use instance::*;
pub use caller::*;
pub use function::*;

use std::marker::PhantomData;
use it_memory_traits::{MemoryView};
use wasmer_it::interpreter::wasm;


pub trait WasmBackend: Clone + Default + 'static {
    // general
    type Module: Module<Self>;
    type Instance: Instance<Self>;
    type Store: Store<Self>;
    type Context<'c>: Context<Self>;
    type ContextMut<'c>: ContextMut<Self>;
    type Caller<'c>: Caller<Self>;

    // imports/exports -- subject to improvement
    type Imports: Imports<Self>; // to be replaced with somethink like Linker or Resolver
    type DynamicFunc: DynamicFunc<'static, Self>;
    type Namespace: Namespace<Self>;

    type Function: Function<Self>;
    type Memory: Memory<Self>;
    type MemoryView: MemoryView<DelayedContextLifetime<Self>> + 'static;

    // wasi
    type Wasi: WasiImplementation<Self>;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> WasmBackendResult<Self::Module>;
}

pub struct DelayedContextLifetime<WB: WasmBackend> {
    _data: PhantomData<WB>,
}

impl<WB: WasmBackend> it_memory_traits::Store for DelayedContextLifetime<WB> {
    type ActualStore<'c> = <WB as WasmBackend>::ContextMut<'c>;
}

