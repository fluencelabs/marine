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
pub fn byte_type(arg: Vec<u8>) -> Vec<u8> {
    let mut arg = unsafe { effector::byte_type(arg) };

    arg.push(1);
    arg
}

#[fce]
pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<u8> {
    let mut result = unsafe { effector::inner_arrays_1(arg) };

    result
}

#[fce]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[fce]
pub fn inner_arrays_2(arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<u8> {
    let mut result = unsafe { effector::inner_arrays_2(arg) };

    result
}

#[fce]
pub fn string_type(arg: Vec<String>) -> Vec<String> {
    let mut arg = unsafe { effector::string_type(arg) };

    arg.push(String::from("test"));
    arg
}

/*
#[fce]
pub fn bool_type(arg: Vec<bool>) -> Vec<bool> {
    let mut arg = unsafe { effector::bool_type(arg) };

    arg.push(false);
    arg
}
 */

#[fce]
pub fn f32_type(arg: Vec<f32>) -> Vec<f32> {
    let mut arg = unsafe { effector::f32_type(arg) };

    arg.push(1.0);
    arg
}

#[fce]
pub fn f64_type(arg: Vec<f64>) -> Vec<f64> {
    let mut arg = unsafe { effector::f64_type(arg) };

    arg.push(1.0);
    arg
}

#[fce]
pub fn u32_type(arg: Vec<u32>) -> Vec<u32> {
    let mut arg = unsafe { effector::u32_type(arg) };

    arg.push(0);
    arg
}

#[fce]
pub fn u64_type(arg: Vec<u64>) -> Vec<u64> {
    let mut arg = unsafe { effector::u64_type(arg) };
    arg.push(0);
    arg
}

#[fce]
pub fn i32_type(arg: Vec<i32>) -> Vec<i32> {
    let mut arg = unsafe { effector::i32_type(arg) };

    arg.push(0);
    arg
}

#[fce]
pub fn i64_type(arg: Vec<i64>) -> Vec<i64> {
    let mut arg = unsafe { effector::i64_type(arg) };

    arg.push(1);
    arg
}

#[fce]
pub fn empty_type() -> Vec<String> {
    unsafe { effector::empty_type() }
}

mod effector {
    use fluence::fce;
    use super::TestRecord;

    #[fce]
    #[link(wasm_import_module = "arrays_passing_effector")]
    extern "C" {
        pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<u8>;

        pub fn inner_arrays_2(arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<u8>;

        pub fn string_type(arg: Vec<String>) -> Vec<String>;

        pub fn byte_type(arg: Vec<u8>) -> Vec<u8>;

        /*
        pub fn bool_type(arg: Vec<bool>) -> Vec<bool>;
         */

        pub fn f32_type(arg: Vec<f32>) -> Vec<f32>;

        pub fn f64_type(arg: Vec<f64>) -> Vec<f64>;

        pub fn u32_type(arg: Vec<u32>) -> Vec<u32>;

        pub fn u64_type(arg: Vec<u64>) -> Vec<u64>;

        pub fn i32_type(arg: Vec<i32>) -> Vec<i32>;

        pub fn i64_type(arg: Vec<i64>) -> Vec<i64>;

        pub fn empty_type() -> Vec<String>;
    }
}
