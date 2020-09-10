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
use wasmer_wit::types::RecordFieldType;

use std::collections::HashMap;

pub(crate) fn json_map_to_ivalues<'a, 'b>(
    mut json_map: serde_json::Map<String, SerdeValue>,
    signature: impl Iterator<Item = (&'a String, &'a IType)>,
    record_types: &'b HashMap<&'b String, &'b Vec1<RecordFieldType>>,
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

pub(crate) fn json_array_to_ivalues<'a, 'b>(
    mut json_array: Vec<SerdeValue>,
    signature: impl Iterator<Item = &'a IType>,
    record_types: &'b HashMap<&'b String, &'b Vec1<RecordFieldType>>,
) -> Result<Vec<IValue>> {
    let mut iargs = Vec::new();

    for (arg_id, arg_type) in signature.enumerate() {
        let json_value = json_array.remove(arg_id);
        let iarg = json_value_to_ivalue(json_value, arg_type, record_types)?;
        iargs.push(iarg);
    }

    if !json_array.is_empty() {
        return Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "function requires {} arguments, {} provided",
            iargs.len(),
            iargs.len() + json_array.len()
        )));
    }

    Ok(iargs)
}

fn json_value_to_ivalue(
    json_value: SerdeValue,
    ty: &IType,
    record_types: &HashMap<&String, &Vec1<RecordFieldType>>,
) -> Result<IValue> {
    match ty {
        IType::S8 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S8(value))
        }
        IType::S16 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S16(value))
        }
        IType::S32 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S32(value))
        }
        IType::S64 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::S64(value))
        }
        IType::U8 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U8(value))
        }
        IType::U16 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U16(value))
        }
        IType::U32 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U32(value))
        }
        IType::U64 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::U64(value))
        }
        IType::F32 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::F32(value))
        }
        IType::F64 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::F64(value))
        }
        IType::String => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::String(value))
        }
        IType::ByteArray => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::ByteArray(value))
        }
        IType::I32 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::I32(value))
        }
        IType::I64 => {
            let value = serde_json::from_value(json_value)
                .map_err(FaaSError::ArgumentDeserializationError)?;
            Ok(IValue::I64(value))
        }
        IType::Record(ty_name) => {
            let value = json_record_type_to_ivalue(json_value, ty_name, &record_types)?;
            Ok(IValue::Record(value))
        }
        IType::Anyref => Err(FaaSError::JsonArgumentsDeserializationError(String::from(
            "anyref interface-type is unsupported now",
        ))),
    }
}

#[allow(clippy::ptr_arg)]
fn json_record_type_to_ivalue(
    json_value: SerdeValue,
    itype_name: &String,
    record_types: &HashMap<&String, &Vec1<RecordFieldType>>,
) -> Result<Vec1<IValue>> {
    let record_type = record_types.get(itype_name).ok_or_else(|| {
        FaaSError::JsonArgumentsDeserializationError(format!(
            "record with type `{}` wasn't found",
            itype_name
        ))
    })?;

    match json_value {
        SerdeValue::Object(json_map) => Ok(Vec1::new(json_map_to_ivalues(
            json_map,
            record_type.iter().map(|field| (&field.name, &field.ty)),
            record_types,
        )?)
        .unwrap()),
        SerdeValue::Array(json_array) => Ok(Vec1::new(json_array_to_ivalues(
            json_array,
            record_type.iter().map(|field| (&field.ty)),
            record_types,
        )?)
        .unwrap()),
        _ => Err(FaaSError::JsonArgumentsDeserializationError(format!(
            "record with type `{}` should be encoded as array or map of fields",
            itype_name
        ))),
    }
}
