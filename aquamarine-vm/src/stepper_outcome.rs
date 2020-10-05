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

// This file is an adapted copy of the StepperOutcome structure and stepper errors.
// Maybe it is better to depend on aquamarine when it become public.

use crate::AquamarineVMError;

use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug)]
pub(crate) struct RawStepperOutcome {
    pub ret_code: i32,
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

#[derive(Debug)]
pub struct StepperOutcome {
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

#[derive(Debug)]
pub enum StepperError {
    /// Errors occurred while parsing aqua script in the form of S expressions.
    SExprParseError(String),

    /// Errors occurred while parsing supplied data.
    DataParseError(String),

    /// Indicates that environment variable with name CURRENT_PEER_ID isn't set.
    CurrentPeerIdNotSet(String),

    /// Semantic errors in instructions.
    InstructionError(String),

    /// Semantic errors in instructions.
    LocalServiceError(String),

    /// Value with such name isn't presence in data.
    VariableNotFound(String),

    /// Value with such path wasn't found in data.
    VariableNotInJsonPath(String),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),

    /// Related to such ret_code that doesn't have match with current StepperError.
    UnknownError(String),
}

impl Error for StepperError {}

impl std::fmt::Display for StepperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            StepperError::SExprParseError(err_msg) => write!(f, "{}", err_msg),
            StepperError::DataParseError(err_msg) => write!(f, "{}", err_msg),
            StepperError::CurrentPeerIdNotSet(err_msg) => write!(f, "{}", err_msg),
            StepperError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            StepperError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            StepperError::VariableNotFound(err_msg) => write!(f, "{}", err_msg),
            StepperError::VariableNotInJsonPath(err_msg) => write!(f, "{}", err_msg),
            StepperError::MultipleValuesInJsonPath(err_msg) => write!(f, "{}", err_msg),
            StepperError::UnknownError(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

impl TryFrom<RawStepperOutcome> for StepperOutcome {
    type Error = AquamarineVMError;

    fn try_from(raw_outcome: RawStepperOutcome) -> Result<Self, Self::Error> {
        macro_rules! to_vm_error {
            ($error_variant:ident) => {
                Err(AquamarineVMError::StepperError(
                    StepperError::$error_variant(raw_outcome.data),
                ))
            };
        }

        match raw_outcome.ret_code {
            0 => Ok(StepperOutcome {
                data: raw_outcome.data,
                next_peer_pks: raw_outcome.next_peer_pks,
            }),
            1 => to_vm_error!(SExprParseError),
            2 => to_vm_error!(DataParseError),
            3 => to_vm_error!(CurrentPeerIdNotSet),
            4 => to_vm_error!(InstructionError),
            5 => to_vm_error!(LocalServiceError),
            6 => to_vm_error!(VariableNotFound),
            7 => to_vm_error!(VariableNotInJsonPath),
            8 => to_vm_error!(MultipleValuesInJsonPath),
            _ => to_vm_error!(UnknownError),
        }
    }
}
