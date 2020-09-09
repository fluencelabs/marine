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
use crate::IType;

use fce::FCE;
use fluence_sdk_main::CallParameters;
use wasmer_wit::vec1::Vec1;
use wasmer_wit::types::RecordFieldType;

use std::cell::RefCell;
use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

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

pub struct FluenceFaaS {
    /// The Fluence Compute Engine instance.
    fce: FCE,

    /// Parameters of call accessible by Wasm modules.
    call_parameters: Rc<RefCell<CallParameters>>,
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
        let call_parameters = Rc::new(RefCell::new(<_>::default()));

        for (module_name, module_config) in config.modules_config {
            let module_bytes = modules.remove(&module_name).ok_or_else(|| {
                FaaSError::InstantiationError(format!(
                    "module with name {} is specified in config, but not found in provided modules",
                    module_name
                ))
            })?;
            let fce_module_config =
                crate::misc::make_fce_config(Some(module_config), call_parameters.clone())?;
            fce.load_module(module_name, &module_bytes, fce_module_config)?;
        }

        Ok(Self {
            fce,
            call_parameters,
        })
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
    pub fn call_with_ivalues<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        args: &[IValue],
        call_parameters: fluence_sdk_main::CallParameters,
    ) -> Result<Vec<IValue>> {
        self.call_parameters.replace(call_parameters);

        self.fce
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_json<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        json_args: serde_json::Value,
        call_parameters: fluence_sdk_main::CallParameters,
    ) -> Result<Vec<IValue>> {
        use serde_json::Value;

        self.call_parameters.replace(call_parameters);
        let module_name = module_name.as_ref();
        let func_name = func_name.as_ref();

        let iargs = {
            let mut func_signatures = self.fce.module_interface(module_name)?;
            let func_signature = func_signatures
                .find(|sign| sign.name == func_name)
                .ok_or_else(|| FaaSError::MissingFunctionError(func_name.to_string()))?;
            let record_types = self
                .fce
                .module_record_types(module_name)?
                .map(|ty| (&ty.name, &ty.fields))
                .collect::<HashMap<_, _>>();

            match json_args {
                Value::Object(json_map) => Self::json_map_to_ivalues(
                    json_map,
                    func_signature
                        .arguments
                        .iter()
                        .map(|arg| (&arg.name, &arg.ty)),
                    &record_types,
                )?,
                Value::Array(json_array) => Self::json_array_to_ivalues(
                    json_array,
                    func_signature.arguments.iter().map(|arg| &arg.ty),
                    &record_types,
                )?,
                _ => unimplemented!(),
            }
        };

        self.fce
            .call(module_name, func_name, &iargs)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> FaaSInterface<'_> {
        let record_types = self.fce.record_types().collect::<Vec<_>>();

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
                                arguments: f.arguments,
                                output_types: f.output_types,
                            },
                        )
                    })
                    .collect();
                (name, signatures)
            })
            .collect();

        FaaSInterface {
            record_types,
            modules,
        }
    }

    fn json_map_to_ivalues<'a, 'b>(
        mut json_map: serde_json::Map<String, serde_json::Value>,
        signature: impl Iterator<Item = (&'a String, &'a IType)>,
        record_types: &'b HashMap<&'b String, &'b Vec1<RecordFieldType>>,
    ) -> Result<Vec<IValue>> {
        let mut iargs = Vec::new();

        for (arg_name, arg_type) in signature {
            let json_value = json_map
                .remove(arg_name)
                .ok_or_else(|| FaaSError::MissingArgumentError(arg_name.clone()))?;
            let iarg = Self::json_value_to_ivalue(json_value, arg_type, record_types)?;
            iargs.push(iarg);
        }

        if !json_map.is_empty() {
            return Err(FaaSError::JsonArgumentsDeserializationError(format!(
                "function requires {} arguments, {} provided",
                iargs.len(),
                iargs.len() + json_map.len()
            )));
        }

        Ok(iargs)
    }

    fn json_array_to_ivalues<'a, 'b>(
        mut json_array: Vec<serde_json::Value>,
        signature: impl Iterator<Item = &'a IType>,
        record_types: &'b HashMap<&'b String, &'b Vec1<RecordFieldType>>,
    ) -> Result<Vec<IValue>> {
        let mut iargs = Vec::new();

        for (arg_id, arg_type) in signature.enumerate() {
            let json_value = json_array.remove(arg_id);
            let iarg = Self::json_value_to_ivalue(json_value, arg_type, record_types)?;
            iargs.push(iarg);
        }

        if !json_array.is_empty() {
            return Err(FaaSError::JsonArgumentsDeserializationError(format!(
                "function requires {} arguments, {} provided",
                iargs.len(),
                iargs.len() + json_array.len()
            )));
        }

        Ok(iargs)
    }

    fn json_value_to_ivalue(
        json_value: serde_json::Value,
        ty: &IType,
        record_types: &HashMap<&String, &Vec1<RecordFieldType>>,
    ) -> Result<IValue> {
        match ty {
            IType::S8 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::S8(value))
            }
            IType::S16 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::S16(value))
            }
            IType::S32 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::S32(value))
            }
            IType::S64 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::S64(value))
            }
            IType::U8 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::U8(value))
            }
            IType::U16 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::U16(value))
            }
            IType::U32 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::U32(value))
            }
            IType::U64 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::U64(value))
            }
            IType::F32 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::F32(value))
            }
            IType::F64 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::F64(value))
            }
            IType::String => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::String(value))
            }
            IType::ByteArray => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::ByteArray(value))
            }
            IType::I32 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::I32(value))
            }
            IType::I64 => {
                let value = serde_json::from_value(json_value)
                    .map_err(FaaSError::ArgumentDeserializationError)?;
                Ok(IValue::I64(value))
            }
            IType::Record(ty_name) => {
                let value = Self::json_record_type_to_ivalue(json_value, ty_name, &record_types)?;
                Ok(IValue::Record(value))
            }
            IType::Anyref => Err(FaaSError::JsonArgumentsDeserializationError(String::from(
                "anyref interface-type is unsupported now",
            ))),
        }
    }

    #[allow(clippy::ptr_arg)]
    fn json_record_type_to_ivalue(
        json_value: serde_json::Value,
        itype_name: &String,
        record_types: &HashMap<&String, &Vec1<RecordFieldType>>,
    ) -> Result<Vec1<IValue>> {
        use serde_json::Value;

        let record_type = record_types.get(itype_name).ok_or_else(|| {
            FaaSError::JsonArgumentsDeserializationError(format!(
                "record with type `{}` wasn't found",
                itype_name
            ))
        })?;

        match json_value {
            Value::Object(json_map) => Ok(Vec1::new(Self::json_map_to_ivalues(
                json_map,
                record_type.iter().map(|field| (&field.name, &field.ty)),
                record_types,
            )?)
            .unwrap()),
            Value::Array(json_array) => Ok(Vec1::new(Self::json_array_to_ivalues(
                json_array,
                record_type.iter().map(|field| (&field.ty)),
                record_types,
            )?)
            .unwrap()),
            _ => Err(FaaSError::JsonArgumentsDeserializationError(format!(
                "record with type `{}` should be encoded as array or map of fields",
                itype_name
            ))),
        }
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

        let fce_module_config = crate::misc::make_fce_config(config, self.call_parameters.clone())?;
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
