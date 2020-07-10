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

const GREETING_MODULE_CONFIG_PATH: &str = "Config.toml";

fn main() -> Result<(), anyhow::Error> {
    let mut greeting_node = FluenceFaaS::new(PathBuf::from(GREETING_MODULE_CONFIG_PATH))?;
    println!(
        "greeting node interface is\n{}",
        greeting_node.get_interface()
    );

    let result = greeting_node.call_module(
        "greeting.wasm",
        "greeting",
        &[IValue::String("Fluence".to_string()), IValue::I32(1)],
    )?;

    println!("execution result {:?}", result);

    Ok(())
}
