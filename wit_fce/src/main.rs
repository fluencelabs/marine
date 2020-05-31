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
#![feature(get_mut_unchecked)]
#![feature(new_uninit)]

mod instance;

use crate::instance::wit_module::WITModule;

use std::collections::HashMap;
use std::sync::Arc;
use wasmer_interface_types::values::InterfaceValue;
use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_core::vm::Ctx;

const IPFS_NODE: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/target/wasm32-unknown-unknown/release/ipfs_node_wit.wasm";

const IPFS_RPC: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/target/wasm32-unknown-unknown/release/ipfs_rpc_wit.wasm";

fn main() {
    let ipfs_node_bytes = std::fs::read(IPFS_NODE).unwrap();
    let ipfs_rpc_bytes = std::fs::read(IPFS_RPC).unwrap();
    let imports = imports! {
        "logger" => {
            "log_utf8_string" => func!(logger_log_utf8_string),
        },
        "host" => {
            "ipfs" => func!(ipfs_call),
        }
    };
    let mut import_object = ImportObject::new();
    import_object.extend(imports);
    let mut modules = HashMap::new();

    println!("loading ipfs node module");
    let ipfs_node = WITModule::new(&ipfs_node_bytes, import_object.clone(), &modules)
        .expect("module successfully created");
    modules.insert("node".to_string(), Arc::new(ipfs_node));

    println!("loading ipfs rpc module");
    let mut ipfs_rpc = WITModule::new(&ipfs_rpc_bytes, import_object, &modules)
        .expect("module successfully created");

    let result1 = ipfs_rpc
        .call("invoke", &[InterfaceValue::String("aaaaaa".to_string())])
        .unwrap();

    println!("stack state {:?}", result1);
}

fn logger_log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_runtime_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => print!("fce logger: incorrect UTF8 string's been supplied to logger"),
    }
}

fn ipfs_call(ctx: &mut Ctx, ptr: i32, size: i32) {
    use wasmer_runtime_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(ptr as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("ipfs_call {}", msg),
        None => print!("fce logger: incorrect UTF8 string's been supplied to logger"),
    }
}
