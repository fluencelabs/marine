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
use std::collections::HashSet;

/// Strategies for module loading.
pub enum ModulesLoadStrategy<'a> {
    /// Try to load all files in a given directory
    #[allow(dead_code)]
    All,
    /// Try to load only files contained in the set
    Named(&'a HashSet<String>),
    /// In a given directory, try to load all files ending with .wasm
    WasmOnly,
}

impl<'a> ModulesLoadStrategy<'a> {
    #[inline]
    /// Returns true if `module` should be loaded.
    pub fn should_load(&self, module: &Path) -> bool {
        match self {
            ModulesLoadStrategy::All => true,
            ModulesLoadStrategy::Named(set) => set.contains(module.to_string_lossy().as_ref()),
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
    pub fn missing_modules<'s>(&self, loaded: impl Iterator<Item = &'s String>) -> Vec<&'s String> {
        match self {
            ModulesLoadStrategy::Named(set) => loaded.fold(vec![], |mut vec, module| {
                if !set.contains(module) {
                    vec.push(module)
                }
                vec
            }),
            _ => <_>::default(),
        }
    }

    #[inline]
    pub fn extract_module_name(&self, module: String) -> String {
        match self {
            ModulesLoadStrategy::WasmOnly => {
                let path: &Path = module.as_ref();
                path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or(module)
            }
            _ => module,
        }
    }
}
