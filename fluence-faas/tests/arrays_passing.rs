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
    let mut arrays_passing_config =
        fluence_faas::TomlFaaSConfig::load("./tests/wasm_tests/arrays_passing/Config.toml")
            .expect("toml faas config should be created");

    arrays_passing_config.modules_dir =
        Some(String::from("./tests/wasm_tests/arrays_passing/artifacts"));

    arrays_passing_config
});

#[test]
pub fn get_interfaces() {
    use std::collections::HashSet;

    let faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let interface = faas.get_interface();

    let byte_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U8)),
    }];
    let byte_type_outputs = vec![IType::Array(Box::new(IType::U8))];

    let byte_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "byte_type",
        arguments: &byte_type_arguments,
        outputs: &byte_type_outputs,
    };

    let inner_arrays_1_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::Array(Box::new(IType::Array(Box::new(
            IType::Array(Box::new(IType::U8)),
        )))))),
    }];
    let inner_arrays_1_outputs = vec![IType::Array(Box::new(IType::Array(Box::new(
        IType::Array(Box::new(IType::Array(Box::new(IType::U8)))),
    ))))];

    let inner_arrays_1_sign = fluence_faas::FaaSFunctionSignature {
        name: "inner_arrays_1",
        arguments: &inner_arrays_1_arguments,
        outputs: &inner_arrays_1_outputs,
    };

    // save it until record will be refactored in the future
    /*
    let inner_arrays_2_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::Array(Box::new(IType::Array(Box::new(
            IType::Array(Box::new(IType::Record(6))),
        )))))),
    }];
    let inner_arrays_2_outputs = vec![IType::Array(Box::new(IType::Array(Box::new(
        IType::Array(Box::new(IType::Array(Box::new(IType::Record(6))))),
    ))))];

    let inner_arrays_2_sign = fluence_faas::FaaSFunctionSignature {
        name: "inner_arrays_1",
        arguments: &inner_arrays_2_arguments,
        outputs: &inner_arrays_2_outputs,
    };
     */

    let string_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::String)),
    }];
    let string_type_outputs = vec![IType::Array(Box::new(IType::String))];

    let string_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "string_type",
        arguments: &string_type_arguments,
        outputs: &string_type_outputs,
    };

    let i32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::S32)),
    }];
    let i32_type_outputs = vec![IType::Array(Box::new(IType::S32))];

    let i32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "i32_type",
        arguments: &i32_type_arguments,
        outputs: &i32_type_outputs,
    };

    let i64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::S64)),
    }];

    let i64_type_outputs = vec![IType::Array(Box::new(IType::S64))];

    let i64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "i64_type",
        arguments: &i64_type_arguments,
        outputs: &i64_type_outputs,
    };

    let u32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U32)),
    }];
    let u32_type_outputs = vec![IType::Array(Box::new(IType::U32))];

    let u32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "u32_type",
        arguments: &u32_type_arguments,
        outputs: &u32_type_outputs,
    };

    let u64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U64)),
    }];
    let u64_type_outputs = vec![IType::Array(Box::new(IType::U64))];

    let u64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "u64_type",
        arguments: &u64_type_arguments,
        outputs: &u64_type_outputs,
    };

    let f32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::F32)),
    }];
    let f32_type_outputs = vec![IType::Array(Box::new(IType::F32))];

    let f32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "f32_type",
        arguments: &f32_type_arguments,
        outputs: &f32_type_outputs,
    };

    let f64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::F64)),
    }];
    let f64_type_outputs = vec![IType::Array(Box::new(IType::F64))];

    let f64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "f64_type",
        arguments: &f64_type_arguments,
        outputs: &f64_type_outputs,
    };

    let empty_type_arguments = vec![];
    let empty_type_outputs = vec![IType::Array(Box::new(IType::String))];

    let empty_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "empty_type",
        arguments: &empty_type_arguments,
        outputs: &empty_type_outputs,
    };

    /*
    let bool_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::I32,
    }];
    let bool_type_outputs = vec![IType::I32];

    let bool_type_sign = fluence_faas::FaaSFunctionSignature {
        name: "bool_type",
        arguments: &bool_type_arguments,
        outputs: &bool_type_outputs,
    };
     */

    let functions = vec![
        byte_type_sign,
        inner_arrays_1_sign,
        string_type_sign,
        f32_type_sign,
        f64_type_sign,
        u32_type_sign,
        u64_type_sign,
        i32_type_sign,
        i64_type_sign,
        empty_type_sign,
    ];

    let pure_module_name = "arrays_passing_pure";
    let effector_module_name = "arrays_passing_effector";

    let pure_module_interface = interface
        .modules
        .get(pure_module_name)
        .expect(&format!("{} should present in interface", pure_module_name));
    let effector_module_interface = interface
        .modules
        .get(effector_module_name)
        .expect(&format!("{} should present in interface", pure_module_name));

    assert!(!pure_module_interface.record_types.is_empty());
    assert!(!effector_module_interface.record_types.is_empty());

    let pure_module_functions: HashSet<_> = pure_module_interface
        .function_signatures
        .iter()
        .filter(|f| f.name != "inner_arrays_2")
        .collect();
    let effector_module_functions: HashSet<_> = effector_module_interface
        .function_signatures
        .iter()
        .filter(|f| f.name != "inner_arrays_2")
        .collect();

    let functions: HashSet<_> = functions.iter().collect();

    assert_eq!(pure_module_functions, functions);
    assert_eq!(effector_module_functions, functions);
}

