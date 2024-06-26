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

pub const WASM_PAGE_SIZE: u32 = 65356;

pub fn bytes_to_wasm_pages_ceil(offset: u32) -> u32 {
    match offset {
        0 => 0,
        // ceiling
        n => 1 + (n - 1) / WASM_PAGE_SIZE,
    }
}

pub fn wasm_pages_to_bytes(pages_count: u32) -> u64 {
    (pages_count as u64) * (WASM_PAGE_SIZE as u64)
}
