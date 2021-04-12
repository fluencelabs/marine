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

#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
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
    let mut result = unsafe {
        effector::all_types(
            arg_0,
            arg_1,
            arg_2,
            arg_3,
            arg_4,
            arg_5,
            arg_6,
            arg_7,
            arg_8,
            arg_9,
            arg_10.clone(),
            arg_11.clone(),
        )
    };

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

#[fce]
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
    let mut result = unsafe {
        effector::all_ref_types(
            arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10, arg_11,
        )
    };

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

#[fce]
pub fn string_type(arg: String) -> String {
    let arg = unsafe { effector::string_type(arg) };

    format!("{}_{}", arg, arg)
}

#[fce]
pub fn string_ref_type(arg: &String) -> String {
    let arg = unsafe { effector::string_ref_type(arg) };

    format!("{}_{}", arg, arg)
}

#[fce]
pub fn str_type(arg: &str) -> String {
    let arg = unsafe { effector::str_type(arg) };

    format!("{}_{}", arg, arg)
}

#[fce]
pub fn bytearray_type(arg: Vec<u8>) -> Vec<u8> {
    let mut arg = unsafe { effector::bytearray_type(arg) };

    arg.push(1);
    arg
}

#[fce]
pub fn bytearray_ref_type(arg: &Vec<u8>) -> Vec<u8> {
    let mut arg = unsafe { effector::bytearray_ref_type(arg) };

    arg.push(1);
    arg
}

#[fce]
pub fn bool_type(arg: bool) -> bool {
    unsafe { effector::bool_type(arg) }
}

#[fce]
pub fn bool_ref_type(arg: &bool) -> bool {
    unsafe { effector::bool_ref_type(arg) }
}

#[fce]
pub fn f32_type(arg: f32) -> f32 {
    let arg = unsafe { effector::f32_type(arg) };
    arg + 1.0
}

#[fce]
pub fn f32_ref_type(arg: &f32) -> f32 {
    let arg = unsafe { effector::f32_ref_type(arg) };
    arg + 1.0
}

#[fce]
pub fn f64_type(arg: f64) -> f64 {
    let arg = unsafe { effector::f64_type(arg) };
    arg + 1.0
}

#[fce]
pub fn f64_ref_type(arg: &f64) -> f64 {
    let arg = unsafe { effector::f64_ref_type(arg) };
    arg + 1.0
}

#[fce]
pub fn u32_type(arg: u32) -> u32 {
    let arg = unsafe { effector::u32_type(arg) };
    arg + 1
}

#[fce]
pub fn u32_ref_type(arg: &u32) -> u32 {
    let arg = unsafe { effector::u32_ref_type(arg) };
    arg + 1
}

#[fce]
pub fn u64_type(arg: u64) -> u64 {
    let arg = unsafe { effector::u64_type(arg) };
    arg + 1
}

#[fce]
pub fn u64_ref_type(arg: &u64) -> u64 {
    let arg = unsafe { effector::u64_ref_type(arg) };
    arg + 1
}

#[fce]
pub fn i32_type(arg: i32) -> i32 {
    let arg = unsafe { effector::i32_type(arg) };
    arg + 1
}

#[fce]
pub fn i32_ref_type(arg: i32) -> i32 {
    let arg = unsafe { effector::i32_type(arg) };
    arg + 1
}

#[fce]
pub fn i64_type(arg: i64) -> i64 {
    let arg = unsafe { effector::i64_type(arg) };
    arg + 1
}

#[fce]
pub fn i64_ref_type(arg: &i64) -> i64 {
    let arg = unsafe { effector::i64_ref_type(arg) };
    arg + 1
}

#[fce]
pub fn empty_type() -> String {
    unsafe { effector::empty_type() }
}

mod effector {
    use fluence::fce;

    #[fce]
    #[link(wasm_import_module = "arguments_passing_effector")]
    extern "C" {
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
        ) -> Vec<u8>;

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
        ) -> Vec<u8>;

        pub fn string_type(arg: String) -> String;
        pub fn string_ref_type(arg: &String) -> String;

        pub fn str_type(arg: &str) -> String;

        pub fn bytearray_type(arg: Vec<u8>) -> Vec<u8>;
        pub fn bytearray_ref_type(arg: &Vec<u8>) -> Vec<u8>;

        pub fn bool_type(arg: bool) -> bool;
        pub fn bool_ref_type(arg: &bool) -> bool;

        pub fn f32_type(arg: f32) -> f32;
        pub fn f32_ref_type(arg: &f32) -> f32;

        pub fn f64_type(arg: f64) -> f64;
        pub fn f64_ref_type(arg: &f64) -> f64;

        pub fn u32_type(arg: u32) -> u32;
        pub fn u32_ref_type(arg: &u32) -> u32;

        pub fn u64_type(arg: u64) -> u64;
        pub fn u64_ref_type(arg: &u64) -> u64;

        pub fn i32_type(arg: i32) -> i32;
        pub fn i32_ref_type(arg: &i32) -> i32;

        pub fn i64_type(arg: i64) -> i64;
        pub fn i64_ref_type(arg: &i64) -> i64;

        pub fn empty_type() -> String;
    }
}
