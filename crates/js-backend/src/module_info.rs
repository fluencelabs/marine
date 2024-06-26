/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine_wasm_backend_traits::FuncSig;
use marine_wasm_backend_traits::ModuleCreationError;
use marine_wasm_backend_traits::WType;
use marine_wasm_backend_traits::impl_utils::MultiMap;

use anyhow::anyhow;
use wasmparser::Parser;
use wasmparser::Payload::*;

use std::collections::HashMap;

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
    pub(crate) fn from_bytes(wasm: &[u8]) -> Result<Self, ModuleCreationError> {
        ModuleInfoParser::new(wasm)?.into_module_info()
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
    pub(crate) fn new(wasm: &'wasm [u8]) -> Result<Self, ModuleCreationError> {
        let mut parser = Self {
            types: <_>::default(),
            functions: <_>::default(),
            exports: <_>::default(),
            custom_sections: <_>::default(),
        };

        parser.parse(wasm)?;

        Ok(parser)
    }

    pub(crate) fn into_module_info(self) -> Result<ModuleInfo, ModuleCreationError> {
        let exports = self.extract_exports()?;
        let custom_sections = self.extract_custom_sections();
        Ok(ModuleInfo {
            exports,
            custom_sections,
        })
    }

    fn parse(&mut self, wasm: &'wasm [u8]) -> Result<(), ModuleCreationError> {
        let parser = Parser::new(0);
        for payload in parser.parse_all(wasm) {
            match payload.map_err(transform_err)? {
                TypeSection(types) => {
                    self.types.reserve(types.count() as usize);
                    for ty in types.into_iter() {
                        let ty = ty.map_err(transform_err)?;
                        let sig = match ty.structural_type {
                            wasmparser::StructuralType::Func(func_type) => {
                                Some(sig_from_wasmparser_ty(&func_type))
                            }
                            _ => None,
                        };

                        self.types.push(sig)
                    }
                }
                ImportSection(imports) => {
                    for import in imports {
                        let import = import.map_err(transform_err)?;
                        if let wasmparser::TypeRef::Func(idx) = import.ty {
                            self.functions.push(idx)
                        }
                    }
                }
                FunctionSection(functions) => {
                    self.functions.reserve(functions.count() as usize);
                    for function in functions {
                        self.functions
                            .push(function.map_err(|e| ModuleCreationError::Other(anyhow!(e)))?);
                    }
                }
                ExportSection(exports) => {
                    self.exports.reserve(exports.count() as usize);
                    for export in exports {
                        self.exports.push(export.map_err(transform_err)?)
                    }
                }
                CustomSection(reader) => self.custom_sections.push((reader.name(), reader.data())),
                _ => {}
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
        self.custom_sections
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

fn transform_err(error: wasmparser::BinaryReaderError) -> ModuleCreationError {
    ModuleCreationError::Other(anyhow!(error))
}
