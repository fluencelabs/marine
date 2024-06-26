/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(improper_ctypes)]
#![allow(clippy::all)]
#![allow(unused_mut)]
#![allow(dead_code)]

use marine_rs_sdk::marine;
static mut DATA: Option<Vec<u8>> = None;

fn main() {}

#[marine]
pub struct TestRecord {
    field1: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
}

#[marine]
pub fn fill_2gb_mem() {
    unsafe {
        DATA = Some(Vec::new());
        let mut data = DATA.as_mut().unwrap();
        data.reserve_exact(1);
        let data_offset = std::mem::transmute::<*const u8, usize>(data.as_ptr());
        let size = 0x80000001 - data_offset;
        data.reserve_exact(size);
    }
}

#[marine]
pub fn pass_record(_record: TestRecord) {}

#[marine]
pub fn pass_string(_record: String) {}

#[marine]
pub fn pass_byte_array(_record: Vec<u8>) {}

#[marine]
pub fn pass_array(_record: Vec<u32>) {}

#[marine]
pub fn return_record() -> TestRecord {
    TestRecord {
        field1: 0,
        field2: 1,
        field3: 2,
        field4: 3,
        field5: 4,
    }
}

#[marine]
pub fn return_string() -> String {
    String::from("1234")
}

#[marine]
pub fn return_byte_array() -> Vec<u8> {
    vec![0, 1, 2, 3]
}

#[marine]
pub fn return_array() -> Vec<u32> {
    vec![0, 1, 2, 3]
}
