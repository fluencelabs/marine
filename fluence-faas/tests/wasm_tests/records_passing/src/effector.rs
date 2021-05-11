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
pub fn test_record(mut test_record: TestRecord2) -> TestRecord2 {
    test_record.test_record_0 = TestRecord0 { field_0: 1 };

    test_record
}

#[marine]
pub fn test_record_ref(test_record: &mut TestRecord2) -> TestRecord2 {
    test_record.test_record_0 = TestRecord0 { field_0: 1 };

    test_record.clone()
}
