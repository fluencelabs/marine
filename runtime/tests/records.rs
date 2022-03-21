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

use marine::Marine;
use marine::IValue;

#[test]
pub fn records() {
    let effector_wasm_bytes = std::fs::read("../examples/records/artifacts/records_effector.wasm")
        .expect("../examples/records/artifacts/records_effector.wasm should presence");

    let pure_wasm_bytes = std::fs::read("../examples/records/artifacts/records_pure.wasm")
        .expect("../examples/records/artifacts/records_pure.wasm should presence");

    let mut marine = Marine::new();
    let load_result = marine.load_module("pure", &pure_wasm_bytes, <_>::default());
    assert!(load_result.is_err());

    marine
        .load_module("records_effector", &effector_wasm_bytes, <_>::default())
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    marine
        .load_module("records_pure", &pure_wasm_bytes, <_>::default())
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let result = marine
        .call("records_pure", "invoke", &[])
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::Record(
            wasmer_it::NEVec::new(vec![
                IValue::Boolean(true),
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

#[test]
pub fn records_allocation() {
    let wasm_bytes = std::fs::read("./tests/wasm_tests/records_allocation/artifacts/records_allocation.wasm")
        .expect("./tests/wasm_tests/records_allocation/artifacts/records_allocation.wasm should be present");

    let mut marine = Marine::new();

    marine
        .load_module("records_allocation", &wasm_bytes, <_>::default())
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));


    let result = marine
        .call("records_allocation", "fill_2gb_mem", &[])
        .unwrap_or_else(|e| panic!("can't invoke fill_2gb_mem: {:?}", e));

    assert_eq!(
        result.len(),0
    );

    let args =   vec![IValue::Record(
        wasmer_it::NEVec::new(vec![
            IValue::S32(1),
            IValue::S32(1),
            IValue::S32(1),
            IValue::S32(1),
            IValue::S32(1),
        ]).unwrap()
    )];

    let result = marine
        .call("records_allocation", "pass_record", &args)
        .unwrap_or_else(|e| panic!("can't invoke pass_record: {:?}", e));



}
