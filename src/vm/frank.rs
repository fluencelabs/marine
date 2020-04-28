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
use crate::vm::module::{FrankModule, ModuleABI, ModuleAPI};
use crate::vm::{config::Config, errors::FrankError, service::FrankService};

use sha2::{digest::generic_array::GenericArray, digest::FixedOutput};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use wasmer_runtime::{func, Ctx};
use wasmer_runtime_core::import::ImportObject;

#[derive(Default)]
pub struct Dispatcher {
    api: HashMap<String, ModuleABI>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            api: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct Frank {
    modules: HashMap<String, FrankModule>,
    dispatcher: Arc<Mutex<Dispatcher>>,
}

impl Frank {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            dispatcher: Arc::new(Mutex::new(Dispatcher::new())),
        }
    }

    /*
    fn out_invoke(ctx: &mut Ctx, offset: i32, size: i32) -> i32 {
        let data = ctx.data as *mut Mutex<Dispatcher>;
        let dispatcher: Arc<Mutex<Dispatcher>> = unsafe { Arc::from_raw(data) };

        let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
        match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
            Some(msg) => print!("{}", msg),
            None => print!("frank logger: incorrect UTF8 string's been supplied to logger"),
        }
        1
    }
    */
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

        let dispatcher = self.dispatcher.clone();
        let dispatcher1 = move || {
            let dtor = (|_: *mut c_void| {}) as fn(*mut c_void);
            // TODO: try with Box
            let dispatcher2: Arc<Mutex<Dispatcher>> = dispatcher.clone();
            let raw_dispatcher =
                Arc::into_raw(dispatcher2) as *mut Mutex<Dispatcher> as *mut c_void;
            (raw_dispatcher, dtor)
        };

        let mut import_object = ImportObject::new_with_data(dispatcher1);
        let dispatcher = self.dispatcher.lock().unwrap();
        for (module, abi) in dispatcher.api.iter() {
            use wasmer_runtime_core::import::Namespace;

            // TODO: introduce a macro for such things
            let mut namespace = Namespace::new();
            let allocate = abi.allocate.clone();
            namespace.insert(
                config.allocate_fn_name.clone(),
                func!(move |_ctx: &mut Ctx, size: i32| -> i32 {
                    allocate
                        .as_ref()
                        .unwrap()
                        .call(size)
                        .expect("allocate failed")
                }),
            );

            let invoke = abi.invoke.clone();
            namespace.insert(
                config.invoke_fn_name.clone(),
                func!(move |_ctx: &mut Ctx, offset: i32, size: i32| -> i32 {
                    invoke
                        .as_ref()
                        .unwrap()
                        .call(offset, size)
                        .expect("invoke failed")
                }),
            );

            let deallocate = abi.deallocate.clone();
            namespace.insert(
                config.deallocate_fn_name.clone(),
                func!(move |_ctx: &mut Ctx, ptr: i32, size: i32| {
                    deallocate
                        .as_ref()
                        .unwrap()
                        .call(ptr, size)
                        .expect("deallocate failed");
                }),
            );

            let store = abi.store.clone();
            namespace.insert(
                config.store_fn_name.clone(),
                func!(move |_ctx: &mut Ctx, offset: i32, value: i32| {
                    store
                        .as_ref()
                        .unwrap()
                        .call(offset, value)
                        .expect("store failed")
                }),
            );

            let load = abi.load.clone();
            namespace.insert(
                config.load_fn_name.clone(),
                func!(move |_ctx: &mut Ctx, offset: i32| -> i32 {
                    load.as_ref().unwrap().call(offset).expect("load failed")
                }),
            );

            import_object.register(module, namespace);
        }

        let (module, module_abi) = FrankModule::new(&prepared_wasm_bytes, config, import_object)?;
        match self.modules.entry(module_name.clone()) {
            Entry::Vacant(entry) => entry.insert(module),
            Entry::Occupied(_) => return Err(FrankError::NonUniqueModuleName),
        };

        // registers new abi in a dispatcher
        let mut dispatcher = self.dispatcher.lock().unwrap();
        dispatcher.api.insert(module_name, module_abi);

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
