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
