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

mod node;
mod errors;

use fce::IValue;
use fce::FCE;
use fce::FCEError;
use fce::FCEModuleConfig;
use fce::WasmProcess;

const IPFS_NODE: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/target/wasm32-unknown-unknown/release/ipfs_node_wit.wasm";

const IPFS_RPC: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/target/wasm32-unknown-unknown/release/ipfs_rpc_wit.wasm";

fn main() {
    let ipfs_node_bytes = std::fs::read(IPFS_NODE).unwrap();
    let ipfs_rpc_bytes = std::fs::read(IPFS_RPC).unwrap();

    let mut fce = FCE::new();
    let config = FCEModuleConfig::default();

    println!("loading ipfs node module");
    fce.load_module("node", &ipfs_node_bytes, config.clone())
        .expect("module successfully created");

    println!("loading ipfs rpc module");
    fce.load_module("rpc", &ipfs_rpc_bytes, config.clone())
        .expect("module successfully created");

    let result = fce
        .call("node_rpc", "invoke", &[IValue::String("aaaa".to_string())])
        .unwrap();

    println!("execution result {:?}", result);
}

/*
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
        Some(msg) => println!("host ipfs_call: {}", msg),
        None => println!("fce logger: incorrect UTF8 string's been supplied to logger"),
    }
}
*/
