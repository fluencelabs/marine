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

#[marine]
pub struct TestRecord {
    pub field_0: bool,
    pub field_1: i8,
    pub field_2: i16,
    pub field_3: i32,
    pub field_4: i64,
    pub field_5: u8,
    pub field_6: u16,
    pub field_7: u32,
    pub field_8: u64,
    pub field_9: f32,
    pub field_10: f64,
    pub field_11: String,
    pub field_12: Vec<u8>,
}
