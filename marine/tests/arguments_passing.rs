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

use marine::Marine;
use marine::IType;

use pretty_assertions::assert_eq;
use once_cell::sync::Lazy;
use serde_json::json;

use std::sync::Arc;

static ARG_CONFIG: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/arguments_passing/Config.toml")
        .expect("toml faas config should be created")
});

const MODULE_NAME: &str = "arguments_passing_pure";

#[cfg(feature = "wasmer")]
type MarineImpl = Marine<marine_wasmer_backend::WasmerBackend>;

#[cfg(feature = "wasmtime")]
type MarineImpl = Marine<marine_wasmtime_backend::WasmtimeWasmBackend>;

#[test]
pub fn get_interfaces() {
    use std::collections::HashSet;

    let faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let interface = faas.get_interface();

    let string_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::String,
    }];
    let string_type_outputs = vec![IType::String];

    let string_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("string_type")),
        arguments: Arc::new(string_type_arguments.clone()),
        outputs: Arc::new(string_type_outputs.clone()),
    };

    let string_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("string_ref_type")),
        arguments: Arc::new(string_type_arguments),
        outputs: Arc::new(string_type_outputs),
    };

    let str_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::String,
    }];
    let str_type_outputs = vec![IType::String];

    let str_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("str_type")),
        arguments: Arc::new(str_type_arguments),
        outputs: Arc::new(str_type_outputs),
    };

    let bytearray_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::ByteArray,
    }];
    let bytearray_type_outputs = vec![IType::ByteArray];

    let bytearray_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("bytearray_type")),
        arguments: Arc::new(bytearray_type_arguments.clone()),
        outputs: Arc::new(bytearray_type_outputs.clone()),
    };

    let bytearray_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("bytearray_ref_type")),
        arguments: Arc::new(bytearray_type_arguments),
        outputs: Arc::new(bytearray_type_outputs),
    };

    let i32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S32,
    }];
    let i32_type_outputs = vec![IType::S32];

    let i32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i32_type")),
        arguments: Arc::new(i32_type_arguments.clone()),
        outputs: Arc::new(i32_type_outputs.clone()),
    };

    let i32_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i32_ref_type")),
        arguments: Arc::new(i32_type_arguments),
        outputs: Arc::new(i32_type_outputs),
    };

    let i64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::S64,
    }];

    let i64_type_outputs = vec![IType::S64];

    let i64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i64_type")),
        arguments: Arc::new(i64_type_arguments.clone()),
        outputs: Arc::new(i64_type_outputs.clone()),
    };

    let i64_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i64_ref_type")),
        arguments: Arc::new(i64_type_arguments),
        outputs: Arc::new(i64_type_outputs),
    };

    let u32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U32,
    }];
    let u32_type_outputs = vec![IType::U32];

    let u32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u32_type")),
        arguments: Arc::new(u32_type_arguments.clone()),
        outputs: Arc::new(u32_type_outputs.clone()),
    };

    let u32_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u32_ref_type")),
        arguments: Arc::new(u32_type_arguments),
        outputs: Arc::new(u32_type_outputs),
    };

    let u64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::U64,
    }];
    let u64_type_outputs = vec![IType::U64];

    let u64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u64_type")),
        arguments: Arc::new(u64_type_arguments.clone()),
        outputs: Arc::new(u64_type_outputs.clone()),
    };

    let u64_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u64_ref_type")),
        arguments: Arc::new(u64_type_arguments),
        outputs: Arc::new(u64_type_outputs),
    };

    let f32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F32,
    }];
    let f32_type_outputs = vec![IType::F32];

    let f32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f32_type")),
        arguments: Arc::new(f32_type_arguments.clone()),
        outputs: Arc::new(f32_type_outputs.clone()),
    };

    let f32_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f32_ref_type")),
        arguments: Arc::new(f32_type_arguments),
        outputs: Arc::new(f32_type_outputs),
    };

    let f64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::F64,
    }];
    let f64_type_outputs = vec![IType::F64];

    let f64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f64_type")),
        arguments: Arc::new(f64_type_arguments.clone()),
        outputs: Arc::new(f64_type_outputs.clone()),
    };

    let f64_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f64_ref_type")),
        arguments: Arc::new(f64_type_arguments),
        outputs: Arc::new(f64_type_outputs),
    };

    let empty_type_arguments = vec![];
    let empty_type_outputs = vec![IType::String];

    let empty_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("empty_type")),
        arguments: Arc::new(empty_type_arguments),
        outputs: Arc::new(empty_type_outputs),
    };

    let bool_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Boolean,
    }];
    let bool_type_outputs = vec![IType::Boolean];

    let bool_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("bool_type")),
        arguments: Arc::new(bool_type_arguments.clone()),
        outputs: Arc::new(bool_type_outputs.clone()),
    };

    let bool_ref_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("bool_ref_type")),
        arguments: Arc::new(bool_type_arguments),
        outputs: Arc::new(bool_type_outputs),
    };

    let all_types_arguments = vec![
        marine::IFunctionArg {
            name: String::from("arg_0"),
            ty: IType::S8,
        },
        marine::IFunctionArg {
            name: String::from("arg_1"),
            ty: IType::S16,
        },
        marine::IFunctionArg {
            name: String::from("arg_2"),
            ty: IType::S32,
        },
        marine::IFunctionArg {
            name: String::from("arg_3"),
            ty: IType::S64,
        },
        marine::IFunctionArg {
            name: String::from("arg_4"),
            ty: IType::U8,
        },
        marine::IFunctionArg {
            name: String::from("arg_5"),
            ty: IType::U16,
        },
        marine::IFunctionArg {
            name: String::from("arg_6"),
            ty: IType::U32,
        },
        marine::IFunctionArg {
            name: String::from("arg_7"),
            ty: IType::U64,
        },
        marine::IFunctionArg {
            name: String::from("arg_8"),
            ty: IType::F32,
        },
        marine::IFunctionArg {
            name: String::from("arg_9"),
            ty: IType::F64,
        },
        marine::IFunctionArg {
            name: String::from("arg_10"),
            ty: IType::String,
        },
        marine::IFunctionArg {
            name: String::from("arg_11"),
            ty: IType::ByteArray,
        },
    ];
    let all_types_outputs = vec![IType::ByteArray];

    let all_types_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("all_types")),
        arguments: Arc::new(all_types_arguments.clone()),
        outputs: Arc::new(all_types_outputs.clone()),
    };

    let all_ref_types_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("all_ref_types")),
        arguments: Arc::new(all_types_arguments),
        outputs: Arc::new(all_types_outputs),
    };

    let functions = vec![
        string_type_sign,
        string_ref_type_sign,
        str_type_sign,
        bytearray_type_sign,
        bytearray_ref_type_sign,
        i32_type_sign,
        i32_ref_type_sign,
        i64_type_sign,
        i64_ref_type_sign,
        u32_type_sign,
        u32_ref_type_sign,
        u64_type_sign,
        u64_ref_type_sign,
        f32_type_sign,
        f32_ref_type_sign,
        f64_type_sign,
        f64_ref_type_sign,
        empty_type_sign,
        bool_type_sign,
        bool_ref_type_sign,
        all_types_sign,
        all_ref_types_sign,
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
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!([
            0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
            0, 65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101,
            19, 55, 0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0,
            0, 0, 0, 0, 65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110,
            99, 101, 19, 55
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
        let result3 = call_faas!(faas, MODULE_NAME, func_name, faas_arg);
        assert_eq!(result3, expected_result);

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
        let result4 = call_faas!(faas, MODULE_NAME, func_name, faas_arg);
        assert_eq!(result4, expected_result);
    };

    test("all_types");
    test("all_ref_types");
}

#[test]
pub fn i32_type() {
    let test = |func_name: &str| {
        let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
            .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1));
        assert_eq!(result4, expected_result);

        let result5 = call_faas!(faas, MODULE_NAME, func_name, json!([[1]]));
        assert_eq!(result5, expected_result);

        let value = i32::MAX - 2;
        let result6 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result6, value + 2);

        let value = i32::MIN;
        let result7 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result7, value + 2);
    };

    test("i32_type");
    test("i32_ref_type");
}

