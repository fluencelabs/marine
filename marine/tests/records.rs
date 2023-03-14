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

use pretty_assertions::assert_eq;
use serde_json::json;

use std::collections::HashMap;
use std::path::PathBuf;

#[test]
pub fn records() {
    let records_config_path = "../examples/records/Config.toml";

    let records_config_raw = std::fs::read(records_config_path)
        .expect("../examples/records/Config.toml should presence");

    let mut records_config: marine::TomlMarineConfig =
        toml::from_slice(&records_config_raw).expect("records config should be well-formed");
    records_config.modules_dir = Some(PathBuf::from("../examples/records/artifacts/"));

    let mut marine = Marine::with_raw_config(records_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let result1 = marine
        .call_with_ivalues("records_pure", "invoke", &[], <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    let expected_result = json!({
        "field_0": true,
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

    let result2 = marine
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!({
                "test_record": {
                    "field_0": false,
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

    assert_eq!(result2, expected_result);

    let result3 = marine
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!({
                "test_record": [false,0,0,0,0,0,0,0,0,0,0,"",[1]]

            }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result3, expected_result);

    let result4 = marine
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!([{
                    "field_0": false,
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

    assert_eq!(result4, expected_result);

    let result5 = marine
        .call_with_json(
            "records_effector",
            "mutate_struct",
            json!([[false, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "", [1]]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(result5, expected_result);
}

#[test]
fn records_passing() {
    let inner_records_config_raw = std::fs::read("./tests/wasm_tests/records_passing/Config.toml")
        .expect("./tests/wasm_tests/records_passing/Config.toml should presence");

    let mut records_passing_config: marine::TomlMarineConfig =
        toml::from_slice(&inner_records_config_raw)
            .expect("argument passing test config should be well-formed");

    records_passing_config.modules_dir = Some(PathBuf::from(
        "./tests/wasm_tests/records_passing/artifacts",
    ));

    let mut marine = Marine::with_raw_config(records_passing_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result = marine
            .call_with_json(
                "records_passing_pure",
                func_name,
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

        let expected_result = json!({
            "test_record_0": {
                "field_0": 1
            },
            "test_record_1": {
                "field_0": 1,
                "field_1": "fluence",
                "field_2": vec![0x13, 0x37],
                "test_record_0": {
                    "field_0": 5
                }
            }
        });

        assert_eq!(result, expected_result);
    };

    test("test_record");
    test("test_record_ref");
}

#[test]
fn records_destruction() {
    let inner_records_config_raw = std::fs::read("./tests/wasm_tests/records_passing/Config.toml")
        .expect("./tests/wasm_tests/records_passing/Config.toml should presence");

    let mut records_passing_config: marine::TomlMarineConfig =
        toml::from_slice(&inner_records_config_raw)
            .expect("argument passing test config should be well-formed");

    records_passing_config.modules_dir = Some(PathBuf::from(
        "./tests/wasm_tests/records_passing/artifacts",
    ));

    let mut marine = Marine::with_raw_config(records_passing_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let record_array = json!([
            {
                    "data": {"id": 3},
                    "data2": [{"id": 4}],
            },
            [
                {
                    "data": {"id": 1},
                    "data2": [{"id": 2}],
                }
            ]
    ]);

    let _result = marine
        .call_with_json(
            "records_passing_pure",
            "pass_droppable_record",
            record_array,
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    let result = marine
        .call_with_json(
            "records_passing_pure",
            "get_drop_count",
            json!([]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    // host -> pure -> effector -> pure -> host
    //       ^       ^           ^
    // Three arrows inside a module, each arrow is an allocation of 4 records for arguments
    // and 4 records for return value,
    // so 4*2*2 + 4*2 destructions must happen.
    assert_eq!(result, json!([16, 8]));
}

#[test]
fn records_return_frees() {
    let inner_records_config_raw = std::fs::read("./tests/wasm_tests/records_passing/Config.toml")
        .expect("./tests/wasm_tests/records_passing/Config.toml should presence");

    let mut records_passing_config: marine::TomlMarineConfig =
        toml::from_slice(&inner_records_config_raw)
            .expect("argument passing test config should be well-formed");

    records_passing_config.modules_dir = Some(PathBuf::from(
        "./tests/wasm_tests/records_passing/artifacts",
    ));

    let mut marine = Marine::with_raw_config(records_passing_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let _result = marine
        .call_with_json(
            "records_passing_pure",
            "return_256kb_struct",
            json!([]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    let stats_after_first_call = marine
        .module_memory_stats()
        .0
        .iter()
        .map(|stat| (stat.name.to_string(), stat.memory_size))
        .collect::<HashMap<String, usize>>();

    for _ in 0..128 {
        let _result = marine
            .call_with_json(
                "records_passing_pure",
                "return_256kb_struct",
                json!([]),
                <_>::default(),
            )
            .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

        for stat in marine.module_memory_stats().0 {
            let memory_size = stats_after_first_call.get(stat.name).unwrap();
            assert_eq!(*memory_size, stat.memory_size)
        }
    }
}

#[test]
fn records_pass_frees() {
    let records_passing_config =
        marine::TomlMarineConfig::load("./tests/wasm_tests/records_passing/Config.toml")
            .expect("argument passing test config should be well-formed");

    let struct_64b = json!({
        "field1": 0,
        "field2": 0,
        "field3": 0,
        "field4": 0,
        "field5": 0,
        "field6": 0,
        "field7": 0,
        "field8": 0,
        "field11": 0,
        "field12": 0,
        "field13": 0,
        "field14": 0,
        "field15": 0,
        "field16": 0,
        "field17": 0,
        "field18": 0,
    });

    let struct_1kb = json!({
        "field1": struct_64b,
        "field2": struct_64b,
        "field3": struct_64b,
        "field4": struct_64b,
        "field5": struct_64b,
        "field6": struct_64b,
        "field7": struct_64b,
        "field8": struct_64b,
        "field11": struct_64b,
        "field12": struct_64b,
        "field13": struct_64b,
        "field14": struct_64b,
        "field15": struct_64b,
        "field16": struct_64b,
        "field17": struct_64b,
        "field18": struct_64b,
    });

    let struct_16kb = json!({
        "field1": struct_1kb,
        "field2": struct_1kb,
        "field3": struct_1kb,
        "field4": struct_1kb,
        "field5": struct_1kb,
        "field6": struct_1kb,
        "field7": struct_1kb,
        "field8": struct_1kb,
        "field11": struct_1kb,
        "field12": struct_1kb,
        "field13": struct_1kb,
        "field14": struct_1kb,
        "field15": struct_1kb,
        "field16": struct_1kb,
        "field17": struct_1kb,
        "field18": struct_1kb,
    });

    let struct_256kb = json!({
        "field1": struct_16kb,
        "field2": struct_16kb,
        "field3": struct_16kb,
        "field4": struct_16kb,
        "field5": struct_16kb,
        "field6": struct_16kb,
        "field7": struct_16kb,
        "field8": struct_16kb,
        "field11": struct_16kb,
        "field12": struct_16kb,
        "field13": struct_16kb,
        "field14": struct_16kb,
        "field15": struct_16kb,
        "field16": struct_16kb,
        "field17": struct_16kb,
        "field18": struct_16kb,
    });

    let mut marine = Marine::with_raw_config(records_passing_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let _result = marine
        .call_with_json(
            "records_passing_pure",
            "pass_256kb_struct",
            json!([struct_256kb]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    let stats_after_first_call = marine
        .module_memory_stats()
        .0
        .iter()
        .map(|stat| (stat.name.to_string(), stat.memory_size))
        .collect::<HashMap<String, usize>>();

    for _ in 0..128 {
        let _result = marine
            .call_with_json(
                "records_passing_pure",
                "pass_256kb_struct",
                json!([struct_256kb.clone()]),
                <_>::default(),
            )
            .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

        for stat in marine.module_memory_stats().0 {
            let memory_size = stats_after_first_call.get(stat.name).unwrap();
            assert_eq!(*memory_size, stat.memory_size)
        }
    }
}
