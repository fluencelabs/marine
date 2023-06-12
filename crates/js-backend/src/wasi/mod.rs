pub(crate) mod js_imports;

use std::collections::HashMap;

use anyhow::anyhow;
use wasm_bindgen::JsError;
use wasm_bindgen::JsValue;

use marine_wasm_backend_traits::prelude::*;

use crate::JsFunction;
use crate::JsContextMut;
use crate::JsCaller;
use crate::JsWasmBackend;

pub struct JsWasi {}

impl WasiImplementation<JsWasmBackend> for JsWasi {
    fn register_in_linker(
        store: &mut JsContextMut<'_>,
        linker: &mut <JsWasmBackend as WasmBackend>::Imports,
        config: WasiParameters,
    ) -> Result<(), WasiError> {
        let context_index = store.inner.wasi_contexts.len();
        store
            .inner
            .wasi_contexts
            .push(WasiContext::new(config.envs)?);
        linker.add_wasi(context_index);

        Ok(())
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <JsWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        Box::new(JsWasiState {})
    }
}

pub struct JsWasiState {}

impl WasiState for JsWasiState {
    fn envs(&self) -> &[Vec<u8>] {
        &[]
    }
}

fn constrain_ctx_getter<F>(func: F) -> F
where
    F: for<'a> Fn(JsContextMut<'a>) -> &'a mut WasiContext + Send + Sync + Copy + 'static,
{
    func
}

pub(crate) struct WasiContext {
    envs: HashMap<String, String>,
    wasi_impl: JsValue,
}

impl WasiContext {
    pub(crate) fn new(envs: HashMap<String, String>) -> Result<Self, WasiError> {
        let envs_js = serde_wasm_bindgen::to_value(&envs)
            .map_err(|e| WasiError::EngineWasiError(anyhow!(e.to_string())))?;

        Ok(Self {
            envs,
            wasi_impl: js_imports::create_wasi(envs_js),
        })
    }

    pub(crate) fn get_imports(&self, module: &js_sys::WebAssembly::Module) -> js_sys::Object {
        js_imports::generate_wasi_imports(module, &self.wasi_impl).into()
    }

    pub(crate) fn bind_to_instance(&self, instance: &js_sys::WebAssembly::Instance) {
        js_imports::bind_to_instance(&self.wasi_impl, instance)
    }
}

fn wasi_snapshot_preview1_exports<F>(
    mut store: &mut impl AsContextMut<JsWasmBackend>,
    ctx_getter: F,
) -> Vec<(String, JsFunction)>
where
    F: for<'a> Fn(JsContextMut<'a>) -> &'a mut WasiContext + Send + Sync + Copy + 'static,
{
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
    ]
    .iter()
    .map(|name| make_wasi_mock_func(name, store, ctx_getter))
    .collect::<Vec<_>>();

    namespace
}

fn make_wasi_mock_func<F>(
    name: &'static str,
    store: &mut impl AsContextMut<JsWasmBackend>,
    ctx_getter: F,
) -> (String, JsFunction)
where
    F: for<'a> Fn(JsContextMut<'a>) -> &'a mut WasiContext + Send + Sync + Copy + 'static,
{
    let js_func = IntoFunc::into_func(
        move |mut store: JsCaller<'_>| {
            let wasi_ctx = ctx_getter(store.as_context_mut());
            log::error!("called unimplemented wasi function \"{name}\"");
        },
        store,
    );

    (name.to_string(), js_func)
}
