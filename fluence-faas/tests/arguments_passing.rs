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
use fluence_faas::IType;

use once_cell::sync::Lazy;
use serde_json::json;

static ARG_CONFIG: Lazy<fluence_faas::TomlFaaSConfig> = Lazy::new(|| {
    let mut arguments_passing_config =
        fluence_faas::TomlFaaSConfig::load("./tests/json_wasm_tests/arguments_passing/Config.toml")
            .expect("toml faas config should be created");

    arguments_passing_config.modules_dir = Some(String::from("./tests/json_wasm_tests/arguments_passing/artifacts"));

    arguments_passing_config
});

#[test]
pub fn get_interfaces() {
    let faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let interface = faas.get_interface();

    let string_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::String,
    }];
    let string_type_output_types = vec![IType::String];

    let string_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &string_type_arguments,
        output_types: &string_type_output_types,
    };

    let bytearray_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U8)),
    }];
    let bytearray_type_output_types = vec![IType::Array(Box::new(IType::U8))];

    let bytearray_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &bytearray_type_arguments,
        output_types: &bytearray_type_output_types,
    };

    let i32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S32,
    }];
    let i32_type_output_types = vec![IType::S32];

    let i32_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &i32_type_arguments,
        output_types: &i32_type_output_types,
    };

    let i64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S64,
    }];

    let i64_type_output_types = vec![IType::S64];

    let i64_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &i64_type_arguments,
        output_types: &i64_type_output_types,
    };

    let u32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U32,
    }];
    let u32_type_output_types = vec![IType::U32];

    let u32_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &u32_type_arguments,
        output_types: &u32_type_output_types,
    };

    let u64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U64,
    }];
    let u64_type_output_types = vec![IType::U64];

    let u64_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &u64_type_arguments,
        output_types: &u64_type_output_types,
    };

    let f32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F32,
    }];
    let f32_type_output_types = vec![IType::F32];

    let f32_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &f32_type_arguments,
        output_types: &f32_type_output_types,
    };

    let f64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F64,
    }];
    let f64_type_output_types = vec![IType::F64];

    let f64_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &f64_type_arguments,
        output_types: &f64_type_output_types,
    };

    let empty_type_arguments = vec![];
    let empty_type_output_types = vec![IType::String];

    let empty_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &empty_type_arguments,
        output_types: &empty_type_output_types,
    };

    let bool_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::I32,
    }];
    let bool_type_output_types = vec![IType::I32];

    let bool_type_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &bool_type_arguments,
        output_types: &bool_type_output_types,
    };

    let all_types_arguments = vec![
        fluence_faas::IFunctionArg {
            name: String::from("arg_0"),
            ty: IType::S8,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_1"),
            ty: IType::S16,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_2"),
            ty: IType::S32,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_3"),
            ty: IType::S64,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_4"),
            ty: IType::U8,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_5"),
            ty: IType::U16,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_6"),
            ty: IType::U32,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_7"),
            ty: IType::U64,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_8"),
            ty: IType::F32,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_9"),
            ty: IType::F64,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_10"),
            ty: IType::String,
        },
        fluence_faas::IFunctionArg {
            name: String::from("arg_11"),
            ty: IType::Array(Box::new(IType::U8)),
        },
    ];
    let all_types_output_types = vec![IType::Array(Box::new(IType::U8))];

    let all_types_sign = fluence_faas::FaaSFunctionSignature {
        arguments: &all_types_arguments,
        output_types: &all_types_output_types,
    };

    let mut functions = std::collections::HashMap::new();
    functions.insert("string_type", string_type_sign);
    functions.insert("bytearray_type", bytearray_type_sign);
    functions.insert("i32_type", i32_type_sign);
    functions.insert("i64_type", i64_type_sign);
    functions.insert("u32_type", u32_type_sign);
    functions.insert("u64_type", u64_type_sign);
    functions.insert("f32_type", f32_type_sign);
    functions.insert("f64_type", f64_type_sign);
    functions.insert("empty_type", empty_type_sign);
    functions.insert("bool_type", bool_type_sign);
    functions.insert("all_types", all_types_sign);

    let mut modules = std::collections::HashMap::new();
    modules.insert("arguments_passing_pure", functions.clone());
    modules.insert("arguments_passing_effector", functions);

    assert_eq!(
        interface,
        fluence_faas::FaaSInterface {
            record_types: <_>::default(),
            modules
        }
    );
}

