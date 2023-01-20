use crate::{AsContextMut, Export, ResolveResult, WasmBackend};

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(
        &'a self,
        store: <WB as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WB>)> + 'a>;

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_index: u32,
    ) -> Option<<WB as WasmBackend>::Memory>;

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Memory>;

    fn get_function<'a>(
        &'a self,
        store: &mut impl AsContextMut<WB>,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Function>;
}
