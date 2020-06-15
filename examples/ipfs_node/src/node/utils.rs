/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use wasmer_core::vm::Ctx;
use super::errors::NodeError;
use super::config::ModuleConfig;

use fce::FCEModuleConfig;

use wasmer_core::import::ImportObject;
use wasmer_runtime::func;
use wasmer_core::typed_func::WasmTypeList;
use wasmer_runtime::Func;
use wasmer_runtime::error::ResolveError;
use wasmer_core::backend::SigRegistry;
use wasmer_runtime::types::LocalOrImport;
use wasmer_core::module::ExportIndex;

use std::path::PathBuf;

// based on Wasmer: https://github.com/wasmerio/wasmer/blob/081f6250e69b98b9f95a8f62ad6d8386534f3279/lib/runtime-core/src/instance.rs#L863
/// Extract export function from Wasmer instance by name.
pub(super) unsafe fn get_export_func_by_name<'a, Args, Rets>(
    ctx: &'a mut Ctx,
    name: &str,
) -> Result<Func<'a, Args, Rets>, ResolveError>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    let module_inner = &(*ctx.module);

    let export_index =
        module_inner
            .info
            .exports
            .get(name)
            .ok_or_else(|| ResolveError::ExportNotFound {
                name: name.to_string(),
            })?;

    let export_func_index = match export_index {
        ExportIndex::Func(func_index) => func_index,
        _ => {
            return Err(ResolveError::ExportWrongType {
                name: name.to_string(),
            })
        }
    };

    let export_func_signature_idx = *module_inner
        .info
        .func_assoc
        .get(*export_func_index)
        .expect("broken invariant, incorrect func index");

    let export_func_signature = &module_inner.info.signatures[export_func_signature_idx];
    let export_func_signature_ref = SigRegistry.lookup_signature_ref(export_func_signature);

    if export_func_signature_ref.params() != Args::types()
        || export_func_signature_ref.returns() != Rets::types()
    {
        return Err(ResolveError::Signature {
            expected: (*export_func_signature).clone(),
            found: Args::types().to_vec(),
        });
    }

    let func_wasm_inner = module_inner
        .runnable_module
        .get_trampoline(&module_inner.info, export_func_signature_idx)
        .unwrap();

    let export_func_ptr = match export_func_index.local_or_import(&module_inner.info) {
        LocalOrImport::Local(local_func_index) => module_inner
            .runnable_module
            .get_func(&module_inner.info, local_func_index)
            .unwrap(),
        _ => {
            return Err(ResolveError::ExportNotFound {
                name: name.to_string(),
            })
        }
    };

    let typed_func: Func<Args, Rets, wasmer_core::typed_func::Wasm> =
        Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, ctx as _);

    Ok(typed_func)
}

/// Make FCE config based on parsed raw config.
pub(super) fn make_wasm_process_config(
    config: Option<ModuleConfig>,
) -> Result<FCEModuleConfig, NodeError> {
    use super::imports::create_host_import_func;
    use super::imports::log_utf8_string;
    use wasmer_core::import::Namespace;

    let mut wasm_module_config = FCEModuleConfig::default();

    let module_config = match config {
        Some(config) => config,
        None => return Ok(wasm_module_config),
    };

    if let Some(mem_pages_count) = module_config.mem_pages_count {
        wasm_module_config.mem_pages_count = mem_pages_count;
    }

    let mut namespace = Namespace::new();

    if let Some(logger_enabled) = module_config.logger_enabled {
        if logger_enabled {
            namespace.insert("log_utf8_string", func!(log_utf8_string));
        }
    }

    if let Some(wasi) = module_config.wasi {
        if let Some(envs) = wasi.envs {
            wasm_module_config.wasi_envs = envs;
        }

        if let Some(preopened_files) = wasi.preopened_files {
            wasm_module_config.wasi_preopened_files = preopened_files
                .iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
        }

        if let Some(mapped_dirs) = wasi.mapped_dirs {
            wasm_module_config.wasi_mapped_dirs = mapped_dirs
                .into_iter()
                .map(|(from, to)| (from, PathBuf::from(to)))
                .collect::<Vec<_>>();
        }

        let mapped_dirs = wasm_module_config
            .wasi_mapped_dirs
            .iter()
            .map(|(from, to)| (format!("{}={}", from, to.as_path().to_str().unwrap()).into_bytes()))
            .collect::<Vec<_>>();

        wasm_module_config.wasi_envs.extend(mapped_dirs);
    };

    if let Some(imports) = module_config.imports {
        for (import_name, host_cmd) in imports {
            let host_import = create_host_import_func(host_cmd);
            namespace.insert(import_name, host_import);
        }
    }

    let mut import_object = ImportObject::new();
    import_object.register("host", namespace);

    wasm_module_config.imports = import_object;
    wasm_module_config.wasi_version = wasmer_wasi::WasiVersion::Latest;

    Ok(wasm_module_config)
}

#[macro_export] // https://github.com/rust-lang/rust/issues/57966#issuecomment-461077932
/// Initialize Wasm function in form of Box<RefCell<Option<Func<'static, args, rets>>>> only once.
macro_rules! init_wasm_func_once {
    ($func:ident, $ctx:ident, $args:ty, $rets:ty, $func_name:ident, $ret_error_code: expr) => {
        if $func.borrow().is_none() {
            let raw_func =
                match super::utils::get_export_func_by_name::<$args, $rets>($ctx, $func_name) {
                    Ok(func) => func,
                    Err(_) => return vec![Value::I32($ret_error_code)],
                };

            // assumed that this function will be used only in the context of closure
            // linked to a corresponding Wasm import - os it is safe to make is static
            let raw_func = std::mem::transmute::<Func<'_, _, _>, Func<'static, _, _>>(raw_func);

            *$func.borrow_mut() = Some(raw_func);
        }
    };
}

#[macro_export]
/// Call Wasm function that have Box<RefCell<Option<Func<'static, args, rets>>>> type.
macro_rules! call_wasm_func {
    ($func:ident, $arg:expr) => {
        $func.borrow().as_ref().unwrap().call($arg).unwrap()
    };
}
