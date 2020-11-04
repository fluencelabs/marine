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
use crate::FaaSError::JsonArgumentsDeserializationError as ArgDeError;

use fce::RecordTypes;
use serde_json::Value as JValue;
use wasmer_wit::vec1::Vec1;

use std::collections::HashMap;

pub(crate) fn json_to_ivalues(
    json_args: JValue,
    func_signature: &crate::FaaSFunctionSignature<'_>,
    record_types: &RecordTypes,
) -> Result<Vec<IValue>> {
    let ivalues = match json_args {
        JValue::Object(json_map) => json_map_to_ivalues(
            json_map,
            func_signature
                .arguments
                .iter()
                .map(|arg| (&arg.name, &arg.ty)),
            &record_types,
        )?,
        JValue::Array(json_array) => json_array_to_ivalues(
            json_array,
            func_signature.arguments.iter().map(|arg| &arg.ty),
            &record_types,
        )?,
        JValue::String(json_string) => json_string_to_ivalue(json_string, func_signature)?,
        json_bool @ JValue::Bool(_) => json_bool_to_ivalue(json_bool, func_signature)?,
        json_number @ JValue::Number(_) => json_number_to_ivalue(json_number, func_signature)?,
        JValue::Null => json_null_to_ivalue(func_signature)?,
    };

    Ok(ivalues)
}

fn json_map_to_ivalues<'a, 'b>(
    mut json_map: serde_json::Map<String, JValue>,
    signature: impl Iterator<Item = (&'a String, &'a IType)>,
    record_types: &'b RecordTypes,
) -> Result<Vec<IValue>> {
    let mut iargs = Vec::new();

    for (arg_name, arg_type) in signature {
        let json_value = json_map
            .remove(arg_name)
            .ok_or_else(|| ArgDeError(format!("missing argument with name {}", arg_name)))?;
        let iarg = json_value_to_ivalue(json_value, arg_type, record_types)?;
        iargs.push(iarg);
    }

    if !json_map.is_empty() {
        return Err(ArgDeError(format!(
            "function requires {} arguments, {} provided",
            iargs.len(),
            iargs.len() + json_map.len()
        )));
    }

    Ok(iargs)
}

fn json_array_to_ivalues<'a, 'b>(
    mut json_array: Vec<JValue>,
    signature: impl Iterator<Item = &'a IType> + std::iter::ExactSizeIterator,
    record_types: &'b RecordTypes,
) -> Result<Vec<IValue>> {
    if json_array.len() != signature.len() {
        return Err(ArgDeError(format!(
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

fn json_string_to_ivalue(
    json_string: String,
    func_signature: &fce::FCEFunctionSignature<'_>,
) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 || func_signature.arguments[0].ty != IType::String {
        return Err(ArgDeError(format!(
            "the called function has the following signature: {:?}, but only one string argument is provided",
            func_signature
        )));
    }

    Ok(vec![IValue::String(json_string)])
}

fn json_bool_to_ivalue(
    json_bool: JValue,
    func_signature: &fce::FCEFunctionSignature<'_>,
) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 {
        return Err(ArgDeError(format!(
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
    json_number: JValue,
    func_signature: &fce::FCEFunctionSignature<'_>,
) -> Result<Vec<IValue>> {
    if func_signature.arguments.len() != 1 {
        return Err(ArgDeError(format!(
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
        return Err(ArgDeError(format!(
            "the called function has the following signature: {:?}, but no arguments is provided",
            func_signature
        )));
    }

    Ok(vec![])
}

fn json_value_to_ivalue(
    json_value: JValue,
    ty: &IType,
    record_types: &RecordTypes,
) -> Result<IValue> {
    macro_rules! to_ivalue(
        ($json_value:expr, $ty:ident) => {
            {
                let value = serde_json::from_value($json_value).map_err(|e| {
                    ArgDeError(format!(
                        "error {:?} occurred while deserialize output result to a json value",
                        e
                    ))
                })?;

                Ok(IValue::$ty(value))
            }
        }
    );

    match ty {
        IType::S8 => to_ivalue!(json_value, S8),
        IType::S16 => to_ivalue!(json_value, S16),
        IType::S32 => to_ivalue!(json_value, S32),
        IType::S64 => to_ivalue!(json_value, S64),
        IType::U8 => to_ivalue!(json_value, U8),
        IType::U16 => to_ivalue!(json_value, U16),
        IType::U32 => to_ivalue!(json_value, U32),
        IType::U64 => to_ivalue!(json_value, U64),
        IType::F32 => to_ivalue!(json_value, F32),
        IType::F64 => to_ivalue!(json_value, F64),
        IType::String => to_ivalue!(json_value, String),
        IType::Array(value_type) => {
            let value = match json_value {
                JValue::Array(json_array) => {
                    let iargs: Result<Vec<_>> = json_array
                        .into_iter()
                        .map(|json_value| {
                            json_value_to_ivalue(json_value, value_type, record_types)
                        })
                        .collect();

                    Ok(iargs?)
                }
                _ => Err(ArgDeError(format!(
                    "expected array of {:?} types, got {:?}",
                    value_type, json_value
                ))),
            }?;

            Ok(IValue::Array(value))
        }
        IType::I32 => to_ivalue!(json_value, I32),
        IType::I64 => to_ivalue!(json_value, I64),
        IType::Record(record_type_id) => {
            let value = json_record_type_to_ivalue(json_value, record_type_id, &record_types)?;
            Ok(IValue::Record(value))
        }
        IType::Anyref => Err(ArgDeError(String::from("anyrefs aren't supported now"))),
    }
}

#[allow(clippy::ptr_arg)]
fn json_record_type_to_ivalue(
    json_value: JValue,
    record_type_id: &u64,
    record_types: &RecordTypes,
) -> Result<Vec1<IValue>> {
    let record_type = record_types.get(record_type_id).ok_or_else(|| {
        ArgDeError(format!(
            "record with type id `{}` wasn't found",
            record_type_id
        ))
    })?;

    match json_value {
        JValue::Object(json_map) => Ok(Vec1::new(json_map_to_ivalues(
            json_map,
            record_type
                .fields
                .iter()
                .map(|field| (&field.name, &field.ty)),
            record_types,
        )?)
        .unwrap()),
        JValue::Array(json_array) => Ok(Vec1::new(json_array_to_ivalues(
            json_array,
            record_type.fields.iter().map(|field| (&field.ty)),
            record_types,
        )?)
        .unwrap()),
        _ => Err(ArgDeError(format!(
            "record with type id `{}` should be encoded as array or map of fields",
            record_type_id
        ))),
    }
}
