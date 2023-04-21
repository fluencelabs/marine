use anyhow::anyhow;
use marine_wasm_backend_traits::prelude::*;
use crate::{JsFunction, JsWasmBackend};

pub struct JsWasi {}

impl WasiImplementation<JsWasmBackend> for JsWasi {
    fn register_in_linker(
        store: &mut <JsWasmBackend as WasmBackend>::ContextMut<'_>,
        linker: &mut <JsWasmBackend as WasmBackend>::Imports,
        config: WasiParameters,
    ) -> Result<(), WasiError> {
        let wasi_namespace = wasi_snapshot_preview1_exports(store);
        linker.register(store, "wasi_snapshot_preview1", wasi_namespace.into_iter())
            .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <JsWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        todo!()
    }
}


fn wasi_snapshot_preview1_exports(
    mut store: &mut impl AsContextMut<JsWasmBackend>,
) -> Vec<(String, JsFunction)> {
    let namespace = [
        "args_get",
        "args_sizes_get",
        "clock_res_get",
        "clock_time_get",
        "environ_get",
        "environ_sizes_get",
        "fd_advise",
        "fd_allocate",
        "fd_close",
        "fd_datasync",
        "fd_fdstat_get",
        "fd_fdstat_set_flags",
        "fd_fdstat_set_rights",
        "fd_filestat_get",
        "fd_filestat_set_size",
        "fd_filestat_set_times",
        "fd_pread",
        "fd_prestat_get",
        "fd_prestat_dir_name",
        "fd_pwrite",
        "fd_read",
        "fd_readdir",
        "fd_renumber",
        "fd_seek",
        "fd_sync",
        "fd_tell",
        "fd_write",
        "path_create_directory",
        "path_filestat_get",
        "path_filestat_set_times",
        "path_link",
        "path_open",
        "path_readlink",
        "path_remove_directory",
        "path_rename",
        "path_symlink",
        "path_unlink_file",
        "poll_oneoff",
        "proc_exit",
        "proc_raise",
        "random_get",
        "sched_yield",
        "sock_recv",
        "sock_send",
        "sock_shutdown",
        "thread-spawn",
    ].iter().map(|name| make_wasi_mock_func(name, store)).collect::<Vec<_>>();

    namespace
}

fn make_wasi_mock_func(name: &'static str, store: &mut impl AsContextMut<JsWasmBackend>) -> (String, JsFunction) {
    let js_func = move || {
        log::error!("called unimplemented wasi function \"{name}\"");
    };

    (name.to_string(), js_func.into_func(store))
}