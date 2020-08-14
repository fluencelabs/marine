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

use crate::misc::ModulesConfig;
use crate::faas_interface::FaaSFunctionSignature;
use crate::faas_interface::FaaSInterface;
use crate::FaaSError;
use crate::Result;
use crate::IValue;

use fce::FCE;

use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf, Path};

// TODO: remove and use mutex instead
unsafe impl Send for FluenceFaaS {}

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
            ModulesLoadStrategy::WasmOnly => module.extension().map_or(false, |e| e == ".wasm"),
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

pub struct FluenceFaaS {
    fce: FCE,
}

impl FluenceFaaS {
    /// Creates FaaS from config on filesystem.
    pub fn new<P: Into<PathBuf>>(config_file_path: P) -> Result<Self> {
        let config = crate::misc::load_config(config_file_path.into())?;
        Self::with_raw_config(config)
    }

    /// Creates FaaS from config deserialized from TOML.
    pub fn with_raw_config<C>(config: C) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_dir
            .as_ref()
            .map_or(Ok(HashMap::new()), |dir| {
                Self::load_modules(dir, ModulesLoadStrategy::WasmOnly)
            })?;

        Self::with_modules::<ModulesConfig>(modules, config)
    }

    /// Creates FaaS with given modules.
    pub fn with_modules<C>(mut modules: HashMap<String, Vec<u8>>, config: C) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        FaaSError: From<C::Error>,
    {
        let mut fce = FCE::new();
        let config = config.try_into()?;

        for (module_name, module_config) in config.modules_config {
            let module_bytes = modules.remove(&module_name).ok_or_else(|| {
                FaaSError::InstantiationError(format!(
                    "module with name {} is specified in config, but not found in provided modules",
                    module_name
                ))
            })?;
            let fce_module_config = crate::misc::make_fce_config(Some(module_config))?;
            fce.load_module(module_name, &module_bytes, fce_module_config)?;
        }

        for (name, bytes) in modules {
            let module_config = config.default_modules_config.clone();
            let fce_module_config = crate::misc::make_fce_config(module_config)?;
            fce.load_module(name.clone(), &bytes, fce_module_config)?;
        }

        Ok(Self { fce })
    }

    /// Searches for modules in `config.modules_dir`, loads only those in the `names` set
    pub fn with_module_names<C>(names: &HashSet<String>, config: C) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_dir
            .as_ref()
            .map_or(Ok(HashMap::new()), |dir| {
                Self::load_modules(dir, ModulesLoadStrategy::Named(names))
            })?;

        Self::with_modules::<ModulesConfig>(modules, config)
    }

    /// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
    fn load_modules(
        modules_dir: &str,
        modules: ModulesLoadStrategy<'_>,
    ) -> Result<HashMap<String, Vec<u8>>> {
        use FaaSError::IOError;

        let mut dir_entries =
            fs::read_dir(modules_dir).map_err(|e| IOError(format!("{}: {}", modules_dir, e)))?;

        let loaded = dir_entries.try_fold(HashMap::new(), |mut hash_map, entry| {
            let entry = entry?;
            let path = entry.path();
            // Skip directories
            if path.is_dir() {
                return Ok(hash_map);
            }

            let module_name = path
                .file_name()
                .ok_or_else(|| IOError(format!("No file name in path {:?}", path)))?
                .to_os_string()
                .into_string()
                .map_err(|name| IOError(format!("invalid file name: {:?}", name)))?;

            if modules.should_load(&module_name.as_ref()) {
                let module_bytes = fs::read(path)?;
                let module_name = modules.extract_module_name(module_name);
                if hash_map.insert(module_name, module_bytes).is_some() {
                    return Err(FaaSError::ConfigParseError(String::from(
                        "module {} is duplicated in config",
                    )));
                }
            }

            Ok(hash_map)
        })?;

        if modules.required_modules_len() > loaded.len() {
            let loaded = loaded.iter().map(|(n, _)| n);
            let not_found = modules.missing_modules(loaded);
            return Err(FaaSError::ConfigParseError(format!(
                "the following modules were not found: {:?}",
                not_found
            )));
        }

        Ok(loaded)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        args: &[IValue],
    ) -> Result<Vec<IValue>> {
        self.fce
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> FaaSInterface<'_> {
        let modules = self
            .fce
            .interface()
            .map(|(name, signatures)| {
                let signatures = signatures
                    .iter()
                    .map(|f| {
                        (
                            f.name,
                            FaaSFunctionSignature {
                                input_types: f.input_types,
                                output_types: f.output_types,
                            },
                        )
                    })
                    .collect();
                (name, signatures)
            })
            .collect();

        FaaSInterface { modules }
    }
}

// This API is intended for testing purposes (mostly in FCE REPL)
#[cfg(feature = "raw-module-api")]
impl FluenceFaaS {
    pub fn load_module<S, C>(&mut self, name: S, wasm_bytes: &[u8], config: Option<C>) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<crate::ModuleConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.map(|c| c.try_into()).transpose()?;

        let fce_module_config = crate::misc::make_fce_config(config)?;
        self.fce
            .load_module(name, &wasm_bytes, fce_module_config)
            .map_err(Into::into)
    }

    pub fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<()> {
        self.fce.unload_module(module_name).map_err(Into::into)
    }

    pub fn module_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Result<&wasmer_wasi::state::WasiState> {
        self.fce.module_wasi_state(module_name).map_err(Into::into)
    }
}
