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

use crate::FCEResult;
use crate::FCEError;
use crate::MINIMAL_SUPPORTED_SDK_VERSION;
use crate::MINIMAL_SUPPORTED_IT_VERSION;

use fce_module_info_parser::sdk_version;

use wasmer_core::Module;

pub(crate) fn check_sdk_version(wasmer_module: &Module) -> FCEResult<()> {
    let module_version = sdk_version::extract_from_wasmer_module(wasmer_module)?;
    let module_version = match module_version {
        Some(module_version) => module_version,
        None => return Err(FCEError::ModuleWithoutVersion),
    };

    MINIMAL_SUPPORTED_SDK_VERSION.with(|required_version| {
        if module_version < *required_version {
            return Err(FCEError::IncompatibleSDKVersions {
                required: required_version.clone(),
                provided: module_version,
            });
        }

        Ok(())
    })
}

pub(crate) fn check_it_version(it_version: &semver::Version) -> FCEResult<()> {
    MINIMAL_SUPPORTED_IT_VERSION.with(|required_version| {
        if it_version < required_version {
            return Err(FCEError::IncompatibleITVersions {
                required: required_version.clone(),
                provided: it_version.clone(),
            });
        }

        Ok(())
    })
}
