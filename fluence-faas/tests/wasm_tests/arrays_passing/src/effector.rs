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
pub fn byte_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(1);
    arg
}

#[marine]
pub fn inner_arrays_1(mut arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    arg.push(vec![]);
    arg.push(vec![vec![]]);
    arg.push(vec![vec![vec![]]]);
    arg.push(vec![vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]]);

    arg
}

#[marine]
#[derive(Debug, Default)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[marine]
pub fn inner_arrays_2(mut arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
    arg.push(vec![]);
    arg.push(vec![vec![]]);
    arg.push(vec![vec![vec![]]]);
    arg.push(vec![vec![vec![TestRecord {
        field_0: 0,
        field_1: vec![vec![1, 2, 3, 4]],
    }]]]);

    arg
}

#[marine]
pub fn string_type(mut arg: Vec<String>) -> Vec<String> {
    arg.push(String::from("from effector"));
    arg
}

#[marine]
pub fn bool_type(mut arg: Vec<bool>) -> Vec<bool> {
    arg.push(true);
    arg.push(false);
    arg.push(true);
    arg
}

#[marine]
pub fn f32_type(mut arg: Vec<f32>) -> Vec<f32> {
    arg.push(13.37);
    arg
}

#[marine]
pub fn f64_type(mut arg: Vec<f64>) -> Vec<f64> {
    arg.push(13.37);
    arg
}

#[marine]
pub fn u32_type(mut arg: Vec<u32>) -> Vec<u32> {
    arg.push(13);
    arg.push(37);
    arg
}

#[marine]
pub fn u64_type(mut arg: Vec<u64>) -> Vec<u64> {
    arg.push(1);
    arg.push(2);
    arg.push(3);
    arg.push(4);

    arg
}

#[marine]
pub fn i32_type(mut arg: Vec<i32>) -> Vec<i32> {
    arg.push(1);
    arg.push(2);
    arg.push(3);
    arg.push(4);
    arg.push(0);
    arg
}

#[marine]
pub fn i64_type(mut arg: Vec<i64>) -> Vec<i64> {
    arg.push(1);
    arg.push(2);
    arg.push(3);
    arg.push(4);
    arg.push(1);
    arg
}

#[marine]
pub fn empty_type() -> Vec<String> {
    vec![String::from("from effector")]
}
