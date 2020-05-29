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

mod instance;

use crate::instance::wit_module::WITModule;

use wasmer_interface_types::values::InterfaceValue;
use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_core::vm::Ctx;

const FILE_NAME: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/target/wasm32-unknown-unknown/release/export_test_wit.wasm";

fn main() {
    let wasm_bytes = std::fs::read(FILE_NAME).unwrap();
    let logger_imports = imports! {
        "logger" => {
            "log_utf8_string" => func!(logger_log_utf8_string),
        },
    };
    let mut import_object = ImportObject::new();
    import_object.extend(logger_imports);

    let mut module =
        WITModule::new(&wasm_bytes, &import_object).expect("module successfully created");

    let result1 = module
        .call("strlen", &[InterfaceValue::String("aaaaaa".to_string())])
        .unwrap();
    let result2 = module
        .call("greeting", &[InterfaceValue::String("Mike".to_string())])
        .unwrap();

    println!("stack state {:?}", result1);
    println!("stack state {:?}", result2);
}

fn logger_log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_runtime_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => print!("fce logger: incorrect UTF8 string's been supplied to logger"),
    }
}