#[test]
pub fn i32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!([[]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(
        result1,
        vec![IValue::Array(vec![
            IValue::S32(0),
            IValue::S32(1),
            IValue::S32(2),
            IValue::S32(3),
            IValue::S32(4),
            IValue::S32(0),
            IValue::S32(2)
        ])]
    );

    let result2 = faas
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!({ "arg": [] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(
        result2,
        vec![IValue::Array(vec![
            IValue::S32(0),
            IValue::S32(1),
            IValue::S32(2),
            IValue::S32(3),
            IValue::S32(4),
            IValue::S32(0),
            IValue::S32(2)
        ])]
    );

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::S32(1),
            IValue::S32(0),
            IValue::S32(1),
            IValue::S32(2),
            IValue::S32(3),
            IValue::S32(4),
            IValue::S32(0),
            IValue::S32(2)
        ])]
    );
}

#[test]
pub fn i64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arrays_passing_pure", "i64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arrays_passing_pure", "i64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "i64_type",
            json!({ "arg": [1] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::S64(1),
            IValue::S64(0),
            IValue::S64(1),
            IValue::S64(2),
            IValue::S64(3),
            IValue::S64(4),
            IValue::S64(1),
            IValue::S64(1)
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "i64_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::S64(1),
            IValue::S64(0),
            IValue::S64(1),
            IValue::S64(2),
            IValue::S64(3),
            IValue::S64(4),
            IValue::S64(1),
            IValue::S64(1)
        ])]
    );
}

#[test]
pub fn u32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arrays_passing_pure", "u32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arrays_passing_pure", "u32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "u32_type",
            json!({ "arg": [1] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u32_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::U32(1),
            IValue::U32(0),
            IValue::U32(13),
            IValue::U32(37),
            IValue::U32(2),
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "u32_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u32_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::U32(1),
            IValue::U32(0),
            IValue::U32(13),
            IValue::U32(37),
            IValue::U32(2),
        ])]
    );
}

#[test]
pub fn u64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arrays_passing_pure", "u64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arrays_passing_pure", "u64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "u64_type",
            json!({ "arg": [1] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u64_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::U64(1),
            IValue::U64(0),
            IValue::U64(1),
            IValue::U64(2),
            IValue::U64(3),
            IValue::U64(4),
            IValue::U64(2),
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "u64_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke u64_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::U64(1),
            IValue::U64(0),
            IValue::U64(1),
            IValue::U64(2),
            IValue::U64(3),
            IValue::U64(4),
            IValue::U64(2),
        ])]
    );
}

#[test]
pub fn f64_type_() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arrays_passing_pure", "f32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arrays_passing_pure", "f32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "f64_type",
            json!({ "arg": [1.0] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::F64(1.0),
            IValue::F64(0.0),
            IValue::F64(13.37),
            IValue::F64(1.0),
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "f64_type",
            json!([[1.0]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::F64(1.0),
            IValue::F64(0.0),
            IValue::F64(13.37),
            IValue::F64(1.0),
        ])]
    );
}

#[test]
#[ignore]
pub fn f64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json("arrays_passing_pure", "f64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json("arrays_passing_pure", "f64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "f64_type",
            json!({ "arg": 1.0 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(result3, vec![IValue::F64(3.0)]);

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "f64_type",
            json!(1.0),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke f64_type: {:?}", e));
    assert_eq!(result4, vec![IValue::F64(3.0)]);
}

#[test]
pub fn string_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arrays_passing_pure",
        "string_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arrays_passing_pure",
        "string_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "string_type",
            json!({ "arg": ["Fluence"] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke string_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::String(String::from("Fluence")),
            IValue::String(String::from("fce")),
            IValue::String(String::from("from effector")),
            IValue::String(String::from("test")),
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "string_type",
            json!([["Fluence"]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke string_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::String(String::from("Fluence")),
            IValue::String(String::from("fce")),
            IValue::String(String::from("from effector")),
            IValue::String(String::from("test")),
        ])]
    );
}

#[test]
pub fn byte_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arrays_passing_pure",
        "byte_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arrays_passing_pure",
        "byte_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "byte_type",
            json!({ "arg": [0x13, 0x37] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::U8(0x13),
            IValue::U8(0x37),
            IValue::U8(0),
            IValue::U8(1),
            IValue::U8(2),
        ])]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "byte_type",
            json!([[0x13, 0x37]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::U8(0x13),
            IValue::U8(0x37),
            IValue::U8(0),
            IValue::U8(1),
            IValue::U8(2),
        ])]
    );
}

