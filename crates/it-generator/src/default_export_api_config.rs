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

use wasmer_it::ast::Interfaces;
use wasmer_it::IType;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use once_cell::sync::Lazy;

use std::rc::Rc;

pub(crate) struct ApiExportFuncDescriptor {
    pub(crate) name: &'static str,
    pub(crate) id: u32,
    pub(crate) arguments: Vec<IFunctionArg>,
    pub(crate) output_types: Vec<IType>,
}

impl ApiExportFuncDescriptor {
    pub fn update_interfaces(&self, interfaces: &mut Interfaces<'_>) {
        let func_type = wasmer_it::ast::Type::Function {
            arguments: Rc::new(self.arguments.clone()),
            output_types: Rc::new(self.output_types.clone()),
        };
        interfaces.types.push(func_type);

        let export = wasmer_it::ast::Export {
            name: self.name,
            function_type: self.id,
        };
        interfaces.exports.push(export);
    }
}

pub(crate) static ALLOCATE_FUNC: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "allocate",
        id: 0,
        arguments: vec![IFunctionArg {
            name: String::from("size"),
            ty: IType::I32,
        }],
        output_types: vec![IType::I32],
    });

pub(crate) static RELEASE_OBJECTS: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "release_objects",
        id: 1,
        arguments: vec![],
        output_types: vec![],
    });

pub(crate) static GET_RESULT_SIZE_FUNC: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "get_result_size",
        id: 2,
        arguments: vec![],
        output_types: vec![IType::I32],
    });

pub(crate) static GET_RESULT_PTR_FUNC: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "get_result_ptr",
        id: 3,
        arguments: vec![],
        output_types: vec![IType::I32],
    });

pub(crate) static SET_RESULT_SIZE_FUNC: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "set_result_size",
        id: 4,
        arguments: vec![IFunctionArg {
            name: String::from("result_size"),
            ty: IType::I32,
        }],
        output_types: vec![],
    });

pub(crate) static SET_RESULT_PTR_FUNC: Lazy<ApiExportFuncDescriptor> =
    Lazy::new(|| ApiExportFuncDescriptor {
        name: "set_result_ptr",
        id: 5,
        arguments: vec![IFunctionArg {
            name: String::from("result_ptr"),
            ty: IType::I32,
        }],
        output_types: vec![],
    });
