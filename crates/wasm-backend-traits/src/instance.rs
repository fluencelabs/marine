use crate::{Export, ResolveResult, RuntimeResult, WasmBackend};

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut <WB as WasmBackend>::Store,
    ) -> Box<dyn Iterator<Item = (String, Export<WB>)> + 'a>;

    fn memory(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        memory_index: u32,
    ) -> <WB as WasmBackend>::Memory;

    fn memory_by_name(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        memory_name: &str,
    ) -> Option<<WB as WasmBackend>::Memory>;

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut <WB as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<
        Box<dyn Fn(&mut <WB as WasmBackend>::Store) -> RuntimeResult<()> + Sync + Send + 'a>,
    >;

    fn get_dyn_func<'a>(
        &'a self,
        store: &mut <WB as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Function>;
}
