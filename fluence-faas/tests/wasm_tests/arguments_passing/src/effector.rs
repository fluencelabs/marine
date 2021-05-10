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

use fluence::marine;

pub fn main() {}

#[marine]
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

#[marine]
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

#[marine]
pub fn string_type(arg: String) -> String {
    format!("{}_{}", arg, arg)
}

#[marine]
pub fn string_ref_type(arg: &String) -> String {
    format!("{}_{}", arg, arg)
}

#[marine]
pub fn str_type(arg: &str) -> String {
    format!("{}_{}", arg, arg)
}

#[marine]
pub fn bytearray_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg
}

#[marine]
pub fn bytearray_ref_type(arg: &mut Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg.clone()
}

#[marine]
pub fn bool_type(arg: bool) -> bool {
    !arg
}

#[marine]
pub fn bool_ref_type(arg: &bool) -> bool {
    !*arg
}

#[marine]
pub fn f32_type(arg: f32) -> f32 {
    arg + 1.0
}

#[marine]
pub fn f32_ref_type(arg: &f32) -> f32 {
    *arg + 1.0
}

#[marine]
pub fn f64_type(arg: f64) -> f64 {
    arg + 1.0
}

#[marine]
pub fn f64_ref_type(arg: &f64) -> f64 {
    *arg + 1.0
}

#[marine]
pub fn u32_type(arg: u32) -> u32 {
    arg + 1
}

#[marine]
pub fn u32_ref_type(arg: &u32) -> u32 {
    *arg + 1
}

#[marine]
pub fn u64_type(arg: u64) -> u64 {
    arg + 1
}

#[marine]
pub fn u64_ref_type(arg: &u64) -> u64 {
    *arg + 1
}

#[marine]
pub fn i32_type(arg: i32) -> i32 {
    arg + 1
}

#[marine]
pub fn i32_ref_type(arg: &i32) -> i32 {
    *arg + 1
}

#[marine]
pub fn i64_type(arg: i64) -> i64 {
    arg + 1
}

#[marine]
pub fn i64_ref_type(arg: &i64) -> i64 {
    *arg + 1
}

#[marine]
pub fn empty_type() -> String {
    String::from("success")
}
