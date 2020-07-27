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

use std::path::PathBuf;

const RECORDS_MODULES_CONFIG_PATH: &str = "Config.toml";

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let mut records_test = FluenceFaaS::new(PathBuf::from(RECORDS_MODULES_CONFIG_PATH))?;
    println!("ipfs node interface is\n{}", records_test.get_interface());

    let result = records_test.call_module("pure.wasm", "invoke", &[])?;
    println!("execution result {:?}", result);

    Ok(())
}
