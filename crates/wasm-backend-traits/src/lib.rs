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
pub use macros::*;

use std::marker::PhantomData;
use it_memory_traits::{MemoryView};

pub trait WasmBackend: Clone + Default + 'static {
    // general
    type Module: Module<Self>;
    type Instance: Instance<Self>;
    type Store: Store<Self>;
    type Context<'c>: Context<Self>;
    type ContextMut<'c>: ContextMut<Self>;
    type Caller<'c>: Caller<Self>;

    type Imports: Imports<Self>; // maybe rename to Linker?

    type Function: Function<Self> + FuncConstructor<Self>;
    type Memory: Memory<Self>;
    type MemoryView: MemoryView<DelayedContextLifetime<Self>>;

    // wasi
    type Wasi: WasiImplementation<Self>;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> CompilationResult<Self::Module>;
}

pub trait WasmType {
    type Type: Copy;

    fn into_type(self) -> Self::Type;
}

impl WasmType for i32 {
    type Type = i32;

    fn into_type(self) -> Self::Type {
        self
    }
}

pub struct DelayedContextLifetime<WB: WasmBackend> {
    _data: PhantomData<WB>,
}

impl<WB: WasmBackend> it_memory_traits::Store for DelayedContextLifetime<WB> {
    type ActualStore<'c> = <WB as WasmBackend>::ContextMut<'c>;
}
