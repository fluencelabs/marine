/*
 * Copyright 2023 Fluence Labs Limited
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

use marine_wasm_backend_traits::FuncSig;
use marine_wasm_backend_traits::ModuleCreationError;
use marine_wasm_backend_traits::WType;
use marine_wasm_backend_traits::impl_utils::MultiMap;

use anyhow::anyhow;
use walrus::IdsToIndices;
use walrus::ExportItem;
use walrus::ValType;

use wasmparser::{Parser, Chunk, Payload::*};

use std::collections::HashMap;
use std::fs::read;

#[derive(Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) custom_sections: MultiMap<String, Vec<u8>>,
    pub(crate) exports: HashMap<String, Export>,
}

#[derive(Clone)]
pub(crate) enum Export {
    Function(FuncSig),
    Memory,
    Table,
    Global,
}

impl ModuleInfo {
    #[tracing::instrument(level = "trace", skip_all)]
    pub(crate) fn from_bytes(wasm: &[u8]) -> Result<Self, ModuleCreationError> {
        ModuleInfoParser::new(wasm)?
            .into_module_info()
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub(crate) fn from_bytes_walrus(wasm: &[u8]) -> Result<Self, ModuleCreationError> {
        let module = {
            let _span = tracing::trace_span!("walrus::ModuleConfig::parse plain").entered();
            walrus::ModuleConfig::new()
                .parse(wasm)
                .map_err(|e| ModuleCreationError::Other(anyhow!(e)))?
        };

        let default_ids = IdsToIndices::default();

        let custom_sections = {
            let _span = tracing::trace_span!("extract custom sections").entered();
            module
                .customs
                .iter()
                .map(|(_, section)| {
                    (
                        section.name().to_string(),
                        section.data(&default_ids).to_vec(),
                    )
                })
                .collect::<MultiMap<String, Vec<u8>>>()
        };

        let exports = {
            let _span = tracing::trace_span!("extract exports").entered();
            module
                .exports
                .iter()
                .map(|export| {
                    let our_export = match export.item {
                        ExportItem::Function(func_id) => {
                            let func = module.funcs.get(func_id);
                            let ty_id = func.ty();
                            let ty = module.types.get(ty_id);
                            let signature = sig_from_walrus_ty(ty);
                            Export::Function(signature)
                        }
                        ExportItem::Table(_) => Export::Table,
                        ExportItem::Memory(_) => Export::Memory,
                        ExportItem::Global(_) => Export::Global,
                    };

                    (export.name.clone(), our_export)
                })
                .collect::<HashMap<String, Export>>()
        };

        Ok(ModuleInfo {
            custom_sections,
            exports,
        })
    }
}

fn sig_from_walrus_ty(ty: &walrus::Type) -> FuncSig {
    let params = ty
        .params()
        .iter()
        .map(wtype_from_walrus_val)
        .collect::<Vec<_>>();
    let results = ty
        .results()
        .iter()
        .map(wtype_from_walrus_val)
        .collect::<Vec<_>>();
    FuncSig::new(params, results)
}

fn wtype_from_walrus_val(val: &walrus::ValType) -> WType {
    match val {
        ValType::I32 => WType::I32,
        ValType::I64 => WType::I64,
        ValType::F32 => WType::F32,
        ValType::F64 => WType::F64,
        ValType::V128 => WType::V128,
        ValType::Externref => WType::ExternRef,
        ValType::Funcref => WType::FuncRef,
    }
}

struct ModuleInfoParser<'wasm> {
    /// all types met
    types: Vec<Option<FuncSig>>,
    /// indexes in `types` field -- function signatures
    functions: Vec<u32>,
    /// export names + indexes in `functions` field
    exports: Vec<wasmparser::Export<'wasm>>,
    /// names and data
    custom_sections: Vec<(&'wasm str, &'wasm [u8])>,
}

impl<'wasm> ModuleInfoParser<'wasm> {
    pub(crate) fn new(wasm: &'wasm[u8]) -> Result<Self, ModuleCreationError> {
        let mut parser = Self {
            types: <_>::default(),
            functions: <_>::default(),
            exports: <_>::default(),
            custom_sections: <_>::default(),
        };

        parser.parse(wasm)?;

        Ok(parser)
    }

    pub(crate) fn into_module_info(&self) -> Result<ModuleInfo, ModuleCreationError> {
        let exports = self.extract_exports()?;
        let custom_sections = self.extract_custom_sections();
        Ok(ModuleInfo { exports, custom_sections })
    }

    fn parse(&mut self, wasm: &'wasm [u8]) -> Result<(), ModuleCreationError> {
        let parser = Parser::new(0);
        for payload in parser.parse_all(&wasm) {
            match payload.map_err(|e| ModuleCreationError::Other(anyhow!(e)))? {
                // Sections for WebAssembly modules
                Version { .. } => { /* ... */ }
                TypeSection(types) => {
                    self.types.reserve(types.count() as usize);
                    for (idx, ty) in types.into_iter().enumerate() {
                        let ty = ty.map_err(|e| ModuleCreationError::Other(anyhow!(e)))?;
                        let sig = match ty.structural_type {
                            wasmparser::StructuralType::Func(func_type) => Some(sig_from_wasmparser_ty(&func_type)),
                            _ => None
                        };

                        self.types.push(sig)
                    }
                }
                ImportSection(imports) => {
                    for import in imports {
                        let import = import.map_err(|e| ModuleCreationError::Other(anyhow!(e)))?;
                        if let wasmparser::TypeRef::Func(idx) = import.ty {
                            self.functions.push(idx)
                        }
                    }
                }
                FunctionSection(functions) => {
                    self.functions.reserve(functions.count() as usize);
                    for function in functions {
                        self.functions.push(function.map_err(|e| ModuleCreationError::Other(anyhow!(e)))?);
                    }
                }
                TableSection(_) => { /* ... */ }
                MemorySection(_) => { /* ... */ }
                TagSection(_) => { /* ... */ }
                GlobalSection(_) => { /* ... */ }
                ExportSection(exports) => {
                    self.exports.reserve(exports.count() as usize);
                    for export in exports {
                        self.exports.push(export.map_err(|e| ModuleCreationError::Other(anyhow!(e)))?)
                    }
                }
                StartSection { .. } => { /* ... */ }
                ElementSection(_) => { /* ... */ }
                DataCountSection { .. } => { /* ... */ }
                DataSection(_) => { /* ... */ }

                // Here we know how many functions we'll be receiving as
                // `CodeSectionEntry`, so we can prepare for that, and
                // afterwards we can parse and handle each function
                // individually.
                CodeSectionStart { .. } => { /* ... */ }
                CodeSectionEntry(body) => {
                    // here we can iterate over `body` to parse the function
                    // and its locals
                }

                // Sections for WebAssembly components
                ModuleSection { .. } => { /* ... */ }
                InstanceSection(_) => { /* ... */ }
                CoreTypeSection(_) => { /* ... */ }
                ComponentSection { .. } => { /* ... */ }
                ComponentInstanceSection(_) => { /* ... */ }
                ComponentAliasSection(_) => { /* ... */ }
                ComponentTypeSection(_) => { /* ... */ }
                ComponentCanonicalSection(_) => { /* ... */ }
                ComponentStartSection { .. } => { /* ... */ }
                ComponentImportSection(_) => { /* ... */ }
                ComponentExportSection(_) => { /* ... */ }

                CustomSection(reader) => self.custom_sections.push((reader.name(), reader.data())),

                // most likely you'd return an error here
                UnknownSection { id, .. } => { /* ... */ }

                // Once we've reached the end of a parser we either resume
                // at the parent parser or the payload iterator is at its
                // end and we're done.
                End(_) => {}
            }
        }

        Ok(())
    }

    fn extract_exports(&self) -> Result<HashMap<String, Export>, ModuleCreationError> {
        self.exports
            .iter()
            .map(|export| {
                let name = export.name.to_string();
                let marine_export = match export.kind {
                    wasmparser::ExternalKind::Func => {
                        let sig = self
                            .functions
                            .get(export.index as usize)
                            .and_then(|ty_index| self.types.get(*ty_index as usize))
                            .and_then(|sig| sig.as_ref())
                            .ok_or_else(|| ModuleCreationError::Other(anyhow!("Function export references non-function type, or the module is malformed")))?;

                        Export::Function(sig.clone())
                    },
                    wasmparser::ExternalKind::Table => Export::Table,
                    wasmparser::ExternalKind::Memory => Export::Memory,
                    wasmparser::ExternalKind::Global => Export::Global,
                    wasmparser::ExternalKind::Tag => return Err(ModuleCreationError::Other(anyhow!("unknown extern type: Tag"))),
                };


                Ok((name, marine_export))
            })
            .collect()
    }

    fn extract_custom_sections(&self) -> MultiMap<String, Vec<u8>> {
        self
            .custom_sections
            .iter()
            .map(|(name, data)| (name.to_string(), data.to_vec()))
            .collect()
    }
}

fn sig_from_wasmparser_ty(ty: &wasmparser::FuncType) -> FuncSig {
    let params = ty
        .params()
        .iter()
        .map(wtype_from_wasmparser_val)
        .collect::<Vec<_>>();
    let results = ty
        .results()
        .iter()
        .map(wtype_from_wasmparser_val)
        .collect::<Vec<_>>();
    FuncSig::new(params, results)
}

fn wtype_from_wasmparser_val(val: &wasmparser::ValType) -> WType {
    match val {
        wasmparser::ValType::I32 => WType::I32,
        wasmparser::ValType::I64 => WType::I64,
        wasmparser::ValType::F32 => WType::F32,
        wasmparser::ValType::F64 => WType::F64,
        wasmparser::ValType::V128 => WType::V128,
        wasmparser::ValType::Ref(_) => WType::ExternRef, // TODO maybe return an error as it is not supported?
    }
}
