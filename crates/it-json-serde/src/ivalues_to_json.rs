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

use crate::IValue;
use crate::IType;
use crate::ITJsonSeDeError::Se;
use crate::JsonResult;
use crate::MRecordTypes;

use serde_json::Value as JValue;

pub fn ivalues_to_json(
    mut ivalues: Vec<IValue>,
    outputs: &[IType],
    record_types: &MRecordTypes,
) -> JsonResult<JValue> {
    if outputs.len() != ivalues.len() {
        return Err(Se(format!(
            "resulted values {:?} and function signature {:?} aren't compatible",
            ivalues, outputs
        )));
    }
    match ivalues.len() {
        0 => Ok(JValue::Null),
        1 => ivalue_to_json(ivalues.remove(0), outputs.first().unwrap(), record_types),
        _ => unimplemented!(
            "multi-values aren't supported now - more then one result values aren't possible"
        ),
    }
}

fn ivalue_to_json(
    ivalue: IValue,
    output: &IType,
    record_types: &MRecordTypes,
) -> JsonResult<JValue> {
    use serde_json::json;

    // clone here needed because binding by-value and by-ref in the same pattern in unstable
    match (ivalue, output.clone()) {
        (IValue::Boolean(value), IType::Boolean) => Ok(json!(value)),
        (IValue::S8(value), IType::S8) => Ok(json!(value)),
        (IValue::S16(value), IType::S16) => Ok(json!(value)),
        (IValue::S32(value), IType::S32) => Ok(json!(value)),
        (IValue::S64(value), IType::S64) => Ok(json!(value)),
        (IValue::U8(value), IType::U8) => Ok(json!(value)),
        (IValue::U16(value), IType::U16) => Ok(json!(value)),
        (IValue::U32(value), IType::U32) => Ok(json!(value)),
        (IValue::U64(value), IType::U64) => Ok(json!(value)),
        (IValue::I32(value), IType::I32) => Ok(json!(value)),
        (IValue::I64(value), IType::I64) => Ok(json!(value)),
        (IValue::F32(value), IType::F32) => Ok(json!(value)),
        (IValue::F64(value), IType::F64) => Ok(json!(value)),
        (IValue::String(value), IType::String) => Ok(json!(value)),
        (IValue::ByteArray(value), IType::ByteArray) => {
            let result = value.into_iter().map(|v| json!(v)).collect();
            Ok(JValue::Array(result))
        }
        (IValue::Array(value), IType::ByteArray) => {
            let result: JsonResult<Vec<_>> = value
                .into_iter()
                .map(|v| ivalue_to_json(v, &IType::U8, record_types))
                .collect();

            Ok(JValue::Array(result?))
        }
        (IValue::ByteArray(value), IType::Array(array_ty)) => {
            let result: JsonResult<Vec<_>> = value
                .into_iter()
                .map(|v| ivalue_to_json(IValue::U8(v), &array_ty, record_types))
                .collect();

            Ok(JValue::Array(result?))
        }
        (IValue::Array(value), IType::Array(array_ty)) => {
            let result: JsonResult<Vec<_>> = value
                .into_iter()
                .map(|v| ivalue_to_json(v, &array_ty, record_types))
                .collect();

            Ok(JValue::Array(result?))
        }
        (IValue::Record(field_values), IType::Record(record_id)) => {
            let record_type = record_types.get(&record_id).ok_or_else(|| {
                Se(format!(
                    "record id {} wasn't found in module record types",
                    record_id
                ))
            })?;
            let field_types = &record_type.fields;

            if field_values.len() != field_types.len() {
                return Err(Se(format!(
                    "output record {:?} isn't compatible to output record fields {:?}",
                    field_values, field_types
                )));
            }

            let field_values = field_values.into_vec();
            let mut result = serde_json::Map::with_capacity(field_values.len());

            for (field_value, field_type) in field_values.into_iter().zip(field_types.iter()) {
                let json_field_value = ivalue_to_json(field_value, &field_type.ty, record_types)?;
                result.insert(field_type.name.clone(), json_field_value);
            }

            Ok(JValue::Object(result))
        }
        (ivalue, itype) => Err(Se(format!(
            "value {:?} is incompatible to type {:?}",
            ivalue, itype
        ))),
    }
}
