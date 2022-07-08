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
#![allow(clippy::all)]

use marine_rs_sdk::marine;
use core::cell::RefCell;

#[marine]
#[derive(Clone, Debug, Default)]
pub struct TestRecord0 {
    pub field_0: i32,
}

#[marine]
#[derive(Clone, Debug, Default)]
pub struct TestRecord1 {
    pub field_0: i32,
    pub field_1: String,
    pub field_2: Vec<u8>,
    pub test_record_0: TestRecord0,
}

#[marine]
#[derive(Clone, Debug, Default)]
pub struct TestRecord2 {
    pub test_record_0: TestRecord0,
    pub test_record_1: TestRecord1,
}

thread_local!(static DROP_COUNT: RefCell<i32> = RefCell::new(0));

#[marine]
#[derive(Debug, Clone, Default)]
pub struct DroppableRecordTree {
    id: i32,
}

#[marine]
#[derive(Clone, Debug, Default)]
pub struct DroppableRecordTreeConainer {
    data: DroppableRecordTree,
    data2: Vec<DroppableRecordTree>,
}

impl Drop for DroppableRecordTree {
    fn drop(&mut self) {
        DROP_COUNT.with(|count| {
            let mut count = count.borrow_mut();
            *count = *count + 1;
        });
    }
}

fn main() {}

#[marine]
#[derive(Default, Clone, Debug)]
pub struct SomeResult {
    records: Vec<DroppableRecordTreeConainer>,
    counts: Vec<i32>,
}

#[marine]
pub fn pass_droppable_record(
    record: DroppableRecordTreeConainer,
    records: Vec<DroppableRecordTreeConainer>,
) -> Vec<DroppableRecordTreeConainer> {
    effector::pass_droppable_record(record.clone(), records.clone()).clone()
}

#[marine]
pub fn get_drop_count() -> Vec<i32> {
    let pure_drop_count = DROP_COUNT.with(|count| *count.borrow());
    let effector_drop_count = effector::get_drop_count();
    vec![pure_drop_count, effector_drop_count]
}

#[marine]
pub fn test_record(test_record: TestRecord2) -> TestRecord2 {
    let mut test_record = effector::test_record(test_record);

    test_record.test_record_1 = TestRecord1 {
        field_0: 1,
        field_1: "fluence".to_string(),
        field_2: vec![0x13, 0x37],
        test_record_0: TestRecord0 { field_0: 5 },
    };

    test_record
}

#[marine]
#[allow(dead_code)]
fn test_record_ref(test_record: &TestRecord2) -> TestRecord2 {
    let mut test_record = effector::test_record_ref(test_record);

    test_record.test_record_1 = TestRecord1 {
        field_0: 1,
        field_1: "fluence".to_string(),
        field_2: vec![0x13, 0x37],
        test_record_0: TestRecord0 { field_0: 5 },
    };

    test_record
}

#[marine]
#[derive(Default)]
pub struct Data64b {
    field1: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
    field6: i32,
    field7: i32,
    field8: i32,
    field11: i32,
    field12: i32,
    field13: i32,
    field14: i32,
    field15: i32,
    field16: i32,
    field17: i32,
    field18: i32,
}

#[marine]
#[derive(Default)]
pub struct Data1KB {
    field1: Data64b,
    field2: Data64b,
    field3: Data64b,
    field4: Data64b,
    field5: Data64b,
    field6: Data64b,
    field7: Data64b,
    field8: Data64b,
    field11: Data64b,
    field12: Data64b,
    field13: Data64b,
    field14: Data64b,
    field15: Data64b,
    field16: Data64b,
    field17: Data64b,
    field18: Data64b,
}

#[marine]
#[derive(Default)]
pub struct Data16KB {
    field1: Data1KB,
    field2: Data1KB,
    field3: Data1KB,
    field4: Data1KB,
    field5: Data1KB,
    field6: Data1KB,
    field7: Data1KB,
    field8: Data1KB,
    field11: Data1KB,
    field12: Data1KB,
    field13: Data1KB,
    field14: Data1KB,
    field15: Data1KB,
    field16: Data1KB,
    field17: Data1KB,
    field18: Data1KB,
}

#[marine]
#[derive(Default)]
pub struct Data256KB {
    field1: Data16KB,
    field2: Data16KB,
    field3: Data16KB,
    field4: Data16KB,
    field5: Data16KB,
    field6: Data16KB,
    field7: Data16KB,
    field8: Data16KB,
    field11: Data16KB,
    field12: Data16KB,
    field13: Data16KB,
    field14: Data16KB,
    field15: Data16KB,
    field16: Data16KB,
    field17: Data16KB,
    field18: Data16KB,
}

#[marine]
fn return_256kb_struct() -> Data256KB {
    effector::return_256kb_struct()
}

#[marine]
fn pass_256kb_struct(arg: Data256KB) {
    effector::pass_256kb_struct(arg)
}

mod effector {
    use marine_rs_sdk::marine;
    use crate::{Data256KB, DroppableRecordTreeConainer};
    use super::TestRecord2;

    #[marine]
    #[link(wasm_import_module = "records_passing_effector")]
    extern "C" {
        pub fn pass_droppable_record(
            record: DroppableRecordTreeConainer,
            records: Vec<DroppableRecordTreeConainer>,
        ) -> Vec<DroppableRecordTreeConainer>;

        pub fn pass_256kb_struct(arg: Data256KB);

        pub fn return_256kb_struct() -> Data256KB;

        pub fn test_record(test_record: TestRecord2) -> TestRecord2;

        pub fn test_record_ref(test_record: &TestRecord2) -> TestRecord2;

        pub fn get_drop_count() -> i32;
    }
}
