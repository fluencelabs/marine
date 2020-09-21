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

#[fce]
pub fn string_type(mut arg: Vec<String>) -> Vec<String> {
    arg.push(String::from("test"));

    arg
}

#[fce]
pub fn bytearray_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg
}

/*
#[fce]
pub fn bool_type(mut arg: Vec<bool>) -> Vec<bool> {
    arg.push(true);

    arg
}
 */

#[fce]
pub fn f32_type(mut arg: Vec<f32>) -> Vec<f32> {
    arg.push(1.0);

    arg
}

#[fce]
pub fn f64_type(mut arg: Vec<f64>) -> Vec<f64> {
    arg.push(1.0);

    arg
}

#[fce]
pub fn u32_type(mut arg: Vec<u32>) -> Vec<u32> {
    arg.push(1);

    arg
}

#[fce]
pub fn u64_type(mut arg: Vec<u64>) -> Vec<u64> {
    arg.push(1);

    arg
}

#[fce]
pub fn i32_type(mut arg: Vec<i32>) -> Vec<i32> {
    arg.push(1);

    arg
}

#[fce]
pub fn i64_type(mut arg: Vec<i64>) -> Vec<i64> {
    arg.push(1);

    arg
}

#[fce]
pub fn empty_type() -> Vec<String> {
    vec![String::from("success")]
}
