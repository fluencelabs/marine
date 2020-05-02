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

use crate::vm::module::frank_result::FrankResult;
use crate::vm::module::{FrankModule, ModuleAPI};
use crate::vm::{config::Config, errors::FrankError, service::FrankService};

use sha2::{digest::generic_array::GenericArray, digest::FixedOutput};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use wasmer_runtime::{func, Ctx};
use wasmer_runtime_core::import::{ImportObject, Namespace};

pub struct Frank {
    // set of modules registered inside Frank
    modules: HashMap<String, FrankModule>,

    // contains ABI of each registered module in specific format for Wasmer
    abi_import_object: ImportObject,
}

impl Frank {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            abi_import_object: ImportObject::new(),
        }
    }

    /// Extracts ABI of a module into Namespace.
    fn create_import_object(module: &FrankModule, config: &Config) -> Namespace {
        let mut namespace = Namespace::new();
        let module_abi = module.get_abi();

        // TODO: introduce a macro for such things
        let allocate = module_abi.allocate.clone().unwrap();
        namespace.insert(
            config.allocate_fn_name.clone(),
            func!(move |_ctx: &mut Ctx, size: i32| -> i32 {
                allocate.call(size).expect("allocate failed")
            }),
        );

        let invoke = module_abi.invoke.clone().unwrap();
        namespace.insert(
            config.invoke_fn_name.clone(),
            func!(move |_ctx: &mut Ctx, offset: i32, size: i32| -> i32 {
                invoke.call(offset, size).expect("invoke failed")
            }),
        );

        let deallocate = module_abi.deallocate.clone().unwrap();
        namespace.insert(
            config.deallocate_fn_name.clone(),
            func!(move |_ctx: &mut Ctx, ptr: i32, size: i32| {
                deallocate.call(ptr, size).expect("deallocate failed");
            }),
        );

        let store = module_abi.store.clone().unwrap();
        namespace.insert(
            config.store_fn_name.clone(),
            func!(move |_ctx: &mut Ctx, offset: i32, value: i32| {
                store.call(offset, value).expect("store failed")
            }),
        );

        let load = module_abi.load.clone().unwrap();
        namespace.insert(
            config.load_fn_name.clone(),
            func!(move |_ctx: &mut Ctx, offset: i32| -> i32 {
                load.call(offset).expect("load failed")
            }),
        );

        namespace
    }
}

impl Default for Frank {
    fn default() -> Self {
        Self::new()
    }
}

impl FrankService for Frank {
    fn invoke(&mut self, module_name: &str, argument: &[u8]) -> Result<FrankResult, FrankError> {
        match self.modules.get_mut(module_name) {
            Some(module) => module.invoke(argument),
            None => Err(FrankError::NoSuchModule),
        }
    }

    fn register_module<S>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: Config,
    ) -> Result<(), FrankError>
    where
        S: Into<String>,
    {
        let prepared_wasm_bytes =
            crate::vm::prepare::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let module = FrankModule::new(
            &prepared_wasm_bytes,
            config.clone(),
            self.abi_import_object.clone(),
        )?;

        // registers ABI of newly registered module in abi_import_object
        let namespace = Frank::create_import_object(&module, &config);
        let module_name: String = module_name.into();
        self.abi_import_object
            .register(module_name.clone(), namespace);

        match self.modules.entry(module_name) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(_) => Err(FrankError::NonUniqueModuleName),
        }
    }

    fn unregister_module(&mut self, module_name: &str) -> Result<(), FrankError> {
        self.modules
            .remove(module_name)
            .ok_or_else(|| FrankError::NoSuchModule)?;
        // unregister abi from a dispatcher

        Ok(())
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
