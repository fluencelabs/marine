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

use crate::IValue;
use crate::IType;
use crate::Result;
use crate::FaaSError;

use serde_json::Value as SerdeValue;
use wasmer_wit::vec1::Vec1;
use wasmer_wit::types::RecordType;

use std::collections::HashMap;

pub(crate) fn json_to_ivalues(
    json_args: serde_json::Value,
    func_signature: &fce::FCEFunctionSignature<'_>,
    record_types: &HashMap<&u64, &RecordType>,
) -> Result<Vec<IValue>> {
    let ivalues = match json_args {
        SerdeValue::Object(json_map) => json_map_to_ivalues(
            json_map,
            func_signature.arguments.iter().map(|arg| (&arg.name, &arg.ty)),
            &record_types,
        )?,
        SerdeValue::Array(json_array) => json_array_to_ivalues(
            json_array,
            func_signature.arguments.iter().map(|arg| &arg.ty),
            &record_types,
        )?,
        SerdeValue::String(json_string) => json_string_to_ivalue(json_string, func_signature)?,
        json_bool @ SerdeValue::Bool(_) => json_bool_to_ivalue(json_bool, func_signature)?,
        json_number @ SerdeValue::Number(_) => json_number_to_ivalue(json_number, func_signature)?,
        SerdeValue::Null => json_null_to_ivalue(func_signature)?,
    };

    Ok(ivalues)
}

fn json_map_to_ivalues<'a, 'b>(
    mut json_map: serde_json::Map<String, SerdeValue>,
    signature: impl Iterator<Item = (&'a String, &'a IType)>,
    record_types: &'b HashMap<&'b u64, &'b RecordType>,
) -> Result<Vec<IValue>> {
    let mut iargs = Vec::new();

    for (arg_name, arg_type) in signature {
        let json_value = json_map
            .remove(arg_name)
            .ok_or_else(|| FaaSError::MissingArgumentError(arg_name.clone()))?;
        let iarg = json_value_to_ivalue(json_value, arg_type, record_types)?;
        iargs.push(iarg);
    }

    if !json_map.is_empty() {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "function requires {} arguments, {} provided",
            iargs.len(),
            iargs.len() + json_map.len()
        )));
    }

    Ok(iargs)
}

fn json_array_to_ivalues<'a, 'b>(
    mut json_array: Vec<SerdeValue>,
    signature: impl Iterator<Item = &'a IType> + std::iter::ExactSizeIterator,
    record_types: &'b HashMap<&'b u64, &'b RecordType>,
) -> Result<Vec<IValue>> {
    if json_array.len() != signature.len() {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "function requires {} arguments, {} provided",
            signature.len(),
            json_array.len()
        )));
    }

    let mut iargs = Vec::with_capacity(signature.len());

    for arg_type in signature {
        // remove here is safe because we've already checked sizes
        let json_value = json_array.remove(0);
        let iarg = json_value_to_ivalue(json_value, arg_type, record_types)?;
        iargs.push(iarg);
    }

    Ok(iargs)
}

fn json_string_to_ivalue(json_string: String, func_signature: &fce::FCEFunctionSignature<'_>) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 || func_signature.arguments[0].ty != IType::String {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "the called function has the following signature: {:?}, but only one string argument is provided",
            func_signature
        )));
    }

    Ok(vec![IValue::String(json_string)])
}

fn json_bool_to_ivalue(json_bool: SerdeValue, func_signature: &fce::FCEFunctionSignature<'_>) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "the called function has the following signature: {:?}, but only one bool argument is provided",
            func_signature
        )));
    }

    Ok(vec![json_value_to_ivalue(
        json_bool,
        &func_signature.arguments[0].ty,
        &HashMap::new(),
    )?])
}

fn json_number_to_ivalue(
    json_number: SerdeValue,
    func_signature: &fce::FCEFunctionSignature<'_>,
) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "the called function has the following signature: {:?}, but only one number argument is provided",
            func_signature
        )));
    }

    Ok(vec![json_value_to_ivalue(
        json_number,
        &func_signature.arguments[0].ty,
        &HashMap::new(),
    )?])
}

fn json_null_to_ivalue(func_signature: &fce::FCEFunctionSignature<'_>) -> Result<Vec<IValue>> {
    if !func_signature.arguments.is_empty() {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "the called function has the following signature: {:?}, but no arguments is provided",
            func_signature
        )));
    }

    Ok(vec![])
}

fn json_value_to_ivalue(
    json_value: SerdeValue,
    ty: &IType,
    record_types: &HashMap<&u64, &RecordType>,
) -> Result<IValue> {
    // TODO: get rid of copy-past
    match ty {
        IType::S8 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S8(value))
        }
        IType::S16 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S16(value))
        }
        IType::S32 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S32(value))
        }
        IType::S64 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S64(value))
        }
        IType::U8 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U8(value))
        }
        IType::U16 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U16(value))
        }
        IType::U32 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U32(value))
        }
        IType::U64 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U64(value))
        }
        IType::F32 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::F32(value))
        }
        IType::F64 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::F64(value))
        }
        IType::String => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::String(value))
        }
        IType::Array(value_type) => {
            let value = match json_value {
                SerdeValue::Array(json_array) => {
                    let mut iargs = Vec::with_capacity(json_array.len());

                    for json_value in json_array {
                        let iarg = json_value_to_ivalue(json_value, value_type, record_types)?;
                        iargs.push(iarg);
                    }

                    Ok(iargs)
                }
                _ => Err(FaaSError::JsonArgumentsDeserializationError(format!(
                    "expected array of {:?} types, got {:?}",
                    value_type, json_value
                ))),
            }?;

            Ok(IValue::Array(value))
        }
        IType::I32 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::I32(value))
        }
        IType::I64 => {
            let value = serde_json::from_value(json_value).map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::I64(value))
        }
        IType::Record(record_type_id) => {
            let value = json_record_type_to_ivalue(json_value, record_type_id, &record_types)?;
            Ok(IValue::Record(value))
        }
        IType::Anyref => Err(FaaSError::JsonArgumentsDeserializationError(String::from(
            "anyrefs aren't supported now",
        ))),
    }
}

#[allow(clippy::ptr_arg)]
fn json_record_type_to_ivalue(
    json_value: SerdeValue,
    record_type_id: &u64,
    record_types: &HashMap<&u64, &RecordType>,
) -> Result<Vec1<IValue>> {
    let record_type = record_types.get(record_type_id).ok_or_else(|| {
        FaaSError::JsonArgumentsDeserializationError(format!("record with type id `{}` wasn't found", record_type_id))
    })?;

    match json_value {
        SerdeValue::Object(json_map) => Ok(Vec1::new(json_map_to_ivalues(
            json_map,
            record_type.fields.iter().map(|field| (&field.name, &field.ty)),
            record_types,
        )?)
        .unwrap()),
        SerdeValue::Array(json_array) => Ok(Vec1::new(json_array_to_ivalues(
            json_array,
            record_type.fields.iter().map(|field| (&field.ty)),
            record_types,
        )?)
        .unwrap()),
        _ => Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "record with type id `{}` should be encoded as array or map of fields",
            record_type_id
        ))),
    }
}
