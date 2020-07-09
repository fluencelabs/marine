use walrus::ModuleConfig;

#[derive(Default, Clone)]
struct WasmAst {
    exports: Vec<fluence_sdk_wit::AstFunctionItem>,
    imports: Vec<fluence_sdk_wit::AstExternModItem>,
    records: Vec<fluence_sdk_wit::AstRecordItem>,
}

pub(crate) fn wasm_ast_extractor(
    wasm_path: std::path::PathBuf,
) -> Result<Vec<fluence_sdk_wit::FCEAst>, std::io::Error> {
    let module = ModuleConfig::new().parse_file(wasm_path).unwrap();
    let mut decoded_ast = Vec::new();

    for custom_module in module.customs.iter().filter(|(_, section)| {
        section
            .name()
            .starts_with(fluence_sdk_wit::GENERATED_SECTION_PREFIX)
    }) {
        let default_ids = walrus::IdsToIndices::default();
        let raw_data = custom_module.1.data(&default_ids);
        let decoded_json: fluence_sdk_wit::FCEAst = serde_json::from_slice(&raw_data).unwrap();
        decoded_ast.push(decoded_json);
    }

    Ok(decoded_ast)
}
