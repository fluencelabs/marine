#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use fluence::fce;
pub fn main() {}
pub fn all_types(
    arg_0: i8,
    arg_1: i16,
    arg_2: i32,
    arg_3: i64,
    arg_4: u8,
    arg_5: u16,
    arg_6: u32,
    arg_7: u64,
    arg_8: f32,
    arg_9: f64,
    arg_10: String,
    arg_11: Vec<u8>,
) -> Vec<u8> {
    let mut result = Vec::new();
    result.push(arg_0 as u8);
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_1));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_2));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_3));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_4));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_5));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_6));
    result.extend(safe_transmute::transmute_one_to_bytes(&arg_7));
    result.extend(&arg_8.to_be_bytes());
    result.extend(&arg_9.to_be_bytes());
    result.extend(arg_10.into_bytes());
    result.extend(arg_11);
    result
}
#[cfg(target_arch = "wasm32")]
#[export_name = "all_types"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_all_types(
    arg_0: i8,
    arg_1: i16,
    arg_2: i32,
    arg_3: i64,
    arg_4: u8,
    arg_5: u16,
    arg_6: u32,
    arg_7: u64,
    arg_8: f32,
    arg_9: f64,
    arg_10: u32,
    arg_11: u32,
    arg_12: u32,
    arg_13: u32,
) {
    let converted_arg_0 = arg_0 as _;
    let converted_arg_1 = arg_1 as _;
    let converted_arg_2 = arg_2 as _;
    let converted_arg_3 = arg_3 as _;
    let converted_arg_4 = arg_4 as _;
    let converted_arg_5 = arg_5 as _;
    let converted_arg_6 = arg_6 as _;
    let converted_arg_7 = arg_7 as _;
    let converted_arg_8 = arg_8 as _;
    let converted_arg_9 = arg_9 as _;
    let converted_arg_10 = String::from_raw_parts(arg_10 as _, arg_11 as _, arg_11 as _);
    unsafe fn __fce_generated_vec_deserializer_12(offset: u32, size: u32) -> Vec<u8> {
        Vec::from_raw_parts(offset as _, size as _, size as _)
    }
    let converted_arg_12 = __fce_generated_vec_deserializer_12(arg_12 as _, arg_13 as _);
    let result = all_types(
        converted_arg_0,
        converted_arg_1,
        converted_arg_2,
        converted_arg_3,
        converted_arg_4,
        converted_arg_5,
        converted_arg_6,
        converted_arg_7,
        converted_arg_8,
        converted_arg_9,
        converted_arg_10,
        converted_arg_12,
    );
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<u8>) -> (u32, u32) {
        (arg.as_ptr() as _, arg.len() as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __fce_generated_vec_serializer(&result);
        fluence::internal::set_result_ptr(serialized_vec_ptr as _);
        fluence::internal::set_result_size(serialized_vec_size as _);
    }
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__all_types"]
pub static __fce_generated_static_global_all_types: [u8; 633usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"all_types\",\"arguments\":[{\"name\":\"arg_0\",\"ty\":{\"I8\":\"ByValue\"}},{\"name\":\"arg_1\",\"ty\":{\"I16\":\"ByValue\"}},{\"name\":\"arg_2\",\"ty\":{\"I32\":\"ByValue\"}},{\"name\":\"arg_3\",\"ty\":{\"I64\":\"ByValue\"}},{\"name\":\"arg_4\",\"ty\":{\"U8\":\"ByValue\"}},{\"name\":\"arg_5\",\"ty\":{\"U16\":\"ByValue\"}},{\"name\":\"arg_6\",\"ty\":{\"U32\":\"ByValue\"}},{\"name\":\"arg_7\",\"ty\":{\"U64\":\"ByValue\"}},{\"name\":\"arg_8\",\"ty\":{\"F32\":\"ByValue\"}},{\"name\":\"arg_9\",\"ty\":{\"F64\":\"ByValue\"}},{\"name\":\"arg_10\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"arg_11\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}],\"output_type\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}}"
};
pub fn all_ref_types(
    arg_0: &i8,
    arg_1: &i16,
    arg_2: &i32,
    arg_3: &i64,
    arg_4: &u8,
    arg_5: &u16,
    arg_6: &u32,
    arg_7: &u64,
    arg_8: &f32,
    arg_9: &f64,
    arg_10: &String,
    arg_11: &Vec<u8>,
) -> Vec<u8> {
    let mut result = Vec::new();
    result.push(*arg_0 as u8);
    result.extend(safe_transmute::transmute_one_to_bytes(arg_1));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_2));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_3));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_4));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_5));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_6));
    result.extend(safe_transmute::transmute_one_to_bytes(arg_7));
    result.extend(&arg_8.to_be_bytes());
    result.extend(&arg_9.to_be_bytes());
    result.extend(arg_10.as_bytes());
    result.extend(arg_11);
    result
}
#[cfg(target_arch = "wasm32")]
#[export_name = "all_ref_types"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_all_ref_types(
    arg_0: i8,
    arg_1: i16,
    arg_2: i32,
    arg_3: i64,
    arg_4: u8,
    arg_5: u16,
    arg_6: u32,
    arg_7: u64,
    arg_8: f32,
    arg_9: f64,
    arg_10: u32,
    arg_11: u32,
    arg_12: u32,
    arg_13: u32,
) {
    let converted_arg_0 = arg_0 as _;
    let converted_arg_1 = arg_1 as _;
    let converted_arg_2 = arg_2 as _;
    let converted_arg_3 = arg_3 as _;
    let converted_arg_4 = arg_4 as _;
    let converted_arg_5 = arg_5 as _;
    let converted_arg_6 = arg_6 as _;
    let converted_arg_7 = arg_7 as _;
    let converted_arg_8 = arg_8 as _;
    let converted_arg_9 = arg_9 as _;
    let converted_arg_10 = String::from_raw_parts(arg_10 as _, arg_11 as _, arg_11 as _);
    unsafe fn __fce_generated_vec_deserializer_12(offset: u32, size: u32) -> Vec<u8> {
        Vec::from_raw_parts(offset as _, size as _, size as _)
    }
    let converted_arg_12 = __fce_generated_vec_deserializer_12(arg_12 as _, arg_13 as _);
    let result = all_ref_types(
        &converted_arg_0,
        &converted_arg_1,
        &converted_arg_2,
        &converted_arg_3,
        &converted_arg_4,
        &converted_arg_5,
        &converted_arg_6,
        &converted_arg_7,
        &converted_arg_8,
        &converted_arg_9,
        &converted_arg_10,
        &converted_arg_12,
    );
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<u8>) -> (u32, u32) {
        (arg.as_ptr() as _, arg.len() as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __fce_generated_vec_serializer(&result);
        fluence::internal::set_result_ptr(serialized_vec_ptr as _);
        fluence::internal::set_result_size(serialized_vec_size as _);
    }
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__all_ref_types"]
pub static __fce_generated_static_global_all_ref_types: [u8; 613usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"all_ref_types\",\"arguments\":[{\"name\":\"arg_0\",\"ty\":{\"I8\":\"ByRef\"}},{\"name\":\"arg_1\",\"ty\":{\"I16\":\"ByRef\"}},{\"name\":\"arg_2\",\"ty\":{\"I32\":\"ByRef\"}},{\"name\":\"arg_3\",\"ty\":{\"I64\":\"ByRef\"}},{\"name\":\"arg_4\",\"ty\":{\"U8\":\"ByRef\"}},{\"name\":\"arg_5\",\"ty\":{\"U16\":\"ByRef\"}},{\"name\":\"arg_6\",\"ty\":{\"U32\":\"ByRef\"}},{\"name\":\"arg_7\",\"ty\":{\"U64\":\"ByRef\"}},{\"name\":\"arg_8\",\"ty\":{\"F32\":\"ByRef\"}},{\"name\":\"arg_9\",\"ty\":{\"F64\":\"ByRef\"}},{\"name\":\"arg_10\",\"ty\":{\"Utf8String\":\"ByRef\"}},{\"name\":\"arg_11\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByRef\"]}}],\"output_type\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}}"
};
pub fn string_type(arg: String) -> String {
    {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", "_"],
            &match (&arg, &arg) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
        res
    }
}
#[cfg(target_arch = "wasm32")]
#[export_name = "string_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_string_type(arg_0: u32, arg_1: u32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = string_type(converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__string_type"]
pub static __fce_generated_static_global_string_type: [u8; 156usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"string_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Utf8String\":\"ByValue\"}}],\"output_type\":{\"Utf8String\":\"ByValue\"}}}"
};
pub fn string_ref_type(arg: &String) -> String {
    {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", "_"],
            &match (&arg, &arg) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
        res
    }
}
#[cfg(target_arch = "wasm32")]
#[export_name = "string_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_string_ref_type(arg_0: u32, arg_1: u32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = string_ref_type(&converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__string_ref_type"]
pub static __fce_generated_static_global_string_ref_type: [u8; 158usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"string_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Utf8String\":\"ByRef\"}}],\"output_type\":{\"Utf8String\":\"ByValue\"}}}"
};
pub fn str_type(arg: &str) -> String {
    {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", "_"],
            &match (&arg, &arg) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
        res
    }
}
#[cfg(target_arch = "wasm32")]
#[export_name = "str_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_str_type(arg_0: u32, arg_1: u32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = str_type(&converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__str_type"]
pub static __fce_generated_static_global_str_type: [u8; 148usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"str_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Utf8Str\":\"ByRef\"}}],\"output_type\":{\"Utf8String\":\"ByValue\"}}}"
};
pub fn bytearray_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg
}
#[cfg(target_arch = "wasm32")]
#[export_name = "bytearray_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_bytearray_type(arg_0: u32, arg_1: u32) {
    unsafe fn __fce_generated_vec_deserializer_0(offset: u32, size: u32) -> Vec<u8> {
        Vec::from_raw_parts(offset as _, size as _, size as _)
    }
    let converted_arg_0 = __fce_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    let result = bytearray_type(converted_arg_0);
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<u8>) -> (u32, u32) {
        (arg.as_ptr() as _, arg.len() as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __fce_generated_vec_serializer(&result);
        fluence::internal::set_result_ptr(serialized_vec_ptr as _);
        fluence::internal::set_result_size(serialized_vec_size as _);
    }
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__bytearray_type"]
pub static __fce_generated_static_global_bytearray_type: [u8; 189usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"bytearray_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}],\"output_type\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}}"
};
pub fn bytearray_ref_type(arg: &mut Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg.clone()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "bytearray_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_bytearray_ref_type(arg_0: u32, arg_1: u32) {
    unsafe fn __fce_generated_vec_deserializer_0(offset: u32, size: u32) -> Vec<u8> {
        Vec::from_raw_parts(offset as _, size as _, size as _)
    }
    let mut converted_arg_0 = __fce_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    let result = bytearray_ref_type(&mut converted_arg_0);
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<u8>) -> (u32, u32) {
        (arg.as_ptr() as _, arg.len() as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __fce_generated_vec_serializer(&result);
        fluence::internal::set_result_ptr(serialized_vec_ptr as _);
        fluence::internal::set_result_size(serialized_vec_size as _);
    }
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__bytearray_ref_type"]
pub static __fce_generated_static_global_bytearray_ref_type: [u8; 194usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"bytearray_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByMutRef\"]}}],\"output_type\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}}"
};
pub fn bool_type(arg: bool) -> bool {
    !arg
}
#[cfg(target_arch = "wasm32")]
#[export_name = "bool_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_bool_type(arg_0: i32) -> i32 {
    let converted_arg_0 = arg_0 != 0;
    let result = bool_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__bool_type"]
pub static __fce_generated_static_global_bool_type: [u8; 148usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"bool_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Boolean\":\"ByValue\"}}],\"output_type\":{\"Boolean\":\"ByValue\"}}}"
};
pub fn bool_ref_type(arg: &bool) -> bool {
    !*arg
}
#[cfg(target_arch = "wasm32")]
#[export_name = "bool_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_bool_ref_type(arg_0: i32) -> i32 {
    let converted_arg_0 = arg_0 != 0;
    let result = bool_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__bool_ref_type"]
pub static __fce_generated_static_global_bool_ref_type: [u8; 150usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"bool_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Boolean\":\"ByRef\"}}],\"output_type\":{\"Boolean\":\"ByValue\"}}}"
};
pub fn f32_type(arg: f32) -> f32 {
    arg + 1.0
}
#[cfg(target_arch = "wasm32")]
#[export_name = "f32_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_f32_type(arg_0: f32) -> f32 {
    let converted_arg_0 = arg_0 as _;
    let result = f32_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__f32_type"]
pub static __fce_generated_static_global_f32_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"f32_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"F32\":\"ByValue\"}}],\"output_type\":{\"F32\":\"ByValue\"}}}"
};
pub fn f32_ref_type(arg: &f32) -> f32 {
    *arg + 1.0
}
#[cfg(target_arch = "wasm32")]
#[export_name = "f32_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_f32_ref_type(arg_0: f32) -> f32 {
    let converted_arg_0 = arg_0 as _;
    let result = f32_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__f32_ref_type"]
pub static __fce_generated_static_global_f32_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"f32_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"F32\":\"ByRef\"}}],\"output_type\":{\"F32\":\"ByValue\"}}}"
};
pub fn f64_type(arg: f64) -> f64 {
    arg + 1.0
}
#[cfg(target_arch = "wasm32")]
#[export_name = "f64_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_f64_type(arg_0: f64) -> f64 {
    let converted_arg_0 = arg_0 as _;
    let result = f64_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__f64_type"]
