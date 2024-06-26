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

fn main() {}

#[marine]
pub fn greeting(name: &str) -> Vec<String> {
    let donkey_greeting = donkey::greeting(name);
    let shrek_greeting = format!("Shrek: hi, {}", name);

    vec![shrek_greeting, donkey_greeting]
}

mod donkey {
    use super::*;

    #[marine]
    #[module_import("donkey")]
    extern "C" {
        pub fn greeting(name: &str) -> String;
    }
}
