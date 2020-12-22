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

mod utils;

use fluence_faas::FluenceFaaS;
use fluence_faas::IType;

use pretty_assertions::assert_eq;
use once_cell::sync::Lazy;
use serde_json::json;

use std::rc::Rc;

static ARG_CONFIG: Lazy<fluence_faas::TomlFaaSConfig> = Lazy::new(|| {
    let mut arguments_passing_config =
        fluence_faas::TomlFaaSConfig::load("./tests/wasm_tests/arguments_passing/Config.toml")
            .expect("toml faas config should be created");

    arguments_passing_config.modules_dir = Some(String::from(
        "./tests/wasm_tests/arguments_passing/artifacts",
    ));

    arguments_passing_config
});

#[test]
pub fn get_interfaces() {
    use std::collections::HashSet;

    let faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let interface = faas.get_interface();

    let string_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::String,
    }];
    let string_type_outputs = vec![IType::String];

    let string_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("string_type")),
        arguments: Rc::new(string_type_arguments),
        outputs: Rc::new(string_type_outputs),
    };

    let bytearray_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U8)),
    }];
    let bytearray_type_outputs = vec![IType::Array(Box::new(IType::U8))];

    let bytearray_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("bytearray_type")),
        arguments: Rc::new(bytearray_type_arguments),
        outputs: Rc::new(bytearray_type_outputs),
    };

    let i32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S32,
    }];
    let i32_type_outputs = vec![IType::S32];

    let i32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("i32_type")),
        arguments: Rc::new(i32_type_arguments),
        outputs: Rc::new(i32_type_outputs),
    };

    let i64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S64,
    }];

    let i64_type_outputs = vec![IType::S64];

    let i64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("i64_type")),
        arguments: Rc::new(i64_type_arguments),
        outputs: Rc::new(i64_type_outputs),
    };

    let u32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U32,
    }];
    let u32_type_outputs = vec![IType::U32];

    let u32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("u32_type")),
        arguments: Rc::new(u32_type_arguments),
        outputs: Rc::new(u32_type_outputs),
    };

    let u64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U64,
    }];
    let u64_type_outputs = vec![IType::U64];

    let u64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("u64_type")),
        arguments: Rc::new(u64_type_arguments),
        outputs: Rc::new(u64_type_outputs),
    };

    let f32_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F32,
    }];
    let f32_type_outputs = vec![IType::F32];

    let f32_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("f32_type")),
        arguments: Rc::new(f32_type_arguments),
        outputs: Rc::new(f32_type_outputs),
    };

    let f64_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F64,
    }];
    let f64_type_outputs = vec![IType::F64];

    let f64_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("f64_type")),
        arguments: Rc::new(f64_type_arguments),
        outputs: Rc::new(f64_type_outputs),
    };

    let empty_type_arguments = vec![];
    let empty_type_outputs = vec![IType::String];

    let empty_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("empty_type")),
        arguments: Rc::new(empty_type_arguments),
        outputs: Rc::new(empty_type_outputs),
    };

    let bool_type_arguments = vec![fluence_faas::IFunctionArg {
        name: String::from("arg"),
        ty: IType::I32,
    }];
    let bool_type_outputs = vec![IType::I32];

    let bool_type_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("bool_type")),
        arguments: Rc::new(bool_type_arguments),
        outputs: Rc::new(bool_type_outputs),
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
    let all_types_outputs = vec![IType::Array(Box::new(IType::U8))];

    let all_types_sign = fluence_faas::FaaSFunctionSignature {
        name: Rc::new(String::from("all_types")),
        arguments: Rc::new(all_types_arguments),
        outputs: Rc::new(all_types_outputs),
    };

    let functions = vec![
        string_type_sign,
        bytearray_type_sign,
        i32_type_sign,
        i64_type_sign,
        u32_type_sign,
        u64_type_sign,
        f32_type_sign,
        f64_type_sign,
        empty_type_sign,
        bool_type_sign,
        all_types_sign,
    ];

    let pure_module_name = "arguments_passing_pure";
    let effector_module_name = "arguments_passing_effector";

    let pure_module_interface = interface
        .modules
        .get(pure_module_name)
        .expect(&format!("{} should present in interface", pure_module_name));
    let effector_module_interface = interface
        .modules
        .get(effector_module_name)
        .expect(&format!("{} should present in interface", pure_module_name));

    assert!(pure_module_interface.record_types.is_empty());
    assert!(effector_module_interface.record_types.is_empty());

    let pure_module_functions: HashSet<_> =
        pure_module_interface.function_signatures.iter().collect();
    let effector_module_functions: HashSet<_> = effector_module_interface
        .function_signatures
        .iter()
        .collect();

    let functions: HashSet<_> = functions.iter().collect();

    assert_eq!(pure_module_functions, functions);
    assert_eq!(effector_module_functions, functions);
}

