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
use test_record::TestRecord;

pub fn main() {}

#[fce]
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

    unsafe { mutate_struct(test_record) }
}

#[fce]
#[link(wasm_import_module = "records_effector")]
extern "C" {
    pub fn mutate_struct(test_record: TestRecord) -> TestRecord;
}
