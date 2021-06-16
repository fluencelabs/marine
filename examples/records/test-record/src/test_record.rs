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
