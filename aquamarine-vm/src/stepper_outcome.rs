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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct RawStepperOutcome {
    pub ret_code: i32,
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StepperOutcome {
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepperError {
    /// Errors occurred while parsing aqua script in the form of S expressions.
    SExprParseError(String),

    /// Errors occurred on aqua data deserialization.
    DataDeserializationError(String),

    /// Errors occurred on aqua data serialization.
    DataSerializationError(String),

    /// Errors occurred while parsing function arguments of an expression.
    FuncArgsSerdeError(String),

    /// Errors occurred while parsing returned by call_service value.
    CallServiceSerdeError(String),

    /// Indicates that environment variable with name CURRENT_PEER_ID isn't set.
    CurrentPeerIdEnvError(String),

    /// Semantic errors in instructions.
    InstructionError(String),

    /// An error is occurred while calling local service via call_service.
    LocalServiceError(String),

    /// Value for such name isn't presence in data.
    VariableNotFound(String),

    /// Multiple values for such name found.
    MultipleVariablesFound(String),

    /// Value with such path wasn't found in data with such error.
    VariableNotInJsonPath(String),

    /// Value for such name isn't presence in data.
    IncompatibleJValueType(String),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),

    /// Fold state wasn't found for such iterator name.
    FoldStateNotFound(String),

    /// Multiple fold states found for such iterator name.
    MultipleFoldStates(String),

    /// Expected evidence state of different type.
    InvalidEvidenceState(String),

    /// Related to such ret_code that doesn't have match with current StepperError.
    UnknownError(String),

    /// Errors occurred on call evidence deserialization.
    CallEvidenceDeserializationError(String),

    /// Errors occurred on call evidence serialization.
    CallEvidenceSerializationError(String),

    /// Errors occurred when reserved keyword is used for variable name.
    ReservedKeywordError(String),

    /// Errors occurred when previous and current evidence states are incompatible.
    IncompatibleEvidenceStates(String),

    /// Errors occurred when previous and current call results are incompatible.
    IncompatibleCallResults(String),

    /// Errors occurred when evidence path contains less elements then corresponding Par has.
    EvidencePathTooSmall(String),
}

impl Error for StepperError {}

impl std::fmt::Display for StepperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            StepperError::SExprParseError(err_msg) => write!(f, "{}", err_msg),
            StepperError::DataDeserializationError(err_msg) => write!(f, "{}", err_msg),
            StepperError::DataSerializationError(err_msg) => write!(f, "{}", err_msg),
            StepperError::FuncArgsSerdeError(err_msg) => write!(f, "{}", err_msg),
            StepperError::CallServiceSerdeError(err_msg) => write!(f, "{}", err_msg),
            StepperError::CurrentPeerIdEnvError(err_msg) => write!(f, "{}", err_msg),
            StepperError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            StepperError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            StepperError::VariableNotFound(err_msg) => write!(f, "{}", err_msg),
            StepperError::MultipleVariablesFound(err_msg) => write!(f, "{}", err_msg),
            StepperError::VariableNotInJsonPath(err_msg) => write!(f, "{}", err_msg),
            StepperError::IncompatibleJValueType(err_msg) => write!(f, "{}", err_msg),
            StepperError::MultipleValuesInJsonPath(err_msg) => write!(f, "{}", err_msg),
            StepperError::FoldStateNotFound(err_msg) => write!(f, "{}", err_msg),
            StepperError::MultipleFoldStates(err_msg) => write!(f, "{}", err_msg),
            StepperError::InvalidEvidenceState(err_msg) => write!(f, "{}", err_msg),
            StepperError::CallEvidenceDeserializationError(err_msg) => write!(f, "{}", err_msg),
            StepperError::CallEvidenceSerializationError(err_msg) => write!(f, "{}", err_msg),
            StepperError::ReservedKeywordError(err_msg) => write!(f, "{}", err_msg),
            StepperError::IncompatibleEvidenceStates(err_msg) => write!(f, "{}", err_msg),
            StepperError::IncompatibleCallResults(err_msg) => write!(f, "{}", err_msg),
            StepperError::EvidencePathTooSmall(err_msg) => write!(f, "{}", err_msg),
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
            2 => to_vm_error!(DataDeserializationError),
            3 => to_vm_error!(DataSerializationError),
            4 => to_vm_error!(FuncArgsSerdeError),
            5 => to_vm_error!(CallServiceSerdeError),
            6 => to_vm_error!(CurrentPeerIdEnvError),
            7 => to_vm_error!(InstructionError),
            8 => to_vm_error!(LocalServiceError),
            9 => to_vm_error!(VariableNotFound),
            10 => to_vm_error!(MultipleVariablesFound),
            11 => to_vm_error!(VariableNotInJsonPath),
            12 => to_vm_error!(IncompatibleJValueType),
            13 => to_vm_error!(MultipleValuesInJsonPath),
            14 => to_vm_error!(FoldStateNotFound),
            15 => to_vm_error!(MultipleFoldStates),
            16 => to_vm_error!(CallEvidenceDeserializationError),
            17 => to_vm_error!(CallEvidenceSerializationError),
            18 => to_vm_error!(ReservedKeywordError),
            19 => to_vm_error!(IncompatibleEvidenceStates),
            20 => to_vm_error!(IncompatibleCallResults),
            21 => to_vm_error!(EvidencePathTooSmall),
            _ => to_vm_error!(UnknownError),
        }
    }
}