#[test]
pub fn i64_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1));
        assert_eq!(result4, expected_result);

        let result5 = call_faas!(faas, MODULE_NAME, func_name, json!([1]));
        assert_eq!(result5, expected_result);

        let value = i64::MAX - 2;
        let result6 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result6, value + 2);

        let value = i64::MIN;
        let result7 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result7, value + 2);
    };

    test("i64_type");
    test("i64_ref_type");
}

#[test]
pub fn u32_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1));
        assert_eq!(result4, expected_result);
    };

    test("u32_type");
    test("u32_ref_type");
}

#[test]
pub fn u64_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1));
        assert_eq!(result4, expected_result);
    };

    test("u64_type");
    test("u64_ref_type");
}

#[test]
pub fn f32_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3.0);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1.0 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1.0));
        assert_eq!(result4, expected_result);

        let value = f32::MAX - 2.0;
        let result5 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result5, value + 2.0);

        let value = f32::MIN;
        let result6 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result6, value + 2.0);
    };

    test("f32_type");
    test("f32_ref_type");
}

#[test]
pub fn f64_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(3.0);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": 1.0 }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(1.0));
        assert_eq!(result4, expected_result);

        let value = f64::MAX - 2.0;
        let result5 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result5, value + 2.0);

        let value = f64::MIN;
        let result6 = call_faas!(faas, MODULE_NAME, func_name, json!(value));
        assert_eq!(result6, value + 2.0);
    };

    test("f64_type");
    test("f64_ref_type");
}

