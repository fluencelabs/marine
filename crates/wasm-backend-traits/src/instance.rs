use crate::{AsContextMut, Export, ResolveResult, WasmBackend};

/// A handle to an instantiated Wasm module. Cloning is cheap.
pub trait Instance<WB: WasmBackend>: Clone {
    /// Returns an `Iterator` to all exports of this instance.
    fn export_iter<'a>(
        &'a self,
        store: <WB as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WB>)> + 'a>;

    /// Returns nth memory export, None if there is no nth memory.
    /// No guaranties is known for memory order, but almost always a module has only one memory,
    /// hence the only valid value for `memory_index` is 0.
    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_index: u32,
    ) -> Option<<WB as WasmBackend>::Memory>;

    /// Returns a memory export with given name.
    /// # Errors:
    ///     Returns an error if there is no export with such name, or it is not a memory.
    fn get_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Memory>;

    /// Returns a function export with given name.
    /// # Errors:
    ///     Returns an error if there is no export with such name, or it is not a function.
    fn get_function<'a>(
        &'a self,
        store: &mut impl AsContextMut<WB>,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Function>;
}
