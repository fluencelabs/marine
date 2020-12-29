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

use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
pub fn records() {
    let records_config_path = "../examples/records/Config.toml";

    let records_config_raw = std::fs::read(records_config_path)
        .expect("../examples/records/Config.toml should presence");

    let mut records_config: fluence_faas::TomlFaaSConfig =
        toml::from_slice(&records_config_raw).expect("records config should be well-formed");
    records_config.modules_dir = Some(String::from("../examples/records/artifacts/"));

    let mut faas = FluenceFaaS::with_raw_config(records_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas
        .call_with_ivalues("records_pure", "invoke", &[], <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    let right_result = json!({
        "field_0": 1,
        "field_1": 1,
        "field_2": 2,
        "field_3": 3,
        "field_4": 4,
        "field_5": 5,
        "field_6": 6,
        "field_7": 7,
        "field_8": 8,
        "field_9": 9.0,
        "field_10": 10.0,
        "field_11": "field_11",
        "field_12": [0x13, 0x37],
    });

    assert_eq!(
        result1,
        vec![IValue::Record(
            wasmer_wit::NEVec::new(vec![
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
                IValue::Array(vec![IValue::U8(0x13), IValue::U8(0x37)])
            ])
            .unwrap()
        )]
    );

    let result2 = faas
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!({
                "test_record": {
                    "field_0": 0,
                    "field_1": 0,
                    "field_2": 0,
                    "field_3": 0,
                    "field_4": 0,
                    "field_5": 0,
                    "field_6": 0,
                    "field_7": 0,
                    "field_8": 0,
                    "field_9": 0,
                    "field_10": 0,
                    "field_11": "field",
                    "field_12": vec![1],

                }
            }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result2, right_result);

    let result3 = faas
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!({
                "test_record": [0,0,0,0,0,0,0,0,0,0,0,"",[1]]

            }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result3, right_result);

    let result4 = faas
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!([{
                    "field_0": 0,
                    "field_1": 0,
                    "field_2": 0,
                    "field_3": 0,
                    "field_4": 0,
                    "field_5": 0,
                    "field_6": 0,
                    "field_7": 0,
                    "field_8": 0,
                    "field_9": 0,
                    "field_10": 0,
                    "field_11": "",
                    "field_12": vec![1],
                }
            ]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result4, right_result);

    let result5 = faas
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!([[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "", [1]]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result5, right_result);
}

#[test]
fn inner_records() {
    let inner_records_config_raw = std::fs::read("./tests/wasm_tests/inner_records/Config.toml")
        .expect("../examples/greeting/artifacts/greeting.wasm should presence");

    let mut inner_records_config: fluence_faas::TomlFaaSConfig =
        toml::from_slice(&inner_records_config_raw)
            .expect("argument passing test config should be well-formed");

    inner_records_config.modules_dir =
        Some(String::from("./tests/wasm_tests/inner_records/artifacts"));

    let mut faas = FluenceFaaS::with_raw_config(inner_records_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result = faas
        .call_with_json(
            "inner_records_pure",
            "test_record",
            json!({
                "test_record": {
                    "test_record_0": {
                        "field_0": 0
                    },
                    "test_record_1": {
                        "field_0": 1,
                        "field_1": "",
                        "field_2": vec![1],
                        "test_record_0": {
                            "field_0": 1
                        }
                    }
                }
            }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke inner_records_pure: {:?}", e));

    let right_result = json!({
            "test_record_0": {
                "field_0": 1
            },
            "test_record_1": {
                "field_0": 1,
                "field_1": "",
                "field_2": vec![1],
                "test_record_0": {
                    "field_0": 1
                }
            }
    });

    assert_eq!(result, right_result);
}
