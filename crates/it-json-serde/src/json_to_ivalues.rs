/*
 * Copyright 2022 Fluence Labs Limited
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
use super::ItJsonSerdeError::DeserializationError;
use super::JsonResult;

use crate::MRecordTypes;
use serde_json::Value as JValue;
use wasmer_it::NEVec;

use std::collections::HashMap;
use std::iter::ExactSizeIterator;

/// Convert json to an array of ivalues according to the supplied argument types.
pub fn json_to_ivalues<'a, 'b>(
    json_args: JValue,
    arg_types: impl Iterator<Item = (&'a String, &'a IType)> + ExactSizeIterator,
    record_types: &'b MRecordTypes,
) -> JsonResult<Vec<IValue>> {
    let ivalues = match json_args {
        JValue::Object(json_map) => json_map_to_ivalues(json_map, arg_types, record_types)?,
        JValue::Array(json_array) => {
            json_array_to_ivalues(json_array, arg_types.map(|arg| arg.1), record_types)?
        }
        JValue::Null => json_null_to_ivalues(arg_types)?,
        json_value => json_value_to_ivalues(json_value, arg_types)?,
    };

    Ok(ivalues)
}

/// Convert json map to an array of ivalues according to the supplied argument types.
fn json_map_to_ivalues<'a, 'b>(
    mut json_map: serde_json::Map<String, JValue>,
    arg_types: impl Iterator<Item = (&'a String, &'a IType)>,
    record_types: &'b MRecordTypes,
) -> JsonResult<Vec<IValue>> {
    let mut iargs = Vec::new();

    for (arg_name, arg_type) in arg_types {
        let json_value = json_map.remove(arg_name).ok_or_else(|| {
            DeserializationError(format!("missing argument with name {}", arg_name))
        })?;
        let iarg = jvalue_to_ivalue(json_value, arg_type, record_types)?;
        iargs.push(iarg);
    }

    if !json_map.is_empty() {
        return Err(DeserializationError(format!(
            "function requires {} arguments, {} provided",
            iargs.len(),
            iargs.len() + json_map.len()
        )));
    }

    Ok(iargs)
}

/// Convert json array to an array of ivalues according to the supplied argument types.
fn json_array_to_ivalues<'a, 'b>(
    json_array: Vec<JValue>,
    arg_types: impl Iterator<Item = &'a IType> + ExactSizeIterator,
    record_types: &'b MRecordTypes,
) -> JsonResult<Vec<IValue>> {
    if json_array.len() != arg_types.len() {
        return Err(DeserializationError(format!(
            "function requires {} arguments, {} provided",
            arg_types.len(),
            json_array.len()
        )));
    }

    let iargs = json_array
        .into_iter()
        .zip(arg_types)
        .map(|(json_value, arg_type)| jvalue_to_ivalue(json_value, arg_type, record_types))
        .collect::<JsonResult<Vec<_>>>()?;

    Ok(iargs)
}

/// Convert json value (Number, String or Bool) to an array of ivalues according to the supplied argument types.
fn json_value_to_ivalues<'a>(
    json_value: JValue,
    mut arg_types: impl Iterator<Item = (&'a String, &'a IType)> + ExactSizeIterator,
) -> JsonResult<Vec<IValue>> {
    if arg_types.len() != 1 {
        return Err(DeserializationError(format!(
            "called function has the following signature: '{:?}', and it isn't suitable for an argument '{:?}' provided",
            arg_types.collect::<Vec<_>>(),
            json_value,
        )));
    }

    // unwrap is safe here because iterator size's been checked
    let arg_type = arg_types.next().unwrap().1;
    let ivalue = jvalue_to_ivalue(json_value, arg_type, &HashMap::new())?;

    Ok(vec![ivalue])
}

/// Convert json Null to an empty array of ivalues.
fn json_null_to_ivalues<'a>(
    arg_types: impl Iterator<Item = (&'a String, &'a IType)> + ExactSizeIterator,
) -> JsonResult<Vec<IValue>> {
    if arg_types.len() != 0 {
        return Err(DeserializationError(format!(
            "the called function has the following signature: {:?}, but no arguments is provided",
            arg_types.collect::<Vec<_>>()
        )));
    }

    Ok(vec![])
}

/// Convert one JValue to an array of ivalues according to the supplied argument type.
fn jvalue_to_ivalue(jvalue: JValue, ty: &IType, record_types: &MRecordTypes) -> JsonResult<IValue> {
    macro_rules! to_ivalue(
        ($json_value:expr, $ty:ident) => {
            {
                let value = match $json_value {
                    // if there is an array with only one element try to implicitly flatten it,
                    // this is needed mostly because jsonpath lib returns Vec<&JValue> and
                    // could be changed in future
                    JValue::Array(mut json_array) if json_array.len() == 1 => {
                        serde_json::from_value(json_array.remove(0))
                    },
                    jvalue => serde_json::from_value(jvalue),
                }.map_err(|e|
                    DeserializationError(format!("error {:?} occurred while deserializing function arguments",e))
                )?;

                Ok(IValue::$ty(value))
            }
        }
    );

    match ty {
        IType::Boolean => to_ivalue!(jvalue, Boolean),
        IType::S8 => to_ivalue!(jvalue, S8),
        IType::S16 => to_ivalue!(jvalue, S16),
        IType::S32 => to_ivalue!(jvalue, S32),
        IType::S64 => to_ivalue!(jvalue, S64),
        IType::U8 => to_ivalue!(jvalue, U8),
        IType::U16 => to_ivalue!(jvalue, U16),
        IType::U32 => to_ivalue!(jvalue, U32),
        IType::U64 => to_ivalue!(jvalue, U64),
        IType::F32 => to_ivalue!(jvalue, F32),
        IType::F64 => to_ivalue!(jvalue, F64),
        IType::String => to_ivalue!(jvalue, String),
        IType::ByteArray => {
            let value = match jvalue {
                JValue::Array(json_array) => {
                    let iargs = json_array
                        .into_iter()
                        .map(|json_value| jvalue_to_ivalue(json_value, &IType::U8, record_types))
                        .collect::<JsonResult<Vec<_>>>()?;

                    Ok(iargs)
                }
                _ => Err(DeserializationError(format!(
                    "expected bytearray, got {:?}",
                    jvalue
                ))),
            }?;

            Ok(IValue::Array(value))
        }
        IType::Array(value_type) => {
            let value = match jvalue {
                JValue::Array(json_array) => {
                    let iargs = json_array
                        .into_iter()
                        .map(|json_value| jvalue_to_ivalue(json_value, value_type, record_types))
                        .collect::<JsonResult<Vec<_>>>()?;

                    Ok(iargs)
                }
                _ => Err(DeserializationError(format!(
                    "expected array of {:?} types, got {:?}",
                    value_type, jvalue
                ))),
            }?;

            Ok(IValue::Array(value))
        }
        IType::I32 => to_ivalue!(jvalue, I32),
        IType::I64 => to_ivalue!(jvalue, I64),
        IType::Record(record_type_id) => {
            let value = json_record_type_to_ivalue(jvalue, record_type_id, record_types)?;
            Ok(IValue::Record(value))
        }
    }
}

#[allow(clippy::ptr_arg)]
/// Convert JValue of array or object types to an IValue record type.
// TODO: after introducing new Record type wrapper change the result type
fn json_record_type_to_ivalue(
    json_value: JValue,
    record_type_id: &u64,
    record_types: &MRecordTypes,
) -> JsonResult<NEVec<IValue>> {
    let record_type = record_types.get(record_type_id).ok_or_else(|| {
        DeserializationError(format!(
            "record with type id `{}` wasn't found",
            record_type_id
        ))
    })?;

    match json_value {
        JValue::Object(json_map) => Ok(NEVec::new(json_map_to_ivalues(
            json_map,
            record_type
                .fields
                .iter()
                .map(|field| (&field.name, &field.ty)),
            record_types,
        )?)
        .unwrap()),
        JValue::Array(json_array) => Ok(NEVec::new(json_array_to_ivalues(
            json_array,
            record_type.fields.iter().map(|field| (&field.ty)),
            record_types,
        )?)
        .unwrap()),
        _ => Err(DeserializationError(format!(
            "record with type id `{}` should be encoded as array or map of fields",
            record_type_id
        ))),
    }
}
