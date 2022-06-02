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

use std::path::Path;
use std::collections::{HashMap, HashSet};
use crate::MarineError;
use std::ffi::OsStr;
use std::borrow::Cow;

type ImportName = String;
type FileName = String;

/// Strategies for module loading.
#[derive(Debug, Clone)]
pub enum ModulesLoadStrategy<'a> {
    /// Load all files in a given directory
    #[allow(dead_code)]
    All,
    /// Load only files contained in the set
    /// Correspondence between module file name and import name is crucial for `extract_module_name`
    Named(&'a HashMap<FileName, ImportName>),
    /// In a given directory, try to load all files ending with .wasm
    #[allow(dead_code)]
    WasmOnly,
}

impl<'a> ModulesLoadStrategy<'a> {
    #[inline]
    /// Returns true if `module` should be loaded.
    pub fn should_load(&self, module: &Path) -> bool {
        match self {
            ModulesLoadStrategy::All => true,
            ModulesLoadStrategy::Named(map) => map.contains_key(module.to_string_lossy().as_ref()),
            ModulesLoadStrategy::WasmOnly => module.extension().map_or(false, |e| e == "wasm"),
        }
    }

    #[inline]
    /// Returns the number of modules that must be loaded.
    pub fn required_modules_len(&self) -> usize {
        match self {
            ModulesLoadStrategy::Named(set) => set.len(),
            _ => 0,
        }
    }

    #[inline]
    /// Returns difference between required and loaded modules.
    pub fn missing_modules<'i>(
        &self,
        loaded: impl Iterator<Item = &'i String>,
    ) -> HashSet<&String> {
        match self {
            ModulesLoadStrategy::Named(map) => {
                let set: HashSet<_> = map.keys().collect();
                loaded.fold(set, |mut set, module| {
                    set.remove(module);
                    set
                })
            }
            _ => <_>::default(),
        }
    }

    #[inline]
    pub fn extract_module_name(&self, module_path: &Path) -> Result<String, MarineError> {
        use MarineError::*;

        fn as_str<'a>(
            os_str: Option<&'a OsStr>,
            path: &'a Path,
        ) -> Result<Cow<'a, str>, MarineError> {
            os_str
                .map(|s| s.to_string_lossy())
                .ok_or_else(|| IOError(format!("No file name in path {:?}", path)))
        }

        match self {
            Self::Named(map) => {
                let file_name = as_str(module_path.file_name(), module_path)?;
                // Take import_name from the mapping and return it
                let import_name = map.get(file_name.as_ref());
                let import_name = import_name.ok_or_else(|| NoSuchModule(file_name.to_string()))?;

                Ok(import_name.clone())
            }
            // for other strategies, simply use file name without extension
            _ => Ok(as_str(module_path.file_stem(), module_path)?.to_string()),
        }
    }
}
