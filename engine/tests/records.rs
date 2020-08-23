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

use fce::FCE;
use fce::IValue;

#[test]
pub fn records() {
    let effector_wasm_bytes =
        std::fs::read("../examples/records/artifacts/wasm_modules/effector.wasm")
            .expect("examples/greeting/artifacts/greeting.wasm should presence");

    let pure_wasm_bytes = std::fs::read("../examples/records/artifacts/wasm_modules/pure.wasm")
        .expect("examples/greeting/artifacts/greeting.wasm should presence");

    let mut fce = FCE::new();
    let load_result = fce.load_module("pure", &pure_wasm_bytes, <_>::default());
    assert!(load_result.is_err());

    fce.load_module("effector.wasm", &effector_wasm_bytes, <_>::default())
        .unwrap_or_else(|e| panic!("can't load a module into FCE: {:?}", e));

    fce.load_module("pure.wasm", &pure_wasm_bytes, <_>::default())
        .unwrap_or_else(|e| panic!("can't load a module into FCE: {:?}", e));

    let result = fce
        .call("pure.wasm", "invoke", &[])
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::Record(
            wasmer_wit::vec1::Vec1::new(vec![
                IValue::I32(1),
                IValue::S8(1),
                IValue::S16(2),
                IValue::S32(3),
                IValue::S64(4),
                IValue::U8(5),
                IValue::U16(6),
                IValue::U32(7),
                IValue::U64(8),
                IValue::F32(9.0),
                IValue::F64(10.0),
                IValue::String(String::from("field_11")),
                IValue::ByteArray(vec![0x13, 0x37])
            ])
            .unwrap()
        )]
    );
}
