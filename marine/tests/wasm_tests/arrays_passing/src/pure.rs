/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(improper_ctypes)]
#![allow(clippy::all)]

use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn byte_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(0);

    let mut arg = effector::byte_type(arg);

    arg.push(2);
    arg
}

#[marine]
pub fn inner_arrays_1(mut arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    arg.push(vec![vec![vec![0]]]);

    let mut arg = effector::inner_arrays_1(arg);

    arg.push(vec![vec![vec![2]]]);
    arg
}

#[marine]
#[derive(Default, Debug)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[marine]
pub fn inner_arrays_2(mut arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
    arg.push(vec![vec![vec![
        TestRecord {
            field_0: 0,
            field_1: vec![vec![1]],
        },
        TestRecord::default(),
    ]]]);

    let mut arg = effector::inner_arrays_2(arg);

    arg.push(vec![vec![vec![
        TestRecord {
            field_0: 1,
            field_1: vec![vec![2]],
        },
        TestRecord::default(),
    ]]]);

    arg
}

#[marine]
pub fn string_type(mut arg: Vec<String>) -> Vec<String> {
    arg.push(String::from("marine"));

    let mut arg = effector::string_type(arg);

    arg.push(String::from("test"));
    arg
}

#[marine]
pub fn bool_type(mut arg: Vec<bool>) -> Vec<bool> {
    arg[0] = !arg[0];
    let mut arg = effector::bool_type(arg);

    arg.push(false);
    arg.push(true);
    arg
}

#[marine]
pub fn f32_type(mut arg: Vec<f32>) -> Vec<f32> {
    arg.push(0.0);

    let mut arg = effector::f32_type(arg);

    arg.push(1.0);
    arg
}

#[marine]
pub fn f64_type(mut arg: Vec<f64>) -> Vec<f64> {
    arg.push(0.0);

    let mut arg = effector::f64_type(arg);

    arg.push(1.0);
    arg
}

#[marine]
pub fn u32_type(mut arg: Vec<u32>) -> Vec<u32> {
    arg.push(0);

    let mut arg = effector::u32_type(arg);

    arg.push(2);
    arg
}

#[marine]
pub fn u64_type(mut arg: Vec<u64>) -> Vec<u64> {
    arg.push(0);

    let mut arg = effector::u64_type(arg);

    arg.push(2);
    arg
}

#[marine]
pub fn i32_type(mut arg: Vec<i32>) -> Vec<i32> {
    arg.push(0);

    let mut arg = effector::i32_type(arg);

    arg.push(2);
    arg
}

#[marine]
pub fn i64_type(mut arg: Vec<i64>) -> Vec<i64> {
    arg.push(0);

    let mut arg = effector::i64_type(arg);

    arg.push(1);
    arg
}

#[marine]
pub fn empty_type() -> Vec<String> {
    effector::empty_type()
}

mod effector {
    use marine_rs_sdk::marine;
    use super::TestRecord;

    #[marine]
    #[module_import("arrays_passing_effector")]
    extern "C" {
        pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>>;

        pub fn inner_arrays_2(
            arg: Vec<Vec<Vec<Vec<TestRecord>>>>,
        ) -> Vec<Vec<Vec<Vec<TestRecord>>>>;

        pub fn string_type(arg: Vec<String>) -> Vec<String>;

        pub fn byte_type(arg: Vec<u8>) -> Vec<u8>;

        pub fn bool_type(arg: Vec<bool>) -> Vec<bool>;

        pub fn f32_type(arg: Vec<f32>) -> Vec<f32>;

        pub fn f64_type(arg: Vec<f64>) -> Vec<f64>;

        pub fn u32_type(arg: Vec<u32>) -> Vec<u32>;

        pub fn u64_type(arg: Vec<u64>) -> Vec<u64>;

        pub fn i32_type(arg: Vec<i32>) -> Vec<i32>;

        pub fn i64_type(arg: Vec<i64>) -> Vec<i64>;

        pub fn empty_type() -> Vec<String>;
    }
}
