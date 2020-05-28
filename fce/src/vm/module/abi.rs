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

use crate::vm::errors::FCEError;

/// Application binary interface of a FCE module. Different module could use such scheme for
/// communicate with each other.
///
/// Given char string req as a request, the general scheme to use this ABI by other module
/// is following:
///
///   1. ptr = allocate(strlen(req)) that returns a pointer to the memory region enough for req
///   2. writes req to the module memory byte-by-byte with store
///   3. res = invoke(ptr, strlen(sql)) to execute the request
///   4. read a result from the res by reading 4 bytes as little-endian result_size
///      and the read result_size bytes as the final result.
///   5. deallocate(res, strlen(sql)) to clean memory.
pub(crate) trait ModuleABI {
    /// Allocates a region of memory inside a module. Used for passing argument inside the module.
    fn allocate(&mut self, size: i32) -> Result<i32, FCEError>;

    /// Deallocates previously allocated memory region.
    fn deallocate(&mut self, ptr: i32, size: i32) -> Result<(), FCEError>;

    /// Calls the main entry point of a module called invoke.
    fn invoke(&mut self, arg_address: i32, arg_size: i32) -> Result<i32, FCEError>;

    /// Stores one byte on given address.
    fn store(&mut self, address: i32, value: i32) -> Result<(), FCEError>;

    /// Loads one byte from given address.
    fn load(&mut self, address: i32) -> Result<i32, FCEError>;
}
