use crate::{InstantiationResult, WasmBackend};

/// A handle to compiled wasm module.
pub trait Module<WB: WasmBackend> {
    /// Returns custom sections corresponding to `key`, if there are any.
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;

    /// Instantiates module by allocating memory, VM state and linking imports with ones from `import` argument.
    /// # Panics:
    ///     If the `Store` given is not the same with `Store` used to create `Imports` and this object.
    fn instantiate(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        imports: &<WB as WasmBackend>::Imports,
    ) -> InstantiationResult<<WB as WasmBackend>::Instance>;
}
