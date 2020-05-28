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

use crate::vm::module::fce_result::FCEResult;
use crate::vm::module::{FCEModule, ModuleAPI};
use crate::vm::{config::Config, errors::FCEError, service::FCEService};

use sha2::{digest::generic_array::GenericArray, digest::FixedOutput};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use wasmer_runtime::func;
use wasmer_runtime_core::import::{ImportObject, Namespace};

pub struct FCE {
    // set of modules registered inside FCE
    modules: HashMap<String, FCEModule>,

    // contains ABI of each registered module in specific format for Wasmer
    abi_import_object: ImportObject,
}

impl FCE {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            abi_import_object: ImportObject::new(),
        }
    }

    /// Extracts ABI of a module into Namespace.
    fn create_namespace_from_module(module: &FCEModule, config: &Config) -> Namespace {
        let mut namespace = Namespace::new();
        let module_abi = module.get_abi().clone();

        // TODO: introduce a macro for such things
        let allocate = module_abi.allocate;
        namespace.insert(
            config.allocate_fn_name.clone(),
            func!(move |size: i32| -> i32 { allocate.call(size).expect("allocate failed") }),
        );

        let invoke = module_abi.invoke;
        namespace.insert(
            config.invoke_fn_name.clone(),
            func!(move |offset: i32, size: i32| -> i32 {
                invoke.call(offset, size).expect("invoke failed")
            }),
        );

        let deallocate = module_abi.deallocate;
        namespace.insert(
            config.deallocate_fn_name.clone(),
            func!(move |ptr: i32, size: i32| {
                deallocate.call(ptr, size).expect("deallocate failed");
            }),
        );

        let store = module_abi.store;
        namespace.insert(
            config.store_fn_name.clone(),
            func!(move |offset: i32, value: i32| {
                store.call(offset, value).expect("store failed")
            }),
        );

        let load = module_abi.load;
        namespace.insert(
            config.load_fn_name.clone(),
            func!(move |offset: i32| -> i32 { load.call(offset).expect("load failed") }),
        );

        namespace
    }
}

impl Default for FCE {
    fn default() -> Self {
        Self::new()
    }
}

impl FCEService for FCE {
    fn invoke(&mut self, module_name: &str, argument: &[u8]) -> Result<FCEResult, FCEError> {
        match self.modules.get_mut(module_name) {
            Some(module) => module.invoke(argument),
            None => Err(FCEError::NoSuchModule),
        }
    }

    fn register_module<S>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: Config,
    ) -> Result<(), FCEError>
    where
        S: Into<String>,
    {
        let prepared_wasm_bytes =
            crate::vm::prepare::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let module = FCEModule::new(
            &prepared_wasm_bytes,
            config.clone(),
            self.abi_import_object.clone(),
        )?;

        // registers ABI of newly registered module in abi_import_object
        let namespace = FCE::create_namespace_from_module(&module, &config);
        let module_name: String = module_name.into();
        self.abi_import_object
            .register(module_name.clone(), namespace);

        match self.modules.entry(module_name) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(_) => Err(FCEError::NonUniqueModuleName),
        }
    }

    fn unregister_module(&mut self, module_name: &str) -> Result<(), FCEError> {
        match self.modules.entry(module_name.to_string()) {
            Entry::Vacant(_) => Err(FCEError::NoSuchModule),

            Entry::Occupied(module) => {
                module.remove_entry();
                Ok(())
            }
        }
    }

    fn compute_state_hash(
        &mut self,
    ) -> GenericArray<u8, <sha2::Sha256 as FixedOutput>::OutputSize> {
        use sha2::Digest;

        let mut hasher = sha2::Sha256::new();

        let sha256_size = 256;
        let mut hash_vec: Vec<u8> = Vec::with_capacity(self.modules.len() * sha256_size);
        for (_, module) in self.modules.iter_mut() {
            hash_vec.extend_from_slice(module.compute_state_hash().as_slice());
        }

        hasher.input(hash_vec);
        hasher.result()
    }
}
