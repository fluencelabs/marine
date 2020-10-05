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

use crate::stepper_outcome::StepperError;
use fluence_faas::FaaSError;

use std::error::Error;

#[derive(Debug)]
pub enum AquamarineVMError {
    /// FaaS errors.
    FaaSError(FaaSError),

    /// Aquamarine result deserialization errors.
    AquamarineResultError(String),

    /// Errors related to stepper execution.
    StepperError(StepperError),
}

impl Error for AquamarineVMError {}

impl std::fmt::Display for AquamarineVMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineVMError::FaaSError(err) => write!(f, "{}", err),
            AquamarineVMError::AquamarineResultError(err_msg) => write!(f, "{}", err_msg),
            AquamarineVMError::StepperError(err) => write!(f, "{}", err),
        }
    }
}

impl From<FaaSError> for AquamarineVMError {
    fn from(err: FaaSError) -> Self {
        AquamarineVMError::FaaSError(err)
    }
}

impl From<StepperError> for AquamarineVMError {
    fn from(err: StepperError) -> Self {
        AquamarineVMError::StepperError(err)
    }
}

impl From<std::convert::Infallible> for AquamarineVMError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
