use crate::{AsContextMut, Export, ResolveResult, RuntimeResult, WasmBackend};

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut impl AsContextMut<WB>,
    ) -> Box<dyn Iterator<Item = (&'a String, Export<WB>)> + 'a>;

    fn memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_index: u32,
    ) -> <WB as WasmBackend>::Memory;

    fn memory_by_name(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_name: &str,
    ) -> Option<<WB as WasmBackend>::Memory>;

    fn get_function<'a>(
        &'a self,
        store: &mut impl AsContextMut<WB>,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Function>;
}
