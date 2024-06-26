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
#![allow(clippy::all)]
#![allow(non_snake_case)]

use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn allocate_single_module_single_piece(size: i64) -> u32 {
    let addr = Vec::<u8>::with_capacity(size as usize).leak().as_ptr();
    unsafe { std::mem::transmute::<*const u8, usize>(addr) as u32 }
}

#[marine]
pub fn allocate_single_module_64KB_pieces(n_pieces: u32) -> u32 {
    let mut acc: u32 = 0;

    for _ in 0..n_pieces {
        unsafe {
            let addr = Box::leak(Box::new([0u8; 1024 * 64]));
            acc ^= std::mem::transmute::<*const u8, usize>(addr.as_ptr()) as u32;
        }
    }

    acc
}

#[marine]
pub fn allocate_two_modules_single_piece(size: i64) -> u32 {
    let first = allocate_single_module_single_piece(size);
    let second = effector::allocate_single_module_single_piece(size);
    first ^ second
}

#[marine]
pub fn allocate_two_modules_64KB_pieces(n_pieces: u32) -> u32 {
    let first = allocate_single_module_64KB_pieces(n_pieces);
    let second = effector::allocate_single_module_64KB_pieces(n_pieces);
    first ^ second
}

mod effector {
    use marine_rs_sdk::marine;

    #[marine]
    #[module_import("memory_limiting_effector")]
    extern "C" {
        pub fn allocate_single_module_single_piece(size: i64) -> u32;

        pub fn allocate_single_module_64KB_pieces(n_pieces: u32) -> u32;
    }
}
