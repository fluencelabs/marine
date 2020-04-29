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

use wasmer_runtime::Func;

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
pub(crate) struct ModuleABI {
    // It is safe to use unwrap() while calling these functions because Option is used here
    // just to allow partially initialization. And all Option fields will contain Some if
    // invoking Frank::new has been succeed.
    /// Allocates a region of memory inside a module. Used for passing argument inside the module.
    pub(crate) allocate: Option<Func<'static, i32, i32>>,

    /// Deallocates previously allocated memory region.
    pub(crate) deallocate: Option<Func<'static, (i32, i32), ()>>,

    /// Calls the main entry point of a module called invoke.
    pub(crate) invoke: Option<Func<'static, (i32, i32), i32>>,

    /// Stores one given byte on provided address.
    pub(crate) store: Option<Func<'static, (i32, i32)>>,

    /// Loads one bytes from provided address.
    pub(crate) load: Option<Func<'static, i32, i32>>,
}
