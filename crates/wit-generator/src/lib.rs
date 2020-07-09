mod wit_generator;
mod wasm_ast_extractor;

use wit_generator::WITGenerator;

pub use fluence_sdk_wit::FCEAst;
use wasmer_wit::ast::Interfaces;

pub fn embed_wit(path: std::path::PathBuf) {
    let ast_set = wasm_ast_extractor::wasm_ast_extractor(path.clone()).unwrap();
    let interfaces = generate_interfaces(&ast_set);
    wit_parser::embed_wit(path.clone(), path.clone(), &interfaces).unwrap();
}

fn generate_interfaces(ast_set: &[FCEAst]) -> Interfaces<'_> {
    let mut interfaces = Interfaces::default();
    generate_default_api(&mut interfaces);

    for ast in ast_set {
        ast.generate_wit(&mut interfaces);
    }

    interfaces
}

fn generate_default_api(interfaces: &mut Interfaces) {
    use wasmer_wit::ast::Type;
    use wasmer_wit::ast::Export;
    use wasmer_wit::types::InterfaceType as IType;

    let allocate_inputs = vec![IType::I32];
    let allocate_outputs = vec![IType::I32];
    let allocate_func_type = Type::Function {
        inputs: allocate_inputs,
        outputs: allocate_outputs,
    };

    let deallocate_inputs = vec![IType::I32, IType::I32];
    let deallocate_outputs = vec![];
    let deallocate_func_type = Type::Function {
        inputs: deallocate_inputs,
        outputs: deallocate_outputs,
    };

    let get_result_inputs = vec![];
    let get_result_outputs = vec![IType::I32];
    let get_result_size_func_type = Type::Function {
        inputs: get_result_inputs.clone(),
        outputs: get_result_outputs.clone(),
    };
    let get_result_ptr_func_type = Type::Function {
        inputs: get_result_inputs,
        outputs: get_result_outputs,
    };

    let set_result_inputs = vec![IType::I32];
    let set_result_outputs = vec![];
    let set_result_size_func_type = Type::Function {
        inputs: set_result_inputs.clone(),
        outputs: set_result_outputs.clone(),
    };
    let set_result_ptr_func_type = Type::Function {
        inputs: set_result_inputs,
        outputs: set_result_outputs,
    };

    interfaces.types.push(allocate_func_type);
    interfaces.types.push(deallocate_func_type);
    interfaces.types.push(get_result_size_func_type);
    interfaces.types.push(get_result_ptr_func_type);
    interfaces.types.push(set_result_size_func_type);
    interfaces.types.push(set_result_ptr_func_type);

    let allocate_export = Export {
        name: "allocate",
        function_type: 0,
    };
    interfaces.exports.push(allocate_export);

    let deallocate_export = Export {
        name: "deallocate",
        function_type: 1,
    };
    interfaces.exports.push(deallocate_export);

    let get_result_size_export = Export {
        name: "get_result_size",
        function_type: 2,
    };
    interfaces.exports.push(get_result_size_export);

    let get_result_ptr_export = Export {
        name: "get_result_ptr",
        function_type: 3,
    };
    interfaces.exports.push(get_result_ptr_export);

    let set_result_size_export = Export {
        name: "set_result_size",
        function_type: 4,
    };
    interfaces.exports.push(set_result_size_export);

    let set_result_ptr_export = Export {
        name: "set_result_ptr",
        function_type: 5,
    };
    interfaces.exports.push(set_result_ptr_export);
}
