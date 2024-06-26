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

use crate::default_export_api_config::*;
use crate::errors::ITGeneratorError;
use crate::instructions_generator::ITGenerator;
use crate::instructions_generator::ITResolver;
use crate::Result;

use wasmer_it::ast::Interfaces;
use wasmer_it::IRecordType;
use wasmer_it::IType;

/// Parse generated by rust-sdk AST types, generate instructions and embed them to Wasm file.
pub fn embed_it<P>(path: P) -> Result<()>
where
    P: Into<std::path::PathBuf>,
{
    let path = path.into();

    let wasm_module = walrus::ModuleConfig::new()
        .parse_file(path.clone())
        .map_err(|e| ITGeneratorError::IOError(format!("{:?} can't be parsed: {:?}", path, e)))?;

    let module_ast = wasm_ast_extractor(&wasm_module)?;
    let interfaces = generate_interfaces(&module_ast)?;

    let wasm_module = marine_it_parser::delete_it_section(wasm_module);
    let mut wasm_module = marine_it_parser::embed_it(wasm_module, &interfaces);

    wasm_module.emit_wasm_file(path).map_err(|e| {
        ITGeneratorError::IOError(format!("resulted Wasm file can't be emitted: {:?}", e))
    })
}

pub(crate) struct ModuleAST {
    pub(crate) records: Vec<marine_macro_impl::RecordType>,
    pub(crate) functions: Vec<marine_macro_impl::FnType>,
    pub(crate) extern_mods: Vec<marine_macro_impl::ExternModType>,
}

/// Extract all custom AST types previously embedded by rust-sdk from compiled binary.
fn wasm_ast_extractor(wasm_module: &walrus::Module) -> Result<ModuleAST> {
    use marine_macro_impl::*;

    let mut records: Vec<RecordType> = Vec::new();
    let mut functions: Vec<FnType> = Vec::new();
    let mut extern_mods: Vec<ExternModType> = Vec::new();

    // consider only sections name of that starts with GENERATED_SECTION_PREFIX
    for custom_module in wasm_module.customs.iter().filter(|(_, section)| {
        let name = section.name();

        name.starts_with(marine_macro_impl::GENERATED_SECTION_PREFIX)
            || name.starts_with(marine_macro_impl::GENERATED_SECTION_PREFIX_FCE)
    }) {
        let default_ids = walrus::IdsToIndices::default();
        let raw_data = custom_module.1.data(&default_ids);
        let decoded_json: SDKAst = serde_json::from_slice(&raw_data)?;

        match decoded_json {
            SDKAst::Record(record) => records.push(record),
            SDKAst::Function(function) => functions.push(function),
            SDKAst::ExternMod(extern_mod) => extern_mods.push(extern_mod),
        }
    }

    Ok(ModuleAST {
        records,
        functions,
        extern_mods,
    })
}

fn generate_interfaces(module_ast: &ModuleAST) -> Result<Interfaces<'_>> {
    let mut it_resolver = ITResolver::default();
    generate_default_export_api(&mut it_resolver.interfaces);

    for record in &module_ast.records {
        record.generate_it(&mut it_resolver)?;
    }
    validate_records(&it_resolver)?;

    for function in &module_ast.functions {
        function.generate_it(&mut it_resolver)?;
    }
    for extern_mod in &module_ast.extern_mods {
        extern_mod.generate_it(&mut it_resolver)?;
    }

    Ok(it_resolver.interfaces)
}

fn generate_default_export_api(interfaces: &mut Interfaces<'_>) {
    // TODO: the order is matter
    ALLOCATE_FUNC.update_interfaces(interfaces);
    RELEASE_OBJECTS.update_interfaces(interfaces);
    GET_RESULT_SIZE_FUNC.update_interfaces(interfaces);
    GET_RESULT_PTR_FUNC.update_interfaces(interfaces);
    SET_RESULT_SIZE_FUNC.update_interfaces(interfaces);
    SET_RESULT_PTR_FUNC.update_interfaces(interfaces);
}

fn validate_records(it_resolver: &ITResolver<'_>) -> Result<()> {
    fn validate_record_type(
        record_type: &IRecordType,
        recursion_level: u32,
        it_resolver: &ITResolver<'_>,
    ) -> Result<()> {
        if recursion_level >= crate::TYPE_RESOLVE_RECURSION_LIMIT {
            return Err(ITGeneratorError::CorruptedRecord(String::from(
                "too many inner structures level",
            )));
        }

        for field in record_type.fields.iter() {
            match &field.ty {
                IType::Record(record_type_id) => {
                    let inner_record_type = it_resolver.get_record_type(*record_type_id)?;
                    validate_record_type(inner_record_type, recursion_level + 1, it_resolver)?;
                }
                _ => continue,
            }
        }

        Ok(())
    }

    if it_resolver.unresolved_types_count() != 0 {
        return Err(ITGeneratorError::CorruptedRecord(format!(
            "{} types unresolved",
            it_resolver.unresolved_types_count()
        )));
    }

    for ty in it_resolver.interfaces.types.iter() {
        let record_type = match ty {
            wasmer_it::ast::Type::Record(ty) => ty,
            _ => continue,
        };

        validate_record_type(record_type, 0, it_resolver)?;
    }

    Ok(())
}
