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

#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(dead_code)]

use marine_rs_sdk::marine;
use core::cell::RefCell;

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

fn main() {}

#[marine]
pub fn pass_droppable_record(
    record: DroppableRecordTreeConainer,
    records: Vec<DroppableRecordTreeConainer>,
) -> Vec<DroppableRecordTreeConainer> {
    let mut records = records.clone();
    records.push(record.clone());
    records
}

#[marine]
pub fn test_record(mut test_record: TestRecord2) -> TestRecord2 {
    test_record.test_record_0 = TestRecord0 { field_0: 1 };

    test_record
}

#[marine]
pub fn test_record_ref(test_record: &mut TestRecord2) -> TestRecord2 {
    test_record.test_record_0 = TestRecord0 { field_0: 1 };

    test_record.clone()
}

#[marine]
pub fn get_drop_count() -> i32 {
    DROP_COUNT.with(|count| *count.borrow())
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
    Data256KB::default()
}

#[marine]
fn pass_256kb_struct(arg: Data256KB) {}
