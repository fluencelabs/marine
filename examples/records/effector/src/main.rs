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
use test_record::TestRecord;

pub fn main() {}

#[fce]
pub fn mutate_struct(mut test_record: TestRecord) -> TestRecord {
    test_record.field_0 = true;
    test_record.field_1 = 1;
    test_record.field_2 = 2;
    test_record.field_3 = 3;
    test_record.field_4 = 4;
    test_record.field_5 = 5;
    test_record.field_6 = 6;
    test_record.field_7 = 7;
    test_record.field_8 = 8;
    test_record.field_9 = 9f32;
    test_record.field_10 = 10f64;
    test_record.field_11 = "field_11".to_string();
    test_record.field_12 = vec![0x13, 0x37];

    test_record
}
