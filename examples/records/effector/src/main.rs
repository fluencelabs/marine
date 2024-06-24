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

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use test_record::TestRecord;

module_manifest!();

pub fn main() {}

#[marine]
pub fn mutate_struct(mut test_record: test_record::TestRecord) -> TestRecord {
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
