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

/// Application binary interface of a Frank module. Different module could use such scheme for
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
pub trait ModuleABI {
    /// Allocates a region of memory inside a module. Used for passing argument inside the module.
    ///   size a size of allocated memory region
    ///   return a pointer to the allocated memory region
    fn allocate(&mut self, size: i32) -> i32;

    /// Deallocates previously allocated memory region.
    ///   ptr a pointer to the previously allocated memory region
    //    size a size of the previously allocated memory region
    fn deallocate(&mut self, ptr: i32, size: i32);

    /// Calls the main entry point of a module called invoke.
    ///   ptr a pointer to the supplied request
    ///   size a size of the supplied request
    ///   return a pointer to the struct contains result_size and result
    fn invoke(&mut self, ptr: i32, size: i32) -> i32;

    /// Stores one given byte on provided address.
    ///   ptr a address at which the needed byte is located
    ///   return the byte at the given address
    fn load(&self, ptr: i32) -> i32;

    /// Loads one bytes from provided address.
    ///   ptr a address where byte should be stored
    //    value a byte to be stored
    fn store(&mut self, ptr: i32, value: i32);
}
