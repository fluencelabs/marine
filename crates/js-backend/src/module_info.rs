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
        let module = walrus::ModuleConfig::new()
            .parse(wasm)
            .map_err(|e| ModuleCreationError::Other(anyhow!(e)))?;

        let default_ids = IdsToIndices::default();

        let custom_sections = module
            .customs
            .iter()
            .map(|(_, section)| {
                (
                    section.name().to_string(),
                    section.data(&default_ids).to_vec(),
                )
            })
            .collect::<MultiMap<String, Vec<u8>>>();

        let exports = module
            .exports
            .iter()
            .map(|export| {
                let our_export = match export.item {
                    ExportItem::Function(func_id) => {
                        let func = module.funcs.get(func_id);
                        let ty_id = func.ty();
                        let ty = module.types.get(ty_id);
                        let sig = sig_from_walrus_ty(ty);
                        Export::Function(sig)
                    }
                    ExportItem::Table(_) => Export::Table,
                    ExportItem::Memory(_) => Export::Memory,
                    ExportItem::Global(_) => Export::Global,
                };

                (export.name.clone(), our_export)
            })
            .collect::<HashMap<String, Export>>();

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
