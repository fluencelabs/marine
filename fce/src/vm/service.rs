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

use crate::vm::config::Config;
use crate::vm::errors::FCEError;
use crate::vm::module::fce_result::FCEResult;

use sha2::digest::generic_array::GenericArray;

/// Describes a service behaviour in the Fluence network.
pub trait FCEService {
    /// Invokes a module supplying byte array and expecting byte array with some outcome back.
    fn invoke(&mut self, module_name: &str, argument: &[u8]) -> Result<FCEResult, FCEError>;

    /// Registers new module in the FCE Service.
    fn register_module<S>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: Config,
    ) -> Result<(), FCEError>
    where
        S: Into<String>;

    /// Unregisters previously registered module.
    fn unregister_module(&mut self, module_name: &str) -> Result<(), FCEError>;

    /// Computes hash of the internal modules state.
    fn compute_state_hash(
        &mut self,
    ) -> GenericArray<u8, <sha2::Sha256 as sha2::digest::FixedOutput>::OutputSize>;
}
