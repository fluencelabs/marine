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

use std::path::PathBuf;
use marine::DefaultMarine;
use marine::MarineModuleInterface;
use marine::IValue;

use pretty_assertions::assert_eq;

use std::sync::Arc;

#[test]
pub fn greeting() {
    let greeting_config_path = "../examples/greeting/Config.toml";

    let greeting_config_raw = std::fs::read(greeting_config_path)
        .expect("../examples/greeting/Config.toml should presence");

    let mut greeting_config: marine::TomlMarineConfig =
        toml::from_slice(&greeting_config_raw).expect("greeting config should be well-formed");
    greeting_config.modules_dir = Some(PathBuf::from("../examples/greeting/artifacts"));

    let mut faas = DefaultMarine::with_raw_config(greeting_config)
        .unwrap_or_else(|e| panic!("can't create Marine instance: {}", e));

    let result1 = faas
        .call_with_ivalues(
            "greeting",
            "greeting",
            &[IValue::String(String::from("Fluence"))],
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    let result2 = faas
        .call_with_ivalues(
            "greeting",
            "greeting",
            &[IValue::String(String::from(""))],
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    assert_eq!(result1, vec![IValue::String(String::from("Hi, Fluence"))]);
    assert_eq!(result2, vec![IValue::String(String::from("Hi, "))]);
}

#[test]
pub fn get_interfaces() {
    let greeting_config_path = "../examples/greeting/Config.toml";

    let greeting_config_raw = std::fs::read(greeting_config_path)
        .expect("../examples/greeting/Config.toml should presence");

    let mut greeting_config: marine::TomlMarineConfig =
        toml::from_slice(&greeting_config_raw).expect("greeting config should be well-formed");
    greeting_config.modules_dir = Some(PathBuf::from("../examples/greeting/artifacts"));

    let faas = DefaultMarine::with_raw_config(greeting_config)
        .unwrap_or_else(|e| panic!("can't create Marine instance: {}", e));

    let interface = faas.get_interface();

    let arguments = vec![marine::IFunctionArg {
        name: String::from("name"),
        ty: marine::IType::String,
    }];
    let output_types = vec![marine::IType::String];

    let greeting_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("greeting")),
        arguments: Arc::new(arguments),
        outputs: Arc::new(output_types),
    };

    let record_types = std::collections::HashMap::new();
    let module_interface = MarineModuleInterface {
        record_types: &record_types,
        function_signatures: vec![greeting_sign],
    };

    let mut modules = std::collections::HashMap::new();
    modules.insert("greeting", module_interface);

    assert_eq!(interface, marine::MarineInterface { modules });
}
