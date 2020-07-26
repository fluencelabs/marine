#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use fluence::fce;
use fluence::WasmLogger;
use std::fs;
use std::path::PathBuf;
const RPC_TMP_FILEPATH: &str = "/tmp/ipfs_rpc_file";
pub fn main() {
    WasmLogger::init_with_level(log::Level::Info).unwrap();
}
pub struct Asadasd {
    pub a: i32,
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
pub(crate) fn __fce_generated_record_serializer_Asadasd(record: Asadasd) -> i32 {
    let mut raw_record = Vec::new();
    raw_record.push(record.a as u64);
    let raw_record_ptr = raw_record.as_ptr();
    std::mem::forget(raw_record);
    raw_record_ptr as _
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
unsafe fn __fce_generated_record_deserializer_Asadasd(offset: i32, size: i32) -> Asadasd {
    let raw_record: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
    let field_0 = raw_record[0usize] as i32;
    Asadasd { a: field_0 }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__Asadasd"]
pub static __fce_generated_static_global_Asadasd: [u8; 73usize] = {
    *b"{\"ast_type\":\"Record\",\"name\":\"Asadasd\",\"fields\":[{\"name\":\"a\",\"ty\":\"I32\"}]}"
};
pub fn invoke(_a: Asadasd) -> String {
    "IPFS_RPC wasm example, it allows to:\ninvoke\nput\nget".to_string()
}
#[export_name = "invoke"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_invoke(arg_0: i32, arg_1: i32) {
    let converted_arg_0 = __fce_generated_record_deserializer_Asadasd(arg_0, arg_1);
    let result = invoke(converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    std::mem::forget(result);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__invoke"]
pub static __fce_generated_static_global_invoke: [u8; 117usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"invoke\",\"input_types\":[{\"Record\":\"Asadasd\"}],\"output_type\":\"Utf8String\"}}"
};
pub fn put(file_content: Vec<u8>) -> String {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["put called with "],
                    &match (&file_content,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                ),
                lvl,
                &(
                    "ipfs_rpc",
                    "ipfs_rpc",
                    "examples/ipfs_node/wasm/ipfs_rpc/src/main.rs",
                    41u32,
                ),
            );
        }
    };
    let rpc_tmp_filepath = RPC_TMP_FILEPATH.to_string();
    let r = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = r {
        return {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["file can\'t be written: "],
                &match (&e,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        };
    }
    ipfs_put(rpc_tmp_filepath)
}
#[export_name = "put"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_put(arg_0: i32, arg_1: i32) {
    let converted_arg_0 = Vec::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = put(converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    std::mem::forget(result);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__put"]
pub static __fce_generated_static_global_put: [u8; 106usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"put\",\"input_types\":[\"ByteVector\"],\"output_type\":\"Utf8String\"}}"
};
pub fn get(hash: String) -> Vec<u8> {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["get called with hash: "],
                    &match (&hash,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &(
                    "ipfs_rpc",
                    "ipfs_rpc",
                    "examples/ipfs_node/wasm/ipfs_rpc/src/main.rs",
                    55u32,
                ),
            );
        }
    };
    let file_path = ipfs_get(hash);
    fs::read(file_path).unwrap_or_else(|_| b"error while reading file".to_vec())
}
#[export_name = "get"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_get(arg_0: i32, arg_1: i32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = get(converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    std::mem::forget(result);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__get"]
pub static __fce_generated_static_global_get: [u8; 106usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"get\",\"input_types\":[\"Utf8String\"],\"output_type\":\"ByteVector\"}}"
};
#[link(wasm_import_module = "ipfs_node.wasm")]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "put"]
    fn __fce_generated_wrapper_func__ipfs_put(arg_0: i32, arg_1: i32);
    #[link_name = "get"]
    fn __fce_generated_wrapper_func__ipfs_get(arg_0: i32, arg_1: i32);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
pub fn ipfs_put(arg_0: String) -> String {
    unsafe {
        let result = __fce_generated_wrapper_func__ipfs_put(arg_0.as_ptr() as _, arg_0.len() as _);
        String::from_raw_parts(
            fluence::internal::get_result_ptr() as _,
            fluence::internal::get_result_size() as _,
            fluence::internal::get_result_size() as _,
        )
    }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
pub fn ipfs_get(arg_0: String) -> String {
    unsafe {
        let result = __fce_generated_wrapper_func__ipfs_get(arg_0.as_ptr() as _, arg_0.len() as _);
        String::from_raw_parts(
            fluence::internal::get_result_ptr() as _,
            fluence::internal::get_result_size() as _,
            fluence::internal::get_result_size() as _,
        )
    }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__ipfs_node_wasm"]
pub static __fce_generated_static_global_ipfs_node_wasm: [u8; 281usize] = {
    * b"{\"ast_type\":\"ExternMod\",\"namespace\":\"ipfs_node.wasm\",\"imports\":[{\"link_name\":\"put\",\"signature\":{\"name\":\"ipfs_put\",\"input_types\":[\"Utf8String\"],\"output_type\":\"Utf8String\"}},{\"link_name\":\"get\",\"signature\":{\"name\":\"ipfs_get\",\"input_types\":[\"Utf8String\"],\"output_type\":\"Utf8String\"}}]}"
};
