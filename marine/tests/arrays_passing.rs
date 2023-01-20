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

use marine::DefaultMarine;
use marine::IType;

use once_cell::sync::Lazy;
use serde_json::json;

use std::sync::Arc;

static ARG_CONFIG: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/arrays_passing/Config.toml")
        .expect("toml marine config should be created")
});

#[test]
pub fn get_interfaces() {
    use std::collections::HashSet;

    let marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let interface = marine.get_interface();

    let byte_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::ByteArray,
    }];
    let byte_type_outputs = vec![IType::ByteArray];

    let byte_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("byte_type")),
        arguments: Arc::new(byte_type_arguments),
        outputs: Arc::new(byte_type_outputs),
    };

    let inner_arrays_1_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::Array(Box::new(IType::Array(Box::new(
            IType::ByteArray,
        )))))),
    }];
    let inner_arrays_1_outputs = vec![IType::Array(Box::new(IType::Array(Box::new(
        IType::Array(Box::new(IType::ByteArray)),
    ))))];

    let inner_arrays_1_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("inner_arrays_1")),
        arguments: Arc::new(inner_arrays_1_arguments),
        outputs: Arc::new(inner_arrays_1_outputs),
    };

    // save it until record will be refactored in the future
    /*
    let inner_arrays_2_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::Array(Box::new(IType::Array(Box::new(
            IType::Array(Box::new(IType::Record(6))),
        )))))),
    }];
    let inner_arrays_2_outputs = vec![IType::Array(Box::new(IType::Array(Box::new(
        IType::Array(Box::new(IType::Array(Box::new(IType::Record(6))))),
    ))))];

    let inner_arrays_2_sign = marine::MarineFunctionSignature {
        name: "inner_arrays_1",
        arguments: &inner_arrays_2_arguments,
        outputs: &inner_arrays_2_outputs,
    };
     */

    let string_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::String)),
    }];
    let string_type_outputs = vec![IType::Array(Box::new(IType::String))];

    let string_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("string_type")),
        arguments: Arc::new(string_type_arguments),
        outputs: Arc::new(string_type_outputs),
    };

    let i32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::S32)),
    }];
    let i32_type_outputs = vec![IType::Array(Box::new(IType::S32))];

    let i32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i32_type")),
        arguments: Arc::new(i32_type_arguments),
        outputs: Arc::new(i32_type_outputs),
    };

    let i64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::S64)),
    }];

    let i64_type_outputs = vec![IType::Array(Box::new(IType::S64))];

    let i64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("i64_type")),
        arguments: Arc::new(i64_type_arguments),
        outputs: Arc::new(i64_type_outputs),
    };

    let u32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U32)),
    }];
    let u32_type_outputs = vec![IType::Array(Box::new(IType::U32))];

    let u32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u32_type")),
        arguments: Arc::new(u32_type_arguments),
        outputs: Arc::new(u32_type_outputs),
    };

    let u64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::U64)),
    }];
    let u64_type_outputs = vec![IType::Array(Box::new(IType::U64))];

    let u64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("u64_type")),
        arguments: Arc::new(u64_type_arguments),
        outputs: Arc::new(u64_type_outputs),
    };

    let f32_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::F32)),
    }];
    let f32_type_outputs = vec![IType::Array(Box::new(IType::F32))];

    let f32_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f32_type")),
        arguments: Arc::new(f32_type_arguments),
        outputs: Arc::new(f32_type_outputs),
    };

    let f64_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::F64)),
    }];
    let f64_type_outputs = vec![IType::Array(Box::new(IType::F64))];

    let f64_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("f64_type")),
        arguments: Arc::new(f64_type_arguments),
        outputs: Arc::new(f64_type_outputs),
    };

    let empty_type_arguments = vec![];
    let empty_type_outputs = vec![IType::Array(Box::new(IType::String))];

    let empty_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("empty_type")),
        arguments: Arc::new(empty_type_arguments),
        outputs: Arc::new(empty_type_outputs),
    };

    let bool_type_arguments = vec![marine::IFunctionArg {
        name: String::from("arg"),
        ty: IType::Array(Box::new(IType::Boolean)),
    }];
    let bool_type_outputs = vec![IType::Array(Box::new(IType::Boolean))];

    let bool_type_sign = marine::MarineFunctionSignature {
        name: Arc::new(String::from("bool_type")),
        arguments: Arc::new(bool_type_arguments),
        outputs: Arc::new(bool_type_outputs),
    };

    let functions = vec![
        byte_type_sign,
        inner_arrays_1_sign,
        string_type_sign,
        bool_type_sign,
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
        .filter(|f| f.name.as_str() != "inner_arrays_2")
        .collect();
    let effector_module_functions: HashSet<_> = effector_module_interface
        .function_signatures
        .iter()
        .filter(|f| f.name.as_str() != "inner_arrays_2")
        .collect();

    let functions: HashSet<_> = functions.iter().collect();

    assert_eq!(pure_module_functions, functions);
    assert_eq!(effector_module_functions, functions);
}

