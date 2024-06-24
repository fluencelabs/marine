/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use wasmer_it::ast::Interfaces;
use wasmer_it::IType;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use once_cell::sync::Lazy;

use std::sync::Arc;

pub(crate) struct ApiExportFuncDescriptor {
    pub(crate) name: &'static str,
    pub(crate) id: u32,
    pub(crate) arguments: Vec<IFunctionArg>,
    pub(crate) output_types: Vec<IType>,
}

impl ApiExportFuncDescriptor {
    pub fn update_interfaces(&self, interfaces: &mut Interfaces<'_>) {
        let func_type = wasmer_it::ast::Type::Function {
            arguments: Arc::new(self.arguments.clone()),
            output_types: Arc::new(self.output_types.clone()),
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
