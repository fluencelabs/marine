#![feature(prelude_import)]
#![allow(improper_ctypes)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
mod path {
    pub(super) fn to_full_path<S>(cmd: S) -> String
    where
        S: Into<String>,
    {
        use std::path::Path;
        use std::path::Component;
        let cmd = cmd.into();
        let path = Path::new(&cmd);
        let mut components = path.components();
        let is_absolute = components.next() == Some(Component::RootDir);
        if !is_absolute {
            return cmd;
        }
        let parent = match components.next() {
            Some(Component::Normal(path)) => path.to_str().unwrap(),
            _ => return cmd,
        };
        match std::env::var(parent) {
            Ok(to_dir) => {
                let mut full_path = std::path::PathBuf::from(to_dir);
                #[allow(clippy::while_let_on_iterator)]
                while let Some(component) = components.next() {
                    full_path.push(component);
                }
                full_path.to_string_lossy().into_owned()
            }
            Err(_) => cmd,
        }
    }
}
use crate::path::to_full_path;
use fluence::fce;
use fluence::WasmLoggerBuilder;
use fluence::MountedBinaryResult;
const RESULT_FILE_PATH: &str = "/tmp/ipfs_rpc_file";
const IPFS_ADDR_ENV_NAME: &str = "IPFS_ADDR";
const TIMEOUT_ENV_NAME: &str = "timeout";
pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::Level::Info)
        .build()
        .unwrap();
}
/// Put file from specified path to IPFS and return its hash.
pub fn put(file_path: String) -> String {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["put called with file path "],
                    &match (&file_path,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &(
                    "ipfs_effector",
                    "ipfs_effector",
                    "examples/ipfs-node/effector/src/main.rs",
                    41u32,
                ),
            );
        }
    };
    let file_path = to_full_path(file_path);
    let timeout = std::env::var(TIMEOUT_ENV_NAME).unwrap_or_else(|_| "1s".to_string());
    let cmd = <[_]>::into_vec(box [
        String::from("add"),
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["--timeout "],
                &match (&timeout,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        },
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["-Q "],
                &match (&file_path,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        },
    ]);
    let exec_result = unsafe { ipfs(cmd) };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["exec_result: ", "\n"],
            &match (&exec_result,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    String::from_utf8(exec_result.stdout).unwrap()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "put"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_put(arg_0: u32, arg_1: u32) {
    let converted_arg_0 = String::from_raw_parts(arg_0 as _, arg_1 as _, arg_1 as _);
    let result = put(converted_arg_0);
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    std::mem::forget(result);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__put"]
pub static __fce_generated_static_global_put: [u8; 118usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"put\",\"arguments\":[[\"file_path\",\"Utf8String\"]],\"output_type\":\"Utf8String\"}}"
};
/// Get file by provided hash from IPFS, saves it to a temporary file and returns a path to it.
pub fn get(hash: String) -> String {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["get called with hash "],
                    &match (&hash,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &(
                    "ipfs_effector",
                    "ipfs_effector",
                    "examples/ipfs-node/effector/src/main.rs",
                    61u32,
                ),
            );
        }
    };
    let result_file_path = to_full_path(RESULT_FILE_PATH);
    let timeout = std::env::var(TIMEOUT_ENV_NAME).unwrap_or_else(|_| "1s".to_string());
    let cmd = <[_]>::into_vec(box [
        String::from("get"),
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["--timeout "],
                &match (&timeout,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        },
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["-o "],
                &match (&result_file_path,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        },
        hash,
    ]);
    let exec_result = unsafe { ipfs(cmd) };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["exec_result: ", "\n"],
            &match (&exec_result,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    RESULT_FILE_PATH.to_string()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "get"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_get(arg_0: u32, arg_1: u32) {
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
pub static __fce_generated_static_global_get: [u8; 113usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"get\",\"arguments\":[[\"hash\",\"Utf8String\"]],\"output_type\":\"Utf8String\"}}"
};
pub fn get_address() -> String {
    match std::env::var(IPFS_ADDR_ENV_NAME) {
        Ok(addr) => addr,
        Err(e) => {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["getting ", " env variable failed with error "],
                &match (&IPFS_ADDR_ENV_NAME, &e) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            ));
            res
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[export_name = "get_address"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_get_address() {
    let result = get_address();
    fluence::internal::set_result_ptr(result.as_ptr() as _);
    fluence::internal::set_result_size(result.len() as _);
    std::mem::forget(result);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__get_address"]
pub static __fce_generated_static_global_get_address: [u8; 100usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"get_address\",\"arguments\":[],\"output_type\":\"Utf8String\"}}"
};
fn ipfs_export(arg: Vec<u16>) {}
#[cfg(target_arch = "wasm32")]
#[export_name = "ipfs_export"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_ipfs_export(arg_0: u32, arg_1: u32) {
    unsafe fn __fce_generated_vec_deserializer_0(offset: u32, size: u32) -> Vec<u16> {
        let size = size / 8;
        let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
        let mut result = Vec::with_capacity(arg.len());
        for value in arg {
            result.push(value as _);
        }
        result
    }
    let converted_arg_0 = __fce_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    ipfs_export(converted_arg_0);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__ipfs_export"]
pub static __fce_generated_static_global_ipfs_export: [u8; 116usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"ipfs_export\",\"arguments\":[[\"arg\",{\"Vector\":\"U16\"}]],\"output_type\":null}}"
};
#[link(wasm_import_module = "host")]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "ipfs"]
    fn __fce_generated_wrapper_func__ipfs(arg_0: u32, arg_1: u32);
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn ipfs(arg_0: Vec<u16>) -> MountedBinaryResult {
    unsafe fn __fce_generated_vec_serializer_arg_0(arg: Vec<u16>) -> (u32, u32) {
        let arg = std::mem::ManuallyDrop::new(arg);
        (arg.as_ptr() as _, (2 * arg.len()) as _)
    }
    let arg_0 = __fce_generated_vec_serializer_arg_0(arg_0);
    let result = __fce_generated_wrapper_func__ipfs(arg_0.0 as _, arg_0.1 as _);
    MountedBinaryResult::__fce_generated_deserialize(fluence::internal::get_result_ptr() as _)
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__host"]
pub static __fce_generated_static_global_host: [u8; 188usize] = {
    * b"{\"ast_type\":\"ExternMod\",\"namespace\":\"host\",\"imports\":[{\"link_name\":null,\"signature\":{\"name\":\"ipfs\",\"arguments\":[[\"cmd\",{\"Vector\":\"U16\"}]],\"output_type\":{\"Record\":\"MountedBinaryResult\"}}}]}"
};