#[test]
pub fn string_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!("Fluence_Fluence_Fluence_Fluence");
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": "Fluence" }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!("Fluence"));
        assert_eq!(result4, expected_result);
    };

    test("string_type");
    test("string_ref_type");
}

#[test]
pub fn str_type() {
    const FUNC_NAME: &'static str = "str_type";

    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let result1 = faas.call_with_json(MODULE_NAME, FUNC_NAME, json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 = faas.call_with_json(MODULE_NAME, FUNC_NAME, json!([]), <_>::default());
    assert!(result2.is_err());

    let expected_result = json!("Fluence_Fluence_Fluence_Fluence");
    let result3 = call_faas!(faas, MODULE_NAME, FUNC_NAME, json!({ "arg": "Fluence" }));
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(faas, MODULE_NAME, FUNC_NAME, json!("Fluence"));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn bytearray_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!([0x13, 0x37, 1, 1]);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": [0x13, 0x37] }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!([[0x13, 0x37]]));
        assert_eq!(result4, expected_result);

        let result5 = call_faas!(faas, MODULE_NAME, func_name, json!([[0x13]]));
        assert_eq!(result5, json!([0x13, 1, 1]));
    };

    test("bytearray_type");
    test("bytearray_ref_type");
}

#[test]
pub fn bool_type() {
    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let mut test = |func_name: &str| {
        let result1 = faas.call_with_json(MODULE_NAME, func_name, json!({}), <_>::default());
        assert!(result1.is_err());

        let result2 = faas.call_with_json(MODULE_NAME, func_name, json!([]), <_>::default());
        assert!(result2.is_err());

        let expected_result = json!(true);
        let result3 = call_faas!(faas, MODULE_NAME, func_name, json!({ "arg": false }));
        assert_eq!(result3, expected_result);

        let result4 = call_faas!(faas, MODULE_NAME, func_name, json!(false));
        assert_eq!(result4, expected_result);
    };

    test("bool_type");
    test("bool_ref_type");
}

#[test]
pub fn empty_type() {
    const FUNC_NAME: &'static str = "empty_type";

    let mut faas = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    let expected_result = json!("success");
    let result1 = call_faas!(faas, MODULE_NAME, FUNC_NAME, json!({}));
    assert_eq!(result1, expected_result);

    let result2 = call_faas!(faas, MODULE_NAME, FUNC_NAME, json!([]));
    assert_eq!(result2, expected_result);

    let result3 = call_faas!(faas, MODULE_NAME, FUNC_NAME, json!([]));
    assert_eq!(result3, expected_result);

    let result4 = faas.call_with_json(MODULE_NAME, FUNC_NAME, json!([1]), <_>::default());
    assert!(result4.is_err());
}