#[test]
pub fn all_types() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "all_types",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "all_types",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!([
        0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0,
        65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55,
        0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0,
        65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101, 19, 55
    ]);

    let faas_arg = json!({
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
    });
    let result3 = call_faas!(faas, "arguments_passing_pure", "all_types", faas_arg);
    assert_eq!(result3, right_result);

    let faas_arg = json!([
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8.1,
        9.1,
        "fluence",
        vec![0x13, 0x37]
    ]);
    let result4 = call_faas!(faas, "arguments_passing_pure", "all_types", faas_arg);
    assert_eq!(result4, right_result);
}

#[test]
pub fn i32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "i32_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "i32_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "i32_type",
        json!({ "arg": 1 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "i32_type", json!(1));
    assert_eq!(result4, right_result);

    let result5 = call_faas!(faas, "arguments_passing_pure", "i32_type", json!([1]));
    assert_eq!(result5, right_result);
}

#[test]
pub fn i64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "i64_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "i64_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "i64_type",
        json!({ "arg": 1 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "i64_type", json!(1));
    assert_eq!(result4, right_result);

    let result5 = call_faas!(faas, "arguments_passing_pure", "i64_type", json!([1]));
    assert_eq!(result5, right_result);
}

#[test]
pub fn u32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "u32_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "u32_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "u32_type",
        json!({ "arg": 1 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "u32_type", json!(1));
    assert_eq!(result4, right_result);
}

#[test]
pub fn u64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "u64_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "u64_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "u64_type",
        json!({ "arg": 1 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "u64_type", json!(1));
    assert_eq!(result4, right_result);
}

#[test]
pub fn f32_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "f32_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "f32_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3.0);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "f32_type",
        json!({ "arg": 1.0 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "f32_type", json!(1.0));
    assert_eq!(result4, right_result);
}

#[test]
pub fn f64_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "f64_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "f64_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(3.0);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "f64_type",
        json!({ "arg": 1.0 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "f64_type", json!(1.0));
    assert_eq!(result4, right_result);
}

#[test]
pub fn string_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "string_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "string_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!("Fluence_Fluence_Fluence_Fluence");
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "string_type",
        json!({ "arg": "Fluence" })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(
        faas,
        "arguments_passing_pure",
        "string_type",
        json!("Fluence")
    );
    assert_eq!(result4, right_result);
}

#[test]
pub fn bytearray_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "bytearray_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "bytearray_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!([0x13, 0x37, 1, 1]);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "bytearray_type",
        json!({ "arg": [0x13, 0x37] })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(
        faas,
        "arguments_passing_pure",
        "bytearray_type",
        json!([[0x13, 0x37]])
    );
    assert_eq!(result4, right_result);
}

#[test]
pub fn bool_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result1 = faas.call_with_json(
        "arguments_passing_pure",
        "bool_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = faas.call_with_json(
        "arguments_passing_pure",
        "bool_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let right_result = json!(1);
    let result3 = call_faas!(
        faas,
        "arguments_passing_pure",
        "bool_type",
        json!({ "arg": 0 })
    );
    assert_eq!(result3, right_result);

    let result4 = call_faas!(faas, "arguments_passing_pure", "bool_type", json!(0));
    assert_eq!(result4, right_result);
}

#[test]
pub fn empty_type() {
    let mut faas = FluenceFaaS::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let right_result = json!("success");
    let result1 = call_faas!(faas, "arguments_passing_pure", "empty_type", json!({}));
    assert_eq!(result1, right_result);

    let result2 = call_faas!(faas, "arguments_passing_pure", "empty_type", json!([]));
    assert_eq!(result2, right_result);

    let result3 = call_faas!(faas, "arguments_passing_pure", "empty_type", json!([]));
    assert_eq!(result3, right_result);

    let result4 = faas.call_with_json(
        "arguments_passing_pure",
        "empty_type",
        json!([1]),
        <_>::default(),
    );
    assert!(result4.is_err());
}
