/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine::Marine;
use marine::MarineModuleInterface;
use marine::IValue;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use marine_wasm_backend_traits::WasmBackend;

use pretty_assertions::assert_eq;

use std::path::PathBuf;
use std::sync::Arc;

#[tokio::test]
pub async fn greeting() {
    let greeting_config_path = "../examples/greeting/Config.toml";

    let greeting_config_raw = std::fs::read(greeting_config_path)
        .expect("../examples/greeting/Config.toml should presence");

    let mut greeting_config: marine::TomlMarineConfig =
        toml::from_slice(&greeting_config_raw).expect("greeting config should be well-formed");
    greeting_config.modules_dir = Some(PathBuf::from("../examples/greeting/artifacts"));

    let mut faas =
        Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), greeting_config)
            .await
            .unwrap_or_else(|e| panic!("can't create Marine instance: {}", e));

    let result1 = faas
        .call_with_ivalues_async(
            "greeting",
            "greeting",
            &[IValue::String(String::from("Fluence"))],
            <_>::default(),
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    let result2 = faas
        .call_with_ivalues_async(
            "greeting",
            "greeting",
            &[IValue::String(String::from(""))],
            <_>::default(),
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    assert_eq!(result1, vec![IValue::String(String::from("Hi, Fluence"))]);
    assert_eq!(result2, vec![IValue::String(String::from("Hi, "))]);
}

#[tokio::test]
pub async fn get_interfaces() {
    let greeting_config_path = "../examples/greeting/Config.toml";

    let greeting_config_raw = std::fs::read(greeting_config_path)
        .expect("../examples/greeting/Config.toml should presence");

    let mut greeting_config: marine::TomlMarineConfig =
        toml::from_slice(&greeting_config_raw).expect("greeting config should be well-formed");
    greeting_config.modules_dir = Some(PathBuf::from("../examples/greeting/artifacts"));

    let faas = Marine::with_raw_config(WasmtimeWasmBackend::new_async().unwrap(), greeting_config)
        .await
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