#[test]
pub fn all_types() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "all_types", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "all_types", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "all_types",
            json!({
              "arg_0": 0,
              "arg_1": 1,
              "arg_2": 2,
              "arg_3": 3,
              "arg_4": 4,
              "arg_5": 5,
              "arg_6": 6,
              "arg_7": 7,
              "arg_8": 8.1,
              "arg_9": 9.1,
              "arg_10": "fluence",
              "arg_11": vec! [0x13, 0x37],
            }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke all_types: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(
            vec![
                0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 65, 1, 153,
                154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55, 0, 1, 0, 2, 0, 0, 0, 3,
                0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 65, 1, 153, 154, 64, 34, 51, 51, 51,
                51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55
            ]
            .iter()
            .map(|v| IValue::U8(*v))
            .collect::<Vec<_>>()
        )]
    );

    let result4 = faas
        .call_with_json(
            "arguments_passing_pure",
            "all_types",
            json!([0, 1, 2, 3, 4, 5, 6, 7, 8.1, 9.1, "fluence", vec![0x13, 0x37]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke all_types: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(
            vec![
                0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 65, 1, 153,
                154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55, 0, 1, 0, 2, 0, 0, 0, 3,
                0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 65, 1, 153, 154, 64, 34, 51, 51, 51,
                51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55
            ]
            .iter()
            .map(|v| IValue::U8(*v))
            .collect::<Vec<_>>()
        )]
    );
}

#[test]
pub fn i32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "i32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "i32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "i32_type",
            json!({ "arg": 1 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(result3, vec![IValue::S32(3)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "i32_type", json!(1), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(result4, vec![IValue::S32(3)]);
}

#[test]
pub fn i64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "i64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "i64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "i64_type",
            json!({ "arg": 1 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(result3, vec![IValue::S64(3)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "i64_type", json!(1), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(result4, vec![IValue::S64(3)]);
}

#[test]
pub fn u32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "u32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "u32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "u32_type",
            json!({ "arg": 1 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u32_type: {:?}", e));
    assert_eq!(result3, vec![IValue::U32(3)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "u32_type", json!(1), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke u32_type: {:?}", e));
    assert_eq!(result4, vec![IValue::U32(3)]);
}

#[test]
pub fn u64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "u64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "u64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "u64_type",
            json!({ "arg": 1 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u64_type: {:?}", e));
    assert_eq!(result3, vec![IValue::U64(3)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "u64_type", json!(1), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke u64_type: {:?}", e));
    assert_eq!(result4, vec![IValue::U64(3)]);
}

#[test]
pub fn f32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "f32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "f32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "f32_type",
            json!({ "arg": 1.0 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f32_type: {:?}", e));
    assert_eq!(result3, vec![IValue::F32(3.0)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "f32_type", json!(1.0), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke f32_type: {:?}", e));
    assert_eq!(result4, vec![IValue::F32(3.0)]);
}

#[test]
pub fn f64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "f64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "f64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "f64_type",
            json!({ "arg": 1.0 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(result3, vec![IValue::F64(3.0)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "f64_type", json!(1.0), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(result4, vec![IValue::F64(3.0)]);
}

#[test]
pub fn string_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "string_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "string_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "string_type",
            json!({ "arg": "Fluence" }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke string_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::String(String::from("Fluence_Fluence_Fluence_Fluence"))]
    );

    let result4 = faas
        .call_with_json(
            "arguments_passing_pure",
            "string_type",
            json!("Fluence"),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke string_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::String(String::from("Fluence_Fluence_Fluence_Fluence"))]
    );
}

#[test]
pub fn bytearray_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "bytearray_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "bytearray_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "bytearray_type",
            json!({ "arg": [0x13, 0x37] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::U8(0x13),
            IValue::U8(0x37),
            IValue::U8(1),
            IValue::U8(1)
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arguments_passing_pure",
            "bytearray_type",
            json!([[0x13, 0x37]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::U8(0x13),
            IValue::U8(0x37),
            IValue::U8(1),
            IValue::U8(1)
        ])]
    );
}

#[test]
pub fn bool_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arguments_passing_pure", "bool_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arguments_passing_pure", "bool_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arguments_passing_pure",
            "bool_type",
            json!({ "arg": 0 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bool_type: {:?}", e));
    assert_eq!(result3, vec![IValue::I32(1)]);

    let result4 = faas
        .call_with_json("arguments_passing_pure", "bool_type", json!(0), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke bool_type: {:?}", e));
    assert_eq!(result4, vec![IValue::I32(1)]);
}

#[test]
pub fn empty_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas
        .call_with_json("arguments_passing_pure", "empty_type", json!({}), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(result1, vec![IValue::String(String::from("success"))]);

    let result2 = faas
        .call_with_json("arguments_passing_pure", "empty_type", json!([]), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(result2, vec![IValue::String(String::from("success"))]);

    let result3 = faas
        .call_with_json("arguments_passing_pure", "empty_type", json!([]), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(result3, vec![IValue::String(String::from("success"))]);

    let result4 = faas.call_with_json("arguments_passing_pure", "empty_type", json!([1]), <_>::default());
    assert!(result4.is_err());
}
