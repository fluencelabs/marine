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

use crate::default_export_api_config::*;
use crate::errors::WITGeneratorError;
use crate::instructions_generator::WITGenerator;
use crate::instructions_generator::WITResolver;
use crate::Result;

pub use fluence_sdk_wit::FCEAst;
use wasmer_wit::ast::Interfaces;
use wasmer_wit::IRecordType;
use wasmer_wit::IType;

/// Parse generated by rust-sdk AST types, generate instructions and embed them to Wasm file.
pub fn embed_wit(path: std::path::PathBuf) -> Result<()> {
    let wasm_module = walrus::ModuleConfig::new()
        .parse_file(path.clone())
        .map_err(|e| WITGeneratorError::IOError(format!("{:?} can't be parsed: {:?}", path, e)))?;

    let module_ast = wasm_ast_extractor(&wasm_module)?;
    let interfaces = generate_interfaces(&module_ast)?;

    let wasm_module = fce_wit_parser::delete_wit_section(wasm_module);
    let mut wasm_module = fce_wit_parser::embed_wit(wasm_module, &interfaces);

    wasm_module.emit_wasm_file(path).map_err(|e| {
        WITGeneratorError::IOError(format!("resulted Wasm file can't be emitted: {:?}", e))
    })
}

pub(crate) struct ModuleAST {
    pub(crate) records: Vec<fluence_sdk_wit::AstRecordItem>,
    pub(crate) functions: Vec<fluence_sdk_wit::AstFunctionItem>,
    pub(crate) extern_mods: Vec<fluence_sdk_wit::AstExternModItem>,
}

/// Extract all custom AST types previously embedded by rust-sdk from compiled binary.
fn wasm_ast_extractor(wasm_module: &walrus::Module) -> Result<ModuleAST> {
    use fluence_sdk_wit::*;

    let mut records: Vec<AstRecordItem> = Vec::new();
    let mut functions: Vec<AstFunctionItem> = Vec::new();
    let mut extern_mods: Vec<AstExternModItem> = Vec::new();

    // consider only sections name of that starts with GENERATED_SECTION_PREFIX
    for custom_module in wasm_module.customs.iter().filter(|(_, section)| {
        section
            .name()
            .starts_with(fluence_sdk_wit::GENERATED_SECTION_PREFIX)
    }) {
        let default_ids = walrus::IdsToIndices::default();
        let raw_data = custom_module.1.data(&default_ids);
        let decoded_json: FCEAst = serde_json::from_slice(&raw_data)?;
        match decoded_json {
            FCEAst::Record(record) => records.push(record),
            FCEAst::Function(function) => functions.push(function),
            FCEAst::ExternMod(extern_mod) => extern_mods.push(extern_mod),
        }
    }

    Ok(ModuleAST {
        records,
        functions,
        extern_mods,
    })
}

fn generate_interfaces(module_ast: &ModuleAST) -> Result<Interfaces<'_>> {
    let mut wit_resolver = WITResolver::default();
    generate_default_export_api(&mut wit_resolver.interfaces);

    for record in &module_ast.records {
        record.generate_wit(&mut wit_resolver)?;
    }
    validate_records(&wit_resolver)?;

    for function in &module_ast.functions {
        function.generate_wit(&mut wit_resolver)?;
    }
    for extern_mod in &module_ast.extern_mods {
        extern_mod.generate_wit(&mut wit_resolver)?;
    }

    Ok(wit_resolver.interfaces)
}

fn generate_default_export_api(interfaces: &mut Interfaces<'_>) {
    // TODO: the order is matter
    ALLOCATE_FUNC.update_interfaces(interfaces);
    DEALLOCATE_FUNC.update_interfaces(interfaces);
    GET_RESULT_SIZE_FUNC.update_interfaces(interfaces);
    GET_RESULT_PTR_FUNC.update_interfaces(interfaces);
    SET_RESULT_SIZE_FUNC.update_interfaces(interfaces);
    SET_RESULT_PTR_FUNC.update_interfaces(interfaces);
}

fn validate_records(wit_resolver: &WITResolver<'_>) -> Result<()> {
    fn validate_record_type(
        record_type: &IRecordType,
        recursion_level: u32,
        wit_resolver: &WITResolver<'_>,
    ) -> Result<()> {
        if recursion_level >= crate::TYPE_RESOLVE_RECURSION_LIMIT {
            return Err(WITGeneratorError::CorruptedRecord(String::from(
                "too many inner structures level",
            )));
        }

        for field in record_type.fields.iter() {
            match &field.ty {
                IType::Record(record_type_id) => {
                    let inner_record_type = wit_resolver.get_record_type(*record_type_id)?;
                    validate_record_type(&inner_record_type, recursion_level + 1, wit_resolver)?;
                }
                _ => continue,
            }
        }

        Ok(())
    }

    if wit_resolver.unresolved_types_count() != 0 {
        return Err(WITGeneratorError::CorruptedRecord(format!(
            "{} types unresolved",
            wit_resolver.unresolved_types_count()
        )));
    }

    for ty in wit_resolver.interfaces.types.iter() {
        let record_type = match ty {
            wasmer_wit::ast::Type::Record(ty) => ty,
            _ => continue,
        };

        validate_record_type(record_type, 0, wit_resolver)?;
    }

    Ok(())
}