#[test]
pub fn inner_arrays_1_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_1",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_1",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "inner_arrays_1",
            json!({ "arg": [[[[0x13, 0x37]]]] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::U8(0x13),
                IValue::U8(0x37),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![IValue::U8(
                0
            ),])])]),
            IValue::Array(vec![]),
            IValue::Array(vec![IValue::Array(vec![])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::U8(1),
                IValue::U8(2),
                IValue::U8(3),
                IValue::U8(4),
                IValue::U8(5),
                IValue::U8(6),
                IValue::U8(7),
                IValue::U8(8),
                IValue::U8(9),
                IValue::U8(10),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![IValue::U8(
                2
            ),])])]),
        ]),]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "inner_arrays_1",
            json!([[[[[0x13, 0x37]]]]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::U8(0x13),
                IValue::U8(0x37),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![IValue::U8(
                0
            ),])])]),
            IValue::Array(vec![]),
            IValue::Array(vec![IValue::Array(vec![])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::U8(1),
                IValue::U8(2),
                IValue::U8(3),
                IValue::U8(4),
                IValue::U8(5),
                IValue::U8(6),
                IValue::U8(7),
                IValue::U8(8),
                IValue::U8(9),
                IValue::U8(10),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![IValue::U8(
                2
            ),])])]),
        ]),]
    );
}

#[test]
pub fn inner_arrays_2_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_2",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_2",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "inner_arrays_2",
            json!({ "arg": [[[[[0, [[1]]]]]]] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(1)])])
                ]).unwrap())
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(1)])])
                ]).unwrap()),
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![]),
                ]).unwrap())
            ])])]),
            IValue::Array(vec![]),
            IValue::Array(vec![IValue::Array(vec![])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![
                        IValue::U8(1),
                        IValue::U8(2),
                        IValue::U8(3),
                        IValue::U8(4),
                    ])])
                ]).unwrap()),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(1),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(2)])])
                ]).unwrap()),
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![])
                ]).unwrap())
            ])])]),
        ]),]
    );

    let result4 = faas
        .call_with_json(
            "arrays_passing_pure",
            "inner_arrays_2",
            json!([[[[[{"field_0": 0, "field_1": [[1]]}]]]]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bytearray_type: {:?}", e));
    assert_eq!(
        result4,
        vec![IValue::Array(vec![
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(1)])])
                ]).unwrap())
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(1)])])
                ]).unwrap()),
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![]),
                ]).unwrap())
            ])])]),
            IValue::Array(vec![]),
            IValue::Array(vec![IValue::Array(vec![])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![IValue::Array(vec![
                        IValue::U8(1),
                        IValue::U8(2),
                        IValue::U8(3),
                        IValue::U8(4),
                    ])])
                ]).unwrap()),
            ])])]),
            IValue::Array(vec![IValue::Array(vec![IValue::Array(vec![
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(1),
                    IValue::Array(vec![IValue::Array(vec![IValue::U8(2)])])
                ]).unwrap()),
                IValue::Record(fce::vec1::Vec1::new(vec![
                    IValue::S32(0),
                    IValue::Array(vec![])
                ]).unwrap())
            ])])]),
        ]),
        ]
    );
}

#[test]
#[ignore]
pub fn bool_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arrays_passing_pure",
        "bool_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arrays_passing_pure",
        "bool_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "bool_type",
            json!({ "arg": 0 }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke bool_type: {:?}", e));
    assert_eq!(result3, vec![IValue::I32(1)]);

    let result4 = faas
        .call_with_json("arrays_passing_pure", "bool_type", json!(0), <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke bool_type: {:?}", e));
    assert_eq!(result4, vec![IValue::I32(1)]);
}

#[test]
pub fn empty_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas
        .call_with_json(
            "arrays_passing_pure",
            "empty_type",
            json!({}),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(
        result1,
        vec![IValue::Array(vec![IValue::String(String::from(
            "from effector"
        ))])]
    );

    let result2 = faas
        .call_with_json(
            "arrays_passing_pure",
            "empty_type",
            json!([]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(
        result2,
        vec![IValue::Array(vec![IValue::String(String::from(
            "from effector"
        ))])]
    );

    let result3 = faas
        .call_with_json(
            "arrays_passing_pure",
            "empty_type",
            json!([]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke empty_type: {:?}", e));
    assert_eq!(
        result3,
        vec![IValue::Array(vec![IValue::String(String::from(
            "from effector"
        ))])]
    );

    let result4 = faas.call_with_json(
        "arrays_passing_pure",
        "empty_type",
        json!([1]),
        <_>::default(),
    );
    assert!(result4.is_err());
}