#[test]
pub fn i32_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let expected_result = json!([0, 1, 2, 3, 4, 0, 2]);

    let result1 = marine
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!([[]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(result1, expected_result);

    let result2 = marine
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!({ "arg": [] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(result2, expected_result);

    let expected_result = json!([1, 0, 1, 2, 3, 4, 0, 2]);
    let result3 = marine
        .call_with_json(
            "arrays_passing_pure",
            "i32_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i32_type: {:?}", e));
    assert_eq!(result3, expected_result);
}

#[test]
pub fn i64_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 =
        marine.call_with_json("arrays_passing_pure", "i64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 =
        marine.call_with_json("arrays_passing_pure", "i64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let expected_result = json!([1, 0, 1, 2, 3, 4, 1, 1]);

    let result3 = marine
        .call_with_json(
            "arrays_passing_pure",
            "i64_type",
            json!({ "arg": [1] }),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(result3, expected_result);

    let result4 = marine
        .call_with_json(
            "arrays_passing_pure",
            "i64_type",
            json!([[1]]),
            <_>::default(),
        )
        .unwrap_or_else(|e| panic!("can't invoke i64_type: {:?}", e));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn u32_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 =
        marine.call_with_json("arrays_passing_pure", "u32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 =
        marine.call_with_json("arrays_passing_pure", "u32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let expected_result = json!([1, 0, 13, 37, 2]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "u32_type",
        json!({ "arg": [1] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(marine, "arrays_passing_pure", "u32_type", json!([[1]]));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn u64_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 =
        marine.call_with_json("arrays_passing_pure", "u64_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 =
        marine.call_with_json("arrays_passing_pure", "u64_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let expected_result = json!([1, 0, 1, 2, 3, 4, 2]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "u64_type",
        json!({ "arg": [1] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(marine, "arrays_passing_pure", "u64_type", json!([[1]]));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn f64_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 =
        marine.call_with_json("arrays_passing_pure", "f32_type", json!({}), <_>::default());
    assert!(result1.is_err());

    let result2 =
        marine.call_with_json("arrays_passing_pure", "f32_type", json!([]), <_>::default());
    assert!(result2.is_err());

    let expected_result = json!([1.0, 0.0, 13.37, 1.0]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "f64_type",
        json!({ "arg": [1.0] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(marine, "arrays_passing_pure", "f64_type", json!([[1.0]]));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn string_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 = marine.call_with_json(
        "arrays_passing_pure",
        "string_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = marine.call_with_json(
        "arrays_passing_pure",
        "string_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let expected_result = json!(["Fluence", "marine", "from effector", "test"]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "string_type",
        json!({ "arg": ["Fluence"] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(
        marine,
        "arrays_passing_pure",
        "string_type",
        json!([["Fluence"]])
    );
    assert_eq!(result4, expected_result);
}

#[test]
pub fn byte_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 = marine.call_with_json(
        "arrays_passing_pure",
        "byte_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = marine.call_with_json(
        "arrays_passing_pure",
        "byte_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let expected_result = json!([0x13, 0x37, 0, 1, 2]);
    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "byte_type",
        json!({ "arg": [0x13, 0x37] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(
        marine,
        "arrays_passing_pure",
        "byte_type",
        json!([[0x13, 0x37]])
    );
    assert_eq!(result4, expected_result);
}

#[test]
pub fn inner_arrays_1_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 = marine.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_1",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = marine.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_1",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let expected_result = json!([
        [[[0x13, 0x37]]],
        [[[0]]],
        [],
        [[]],
        [[[]]],
        [[[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]],
        [[[2]]]
    ]);
    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "inner_arrays_1",
        json!({ "arg": [[[[0x13, 0x37]]]] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(
        marine,
        "arrays_passing_pure",
        "inner_arrays_1",
        json!([[[[[0x13, 0x37]]]]])
    );
    assert_eq!(result4, expected_result);
}

#[test]
pub fn inner_arrays_2_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 = marine.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_2",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = marine.call_with_json(
        "arrays_passing_pure",
        "inner_arrays_2",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let expected_result = json!([
    [[[{
        "field_0": 0,
        "field_1": [[1]]
    }]]],
    [[[
    {
        "field_0": 0,
        "field_1": [[1]]
    },
    {
        "field_0": 0,
        "field_1": []
    },
    ]]],
    [],
    [[]],
    [[[]]],
    [[[{
        "field_0": 0,
        "field_1": [[1,2,3,4]]
    }]]],
    [[[
    {
        "field_0": 1,
        "field_1": [[2]]
    },
    {
        "field_0": 0,
        "field_1": []
    },
    ]]],
     ]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "inner_arrays_2",
        json!({ "arg": [[[[[0, [[1]]]]]]] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(
        marine,
        "arrays_passing_pure",
        "inner_arrays_2",
        json!([[[[[{"field_0": 0, "field_1": [[1]]}]]]]])
    );
    assert_eq!(result4, expected_result);
}

#[test]
pub fn bool_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let result1 = marine.call_with_json(
        "arrays_passing_pure",
        "bool_type",
        json!({}),
        <_>::default(),
    );
    assert!(result1.is_err());

    let result2 = marine.call_with_json(
        "arrays_passing_pure",
        "bool_type",
        json!([]),
        <_>::default(),
    );
    assert!(result2.is_err());

    let expected_result = json!([true, true, false, true, false, true]);

    let result3 = call_faas!(
        marine,
        "arrays_passing_pure",
        "bool_type",
        json!({ "arg": [false] })
    );
    assert_eq!(result3, expected_result);

    let result4 = call_faas!(marine, "arrays_passing_pure", "bool_type", json!([[false]]));
    assert_eq!(result4, expected_result);
}

#[test]
pub fn empty_type() {
    let mut marine = MarineImpl::with_raw_config(ARG_CONFIG.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence Marine instance: {}", e));

    let expected_result = json!(["from effector"]);
    let result1 = call_faas!(marine, "arrays_passing_pure", "empty_type", json!({}));
    assert_eq!(result1, expected_result);

    let result2 = call_faas!(marine, "arrays_passing_pure", "empty_type", json!([]));
    assert_eq!(result2, expected_result);

    let result3 = call_faas!(marine, "arrays_passing_pure", "empty_type", json!([]));
    assert_eq!(result3, expected_result);

    let result4 = marine.call_with_json(
        "arrays_passing_pure",
        "empty_type",
        json!([1]),
        <_>::default(),
    );
    assert!(result4.is_err());
}
