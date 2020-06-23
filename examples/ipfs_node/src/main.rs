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

use fluence_faas::FluenceFaaS;
use fluence_faas::IValue;

use std::path::PathBuf;
use anyhow::Context;

const IPFS_MODULES_CONFIG_PATH: &str = "Config.toml";
const IPFS_RPC: &str = "wasm/artifacts/wasm_ipfs_rpc_wit.wasi.wasm";

fn main() -> Result<(), anyhow::Error> {
    let ipfs_rpc = std::fs::read(IPFS_RPC).context(format!("{} wasn't found", IPFS_RPC))?;

    let mut ipfs_node = FluenceFaaS::new(PathBuf::from(IPFS_MODULES_CONFIG_PATH))?;

    println!("ipfs node interface is\n{}", ipfs_node.get_interface());

    let node_addresses = ipfs_node.call_module("ipfs_node.wasm", "get_addresses", &[])?;

    println!("ipfs node addresses are:\n{:?}", node_addresses);

    let result = ipfs_node.call_code(
        &ipfs_rpc,
        "put",
        &[IValue::String("Hello, world".to_string())],
    )?;

    println!("execution result {:?}", result);

    Ok(())
}
