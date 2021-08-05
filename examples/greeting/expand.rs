#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
const __M_SDK_BUILD_TIME: &str = "2021-08-04T14:52:56.083033+00:00";
const __M_SDK_AUTHORS_SIZE: usize = "Fluence Labs".as_bytes().len();
const __M_SDK_VERSION_SIZE: usize = "0.1.0".as_bytes().len();
const __M_SDK_DESCRIPTION_SIZE: usize = "The greeting module for the Fluence network"
    .as_bytes()
    .len();
const __M_SDK_REPOSITORY_SIZE: usize =
    "https://github.com/fluencelabs/marine/tree/master/examples/greeting"
        .as_bytes()
        .len();
const __M_SDK_BUILD_TIME_SIZE: usize = __M_SDK_BUILD_TIME.as_bytes().len();
const __M_SDK_FIELD_PREFIX_SIZE: usize = std::mem::size_of::<u64>();
const __M_MANIFEST_SIZE: usize = __M_SDK_AUTHORS_SIZE
    + __M_SDK_VERSION_SIZE
    + __M_SDK_DESCRIPTION_SIZE
    + __M_SDK_REPOSITORY_SIZE
    + __M_SDK_BUILD_TIME_SIZE
    + __M_SDK_FIELD_PREFIX_SIZE * 5;
const fn __m_sdk_append_data(
    mut manifest: [u8; __M_MANIFEST_SIZE],
    data: &'static str,
    offset: usize,
) -> ([u8; __M_MANIFEST_SIZE], usize) {
    let data_as_bytes = data.as_bytes();
    let data_len = data_as_bytes.len();
    let data_len_u64 = data_len as u64;
    let data_len_le_bytes = data_len_u64.to_le_bytes();
    let mut byte_idx = 0;
    while byte_idx < __M_SDK_FIELD_PREFIX_SIZE {
        manifest[offset + byte_idx] = data_len_le_bytes[byte_idx];
        byte_idx += 1;
    }
    let mut byte_idx = 0;
    while byte_idx < data_len {
        manifest[__M_SDK_FIELD_PREFIX_SIZE + offset + byte_idx] = data_as_bytes[byte_idx];
        byte_idx += 1;
    }
    (manifest, offset + __M_SDK_FIELD_PREFIX_SIZE + data_len)
}
const fn generate_manifest() -> [u8; __M_MANIFEST_SIZE] {
    let manifest: [u8; __M_MANIFEST_SIZE] = [0; __M_MANIFEST_SIZE];
    let offset = 0;
    let (manifest, offset) = __m_sdk_append_data(manifest, "Fluence Labs", offset);
    let (manifest, offset) = __m_sdk_append_data(manifest, "0.1.0", offset);
    let (manifest, offset) = __m_sdk_append_data(
        manifest,
        "The greeting module for the Fluence network",
        offset,
    );
    let (manifest, offset) = __m_sdk_append_data(
        manifest,
        "https://github.com/fluencelabs/marine/tree/master/examples/greeting",
        offset,
    );
    let (manifest, _) = __m_sdk_append_data(manifest, __M_SDK_BUILD_TIME, offset);
    manifest
}
#[cfg(target_arch = "wasm32")]
#[link_section = "__fluence_wasm_module_manifest"]
#[doc(hidden)]
pub static __M_WASM_MODULE_MANIFEST: [u8; __M_MANIFEST_SIZE] = generate_manifest();
pub fn main() {}
pub fn greeting(name: String) -> String {
    {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["Hi, "],
            &match (&name,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
        res
    }
}
#[cfg(target_arch = "wasm32")]
#[export_name = "greeting"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __m_generated_wrapper_func_greeting(arg_0: u32, arg_1: u32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = greeting(converted_arg_0);
    marine_rs_sdk::internal::set_result_ptr(result.as_ptr() as _);
    marine_rs_sdk::internal::set_result_size(result.len() as _);
    marine_rs_sdk::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__m_generated_section__greeting"]
pub static __m_generated_static_global_greeting: [u8; 157usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"greeting\",\"arguments\":[{\"name\":\"name\",\"ty\":{\"Utf8String\":\"ByValue\"}}],\"output_types\":[{\"Utf8String\":\"ByValue\"}]}}"
};
