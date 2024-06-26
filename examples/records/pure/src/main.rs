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

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use test_record::TestRecord;

module_manifest!();

pub fn main() {}

#[marine]
pub fn invoke() -> TestRecord {
    let test_record = TestRecord {
        field_0: false,
        field_1: 0,
        field_2: 0,
        field_3: 0,
        field_4: 0,
        field_5: 0,
        field_6: 0,
        field_7: 0,
        field_8: 0,
        field_9: 0f32,
        field_10: 0f64,
        field_11: String::new(),
        field_12: Vec::new(),
    };

    mutate_struct(test_record)
}

#[marine]
#[module_import("records_effector")]
extern "C" {
    pub fn mutate_struct(test_record: TestRecord) -> TestRecord;
}
