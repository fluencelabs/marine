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

use crate::vm::module::{FrankModule, ModuleAPI};
use crate::vm::{config::Config, errors::FrankError, service::FrankService};
use crate::vm::module::frank_result::FrankResult;

use wasmer_runtime_core::import::ImportObject;
use sha2::{digest::generic_array::GenericArray, digest::FixedOutput};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::os::raw::c_void;

pub struct Dispatcher {
    api: HashMap<String, FrankModule>,
}

impl Dispatcher {
    pub fn new(api: HashMap<String, FrankModule>) -> Self {
        Self {
            api
        }
    }
}

pub struct Frank {
    modules: HashMap<String, FrankModule>,
}

impl Frank {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
}

impl FrankService for Frank {
    fn invoke(&mut self, module_name: String, argument: &[u8]) -> Result<FrankResult, FrankError> {
        match self.modules.entry(module_name) {
            Entry::Vacant(_) => Err(FrankError::NoSuchModule),
            Entry::Occupied(mut module) => module.get_mut().invoke(argument),
        }
    }

    fn register_module(
        &mut self,
        module_name: String,
        wasm_bytes: &[u8],
        config: Config,
    ) -> Result<(), FrankError> {
        let prepared_wasm_bytes =
            crate::vm::prepare::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let modules_copy = self.modules.clone();
        let dispatcher = move || {
            let dispatcher = Dispatcher::new(modules_copy);
            let dispatcher = Box::new(dispatcher);
            let dtor = (|data: *mut c_void| unsafe {
                drop(Box::from_raw(data as *mut Dispatcher));
            }) as fn(*mut c_void);

            // and then release corresponding Box object obtaining the raw pointer
            (Box::leak(dispatcher) as *mut Dispatcher as *mut c_void, dtor)
        };

        let mut import_object = ImportObject::new_with_data(dispatcher);
        //import_object.register();

        let module = FrankModule::new(&prepared_wasm_bytes, config, import_object)?;
        match self.modules.entry(module_name) {
            Entry::Vacant(entry) => entry.insert(module),
            Entry::Occupied(_) => return Err(FrankError::NonUniqueModuleName),
        };

        // registers new abi in a dispatcher

        Ok(())
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