pub static __fce_generated_static_global_f64_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"f64_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"F64\":\"ByValue\"}}],\"output_type\":{\"F64\":\"ByValue\"}}}"
};
pub fn f64_ref_type(arg: &f64) -> f64 {
    *arg + 1.0
}
#[cfg(target_arch = "wasm32")]
#[export_name = "f64_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_f64_ref_type(arg_0: f64) -> f64 {
    let converted_arg_0 = arg_0 as _;
    let result = f64_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__f64_ref_type"]
pub static __fce_generated_static_global_f64_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"f64_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"F64\":\"ByRef\"}}],\"output_type\":{\"F64\":\"ByValue\"}}}"
};
pub fn u32_type(arg: u32) -> u32 {
    arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "u32_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_u32_type(arg_0: u32) -> u32 {
    let converted_arg_0 = arg_0 as _;
    let result = u32_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__u32_type"]
pub static __fce_generated_static_global_u32_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"u32_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"U32\":\"ByValue\"}}],\"output_type\":{\"U32\":\"ByValue\"}}}"
};
pub fn u32_ref_type(arg: &u32) -> u32 {
    *arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "u32_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_u32_ref_type(arg_0: u32) -> u32 {
    let converted_arg_0 = arg_0 as _;
    let result = u32_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__u32_ref_type"]
pub static __fce_generated_static_global_u32_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"u32_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"U32\":\"ByRef\"}}],\"output_type\":{\"U32\":\"ByValue\"}}}"
};
pub fn u64_type(arg: u64) -> u64 {
    arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "u64_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_u64_type(arg_0: u64) -> u64 {
    let converted_arg_0 = arg_0 as _;
    let result = u64_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__u64_type"]
pub static __fce_generated_static_global_u64_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"u64_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"U64\":\"ByValue\"}}],\"output_type\":{\"U64\":\"ByValue\"}}}"
};
pub fn u64_ref_type(arg: &u64) -> u64 {
    *arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "u64_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_u64_ref_type(arg_0: u64) -> u64 {
    let converted_arg_0 = arg_0 as _;
    let result = u64_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__u64_ref_type"]
pub static __fce_generated_static_global_u64_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"u64_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"U64\":\"ByRef\"}}],\"output_type\":{\"U64\":\"ByValue\"}}}"
};
pub fn i32_type(arg: i32) -> i32 {
    arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "i32_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_i32_type(arg_0: i32) -> i32 {
    let converted_arg_0 = arg_0 as _;
    let result = i32_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__i32_type"]
pub static __fce_generated_static_global_i32_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"i32_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"I32\":\"ByValue\"}}],\"output_type\":{\"I32\":\"ByValue\"}}}"
};
pub fn i32_ref_type(arg: &i32) -> i32 {
    *arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "i32_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_i32_ref_type(arg_0: i32) -> i32 {
    let converted_arg_0 = arg_0 as _;
    let result = i32_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__i32_ref_type"]
pub static __fce_generated_static_global_i32_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"i32_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"I32\":\"ByRef\"}}],\"output_type\":{\"I32\":\"ByValue\"}}}"
};
pub fn i64_type(arg: i64) -> i64 {
    arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "i64_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_i64_type(arg_0: i64) -> i64 {
    let converted_arg_0 = arg_0 as _;
    let result = i64_type(converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__i64_type"]
pub static __fce_generated_static_global_i64_type: [u8; 139usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"i64_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"I64\":\"ByValue\"}}],\"output_type\":{\"I64\":\"ByValue\"}}}"
};
pub fn i64_ref_type(arg: &i64) -> i64 {
    *arg + 1
}
#[cfg(target_arch = "wasm32")]
#[export_name = "i64_ref_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_i64_ref_type(arg_0: i64) -> i64 {
    let converted_arg_0 = arg_0 as _;
    let result = i64_ref_type(&converted_arg_0);
    let res = result as _;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["import res: ", " ", "\n"],
            &match (&res, &result) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
    };
    return res;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__i64_ref_type"]
pub static __fce_generated_static_global_i64_ref_type: [u8; 141usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"i64_ref_type\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"I64\":\"ByRef\"}}],\"output_type\":{\"I64\":\"ByValue\"}}}"
};
pub fn empty_type() -> String {
    String::from("success")
}
#[cfg(target_arch = "wasm32")]
#[export_name = "empty_type"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_empty_type() {
    let result = empty_type();
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__empty_type"]
pub static __fce_generated_static_global_empty_type: [u8; 111usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"empty_type\",\"arguments\":[],\"output_type\":{\"Utf8String\":\"ByValue\"}}}"
};
