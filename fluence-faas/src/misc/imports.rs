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

use crate::init_wasm_func_once;
use crate::call_wasm_func;
use crate::IValue;
use crate::IType;

use wasmer_core::memory::ptr::{Array, WasmPtr};
use wasmer_core::vm::Ctx;
use wasmer_core::typed_func::DynamicFunc;
use wasmer_core::types::Value as WValue;
use wasmer_core::types::Type as WType;
use wasmer_core::types::FuncSig;
use wasmer_wit::types::RecordType;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const ALLOCATE_FUNC_NAME: &str = "allocate";
const SET_PTR_FUNC_NAME: &str = "set_result_ptr";
const SET_SIZE_FUNC_NAME: &str = "set_result_size";

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => log::info!("{}", msg),
        None => log::warn!("logger: incorrect UTF8 string's been supplied to logger"),
    }
}

fn write_to_mem(context: &mut Ctx, address: usize, value: &[u8]) {
    let memory = context.memory(0);

    memory.view::<u8>()[address..(address + value.len())]
        .iter()
        .zip(value.iter())
        .for_each(|(cell, byte)| cell.set(*byte));
}

fn itypes_to_wtypes(itypes: &[IType]) -> Vec<WType> {
    itypes
        .iter()
        .map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) => vec![WType::I32, WType::I32],
            _ => vec![WType::I32],
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn wvalues_to_ivalues(
    wvalues: &[WValue],
    itypes: &[IType],
    ctx: &Ctx,
    record_types: &HashMap<&u64, &RecordType>,
) -> Vec<IValue> {
    let mut result = Vec::new();
    let mut wvalue = wvalues.iter();

    macro_rules! simple_wvalue_to_ivalue(
        ($wtype:ident, $ivalue:ident, $result:ident) => {
            {
                let w = match wvalue.next().unwrap() {
                    WValue::$wtype(v) => *v,
                    _ => unreachable!(),
                };
                $result.push(IValue::$ivalue(w as _))
            }
        }
    );

    for itype in itypes.iter() {
        match itype {
            IType::S8 => simple_wvalue_to_ivalue!(I32, S8, result),
            IType::S16 => simple_wvalue_to_ivalue!(I32, S16, result),
            IType::S32 => simple_wvalue_to_ivalue!(I32, S32, result),
            IType::S64 => simple_wvalue_to_ivalue!(I64, S64, result),
            IType::U8 => simple_wvalue_to_ivalue!(I32, U8, result),
            IType::U16 => simple_wvalue_to_ivalue!(I32, U16, result),
            IType::U32 => simple_wvalue_to_ivalue!(I32, U32, result),
            IType::U64 => simple_wvalue_to_ivalue!(I64, U64, result),
            IType::I32 => simple_wvalue_to_ivalue!(I32, I32, result),
            IType::I64 => simple_wvalue_to_ivalue!(I64, I64, result),
            IType::F32 => simple_wvalue_to_ivalue!(F32, F32, result),
            IType::F64 => simple_wvalue_to_ivalue!(F64, F64, result),
            IType::Anyref => unimplemented!(),
            IType::String => {
                let offset = match wvalue.next().unwrap() {
                    WValue::I32(v) => *v,
                    _ => unreachable!(),
                };
                let size = match wvalue.next().unwrap() {
                    WValue::I32(v) => *v,
                    _ => unreachable!(),
                };

                let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
                let str = wasm_ptr.get_utf8_string(ctx.memory(0), size as _).unwrap();

                result.push(IValue::String(str.to_string()));
            }
            IType::Array(ty) => {
                let offset = match wvalue.next().unwrap() {
                    WValue::I32(v) => *v,
                    _ => unreachable!(),
                };
                let size = match wvalue.next().unwrap() {
                    WValue::I32(v) => *v,
                    _ => unreachable!(),
                };

                let array = lift_array(ctx, ty, offset as _, size as _);
                result.push(IValue::Array(array));
            }
            IType::Record(record_type_id) => {
                let record_type = record_types.get(record_type_id).unwrap();

                let offset = match wvalue.next().unwrap() {
                    WValue::I32(v) => *v,
                    _ => unreachable!(),
                };

                let record = lift_record(ctx, record_type, offset as _, record_types);
                result.push(record);
            }
        }
    }

    result
}

fn lift_array(ctx: &Ctx, value_type: &IType, offset: usize, size: usize) -> Vec<IValue> {
    if size == 0 {
        return vec![];
    }

    macro_rules! simple_type_array_convert(
        ($offset:ident, $size:ident, $ctx:ident, $rtype:ident, $itype:ident) => {
            {
                let wasm_ptr = WasmPtr::<$rtype, Array>::new($offset as _);
                wasm_ptr.deref($ctx.memory(0), $offset as _, $size as _).unwrap().iter().map(|v| IValue::$itype(v.get() as _)).collect::<Vec<_>>()
            }
        }
    );

    let result_array = match value_type {
        IType::S8 => simple_type_array_convert!(offset, size, ctx, i8, S8),
        IType::S16 => simple_type_array_convert!(offset, size, ctx, i16, S16),
        IType::S32 => simple_type_array_convert!(offset, size, ctx, i32, S32),
        IType::S64 => simple_type_array_convert!(offset, size, ctx, i64, S64),
        IType::U8 => simple_type_array_convert!(offset, size, ctx, u8, U8),
        IType::U16 => simple_type_array_convert!(offset, size, ctx, u16, U16),
        IType::U32 => simple_type_array_convert!(offset, size, ctx, u32, U32),
        IType::U64 => simple_type_array_convert!(offset, size, ctx, u64, U64),
        IType::F32 => simple_type_array_convert!(offset, size, ctx, f32, F32),
        IType::F64 => simple_type_array_convert!(offset, size, ctx, f64, F64),
        IType::I32 => simple_type_array_convert!(offset, size, ctx, i32, I32),
        IType::I64 => simple_type_array_convert!(offset, size, ctx, i64, I64),
        IType::Anyref => unimplemented!(),
        IType::String => {
            let wasm_ptr = WasmPtr::<u32, Array>::new(offset as _);
            let data = wasm_ptr
                .deref(ctx.memory(0), offset as _, size as _)
                .unwrap();

            let mut result = Vec::with_capacity(data.len() / 2);
            let mut data = data.iter();

            while let Some(string_offset) = data.next() {
                let string_size = data.next().unwrap();

                let string = WasmPtr::<u8, Array>::new(string_offset.get() as _)
                    .get_utf8_string(ctx.memory(0), string_size.get() as _)
                    .unwrap();

                result.push(IValue::String(string.to_string()));
            }

            result
        }
        IType::Array(ty) => {
            let wasm_ptr = WasmPtr::<u32, Array>::new(offset as _);
            let data = wasm_ptr
                .deref(ctx.memory(0), offset as _, size as _)
                .unwrap();

            let mut result = Vec::with_capacity(data.len() / 2);
            let mut data = data.iter();

            while let Some(array_offset) = data.next() {
                let array_size = data.next().unwrap();
                let array = lift_array(ctx, ty, array_offset.get() as _, array_size.get() as _);

                result.push(IValue::Array(array));
            }

            result
        }
        IType::Record(_record_type_id) => unimplemented!(),
    };

    result_array
}

pub(super) fn lower_array(
    ctx: &mut Ctx,
    array_values: Vec<InterfaceValue>,
    allocate_func: Box<RefCell<Option<Func<'static, i32, i32>>>>,
) -> (usize, usize) {
    let mut result: Vec<u64> = Vec::with_capacity(array_values.len());

    for value in array_values {
        match value {
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.push(value as _),
            IValue::S32(value) => result.push(value as _),
            IValue::S64(value) => result.push(value as _),
            IValue::U8(value) => result.push(value as _),
            IValue::U16(value) => result.push(value as _),
            IValue::U32(value) => result.push(value as _),
            IValue::U64(value) => result.push(value as _),
            IValue::I32(value) => result.push(value as _),
            IValue::I64(value) => result.push(value as _),
            IValue::F32(value) => result.push(value as _),
            IValue::F64(value) => result.push(value.to_bits()),
            IValue::String(value) => {
                let mem_address = call_wasm_func!(allocate_func, value.len() as _);
                write_to_mem(ctx, mem_address as usize, str.as_bytes());

                result.push(mem_address as _);
                result.push(value.len() as _);
            }

            InterfaceValue::Array(values) => {
                let (array_offset, array_size) = if !values.is_empty() {
                    lower_array(values, ctx, allocate_func.clone())?
                } else {
                    (0, 0)
                };

                result.push(array_offset as _);
                result.push(array_size as _);
            }

            InterfaceValue::Record(values) => {
                let record_offset = lower_record(values, ctx, allocate_func.clone())?;
                result.push(record_offset as _);
            }
        }
    }

    let result = safe_transmute::transmute_to_bytes::<u64>(&result);
    let mem_address = call_wasm_func!(allocate_func, result.len() as _);
    let result_pointer = write_to_mem(ctx, mem_address as _, result)?;

    (result_pointer as _, result.len() as _)
}

pub(super) fn lift_record(
    ctx: &Ctx,
    record_type: &RecordType,
    offset: usize,
    record_types: &HashMap<&u64, &RecordType>,
) -> IValue {
    use wasmer_wit::vec1::Vec1;

    fn record_size(record_type: &RecordType) -> usize {
        let mut record_size = 0;

        for field_type in record_type.fields.iter() {
            let params_count = match field_type.ty {
                IType::String | IType::Array(_) => 2,
                _ => 1,
            };

            record_size += std::mem::size_of::<u64>() * params_count;
        }

        record_size
    }

    let length = record_type.fields.len();
    let mut values = Vec::with_capacity(length);
    let size = record_size(record_type);
    let data = WasmPtr::<u64, Array>::new(offset as _)
        .deref(ctx.memory(0), offset as _, size as _)
        .unwrap();

    let mut field_id = 0;
    for field in (*record_type.fields).iter() {
        let value = data[field_id].get();
        match &field.ty {
            IType::S8 => {
                values.push(IValue::S8(value as _));
            }
            IType::S16 => {
                values.push(IValue::S16(value as _));
            }
            IType::S32 => {
                values.push(IValue::S32(value as _));
            }
            IType::S64 => {
                values.push(IValue::S64(value as _));
            }
            IType::I32 => {
                values.push(IValue::I32(value as _));
            }
            IType::I64 => {
                values.push(IValue::I64(value as _));
            }
            IType::U8 => {
                values.push(IValue::U8(value as _));
            }
            IType::U16 => {
                values.push(IValue::U16(value as _));
            }
            IType::U32 => {
                values.push(IValue::U32(value as _));
            }
            IType::U64 => {
                values.push(IValue::U64(value as _));
            }
            IType::F32 => {
                values.push(IValue::F32(value as _));
            }
            IType::F64 => values.push(IValue::F64(f64::from_bits(value))),
            IType::Anyref => {}
            IType::String => {
                let string_offset = value;
                field_id += 1;
                let string_size = data[field_id].get();

                if string_size != 0 {
                    let string = WasmPtr::<u8, Array>::new(string_offset as _)
                        .get_utf8_string(ctx.memory(0), size as _)
                        .unwrap();
                    values.push(IValue::String(string.to_string()));
                } else {
                    values.push(IValue::String(String::new()));
                }
            }
            IType::Array(ty) => {
                let array_offset = value;
                field_id += 1;
                let array_size = data[field_id].get();

                if array_size != 0 {
                    let array = lift_array(ctx, ty, array_offset as _, array_size as _);
                    values.push(IValue::Array(array));
                } else {
                    values.push(IValue::Array(vec![]));
                }
            }
            IType::Record(record_type_id) => {
                let offset = value;

                let record_type = record_types.get(record_type_id).unwrap();
                values.push(lift_record(ctx, record_type, offset as _, record_types));
            }
        }
        field_id += 1;
    }

    IValue::Record(
        Vec1::new(values.into_iter().collect())
            .expect("Record must have at least one field, zero given"),
    )
}

fn lower_record(
    ctx: &mut Ctx,
    values: Vec1<InterfaceValue>,
    allocate_func: Box<RefCell<Option<Func<'static, i32, i32>>>>,
) -> i32 {
    let mut result: Vec<u64> = Vec::with_capacity(values.len());

    for value in values.into_vec() {
        match value {
            InterfaceValue::S8(value) => result.push(value as _),
            InterfaceValue::S16(value) => result.push(value as _),
            InterfaceValue::S32(value) => result.push(value as _),
            InterfaceValue::S64(value) => result.push(value as _),
            InterfaceValue::U8(value) => result.push(value as _),
            InterfaceValue::U16(value) => result.push(value as _),
            InterfaceValue::U32(value) => result.push(value as _),
            InterfaceValue::U64(value) => result.push(value as _),
            InterfaceValue::I32(value) => result.push(value as _),
            InterfaceValue::I64(value) => result.push(value as _),
            InterfaceValue::F32(value) => result.push(value as _),
            InterfaceValue::F64(value) => result.push(value.to_bits()),
            InterfaceValue::String(value) => {
                let string_pointer = if !value.is_empty() {
                    let mem_address = call_wasm_func!(allocate_func, str.len() as i32);
                    write_to_mem(ctx, mem_address as usize, str.as_bytes());
                    mem_address
                } else {
                    0
                };

                result.push(string_pointer as _);
                result.push(value.len() as _);
            }

            InterfaceValue::Array(values) => {
                let (offset, size) = if !values.is_empty() {
                    lower_array(ctx, values, allocate_func.clone())
                } else {
                    (0, 0)
                };

                result.push(offset as _);
                result.push(size as _);
            }

            InterfaceValue::Record(values) => {
                let record_ptr = loer_record(ctx, values, allocate_func.clone())?;

                result.push(record_ptr as _);
            }
        }
    }

    let result = safe_transmute::transmute_to_bytes::<u64>(&result);
    let mem_address = call_wasm_func!(allocate_func, result.len() as _);
    write_to_mem(ctx, mem_address as usize, str.as_bytes());

    Ok(result_pointer as _)
}

fn ivalues_to_wvalues(
    ivalues: Vec<IValue>,
    ctx: &mut Ctx,
    allocate_func: Box<RefCell<Option<Func<'static, i32, i32>>>>,
    record_types: &HashMap<&u64, &RecordType>,
) -> Vec<WValue> {
    let mut result = Vec::new();

    for ivalue in ivalues {
        match ivalue {
            IValue::S8(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::S16(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::S32(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::S64(v) => {
                result.push(WValue::I64(v as _));
            }
            IValue::U8(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::U16(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::U32(v) => {
                result.push(WValue::I32(v as _));
            }
            IValue::U64(v) => {
                result.push(WValue::I64(v as _));
            }
            IValue::F32(v) => {
                result.push(WValue::F32(v));
            }
            IValue::F64(v) => {
                result.push(WValue::F64(v));
            }
            IValue::String(str) => {
                let mem_address = call_wasm_func!(allocate_func, str.len() as i32);
                write_to_mem(ctx, mem_address as usize, str.as_bytes());

                result.push(WValue::I32(mem_address as _));
                result.push(WValue::I32(str.len() as _));
            }
            IValue::Array(values) => {

            }
        }
    }

    result
}

// #[rustfmt::skip]
pub(super) fn create_host_import_func<F>(
    closure: Box<F>,
    argument_types: Vec<IType>,
    output_types: Vec<IType>,
    record_types: &HashMap<&u64, &RecordType>,
) -> DynamicFunc<'static>
where
    F: Fn(&mut Ctx, &[IValue]) -> Vec<IValue> + 'static,
{
    use wasmer_core::Func;

    let allocate_func: Box<RefCell<Option<Func<'static, i32, i32>>>> = Box::new(RefCell::new(None));
    let set_result_ptr_func: Box<RefCell<Option<Func<'static, i32, ()>>>> =
        Box::new(RefCell::new(None));
    let set_result_size_func: Box<RefCell<Option<Func<'static, i32, ()>>>> =
        Box::new(RefCell::new(None));

    let raw_args = itypes_to_wtypes(&argument_types);
    let raw_output = itypes_to_wtypes(&output_types);

    let func = move |ctx: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
        let ivalues = wvalues_to_ivalues(inputs, &argument_types, ctx, record_types);

        let result = closure(ctx, &ivalues);

        unsafe {
            init_wasm_func_once!(allocate_func, ctx, i32, i32, ALLOCATE_FUNC_NAME, 2);
            init_wasm_func_once!(set_result_ptr_func, ctx, i32, (), SET_PTR_FUNC_NAME, 3);
            init_wasm_func_once!(set_result_size_func, ctx, i32, (), SET_SIZE_FUNC_NAME, 4);

            let mem_address = call_wasm_func!(allocate_func, result.len() as i32);
            write_to_mem(ctx, mem_address as usize, result.as_bytes());
            call_wasm_func!(set_result_ptr_func, mem_address);
            call_wasm_func!(set_result_size_func, result.len() as i32);

            vec![]
        }
    };

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(raw_args, raw_output)),
        func,
    )
}

pub(super) fn create_get_call_parameters_func(
    call_parameters: Rc<RefCell<crate::CallParameters>>,
) -> DynamicFunc<'static> {
    use wasmer_core::Func;

    let allocate_func: Box<RefCell<Option<Func<'static, i32, i32>>>> = Box::new(RefCell::new(None));

    // TODO: refactor this approach after switching on the new Wasmer
    let func = move |ctx: &mut Ctx, _inputs: &[WValue]| -> Vec<WValue> {
        unsafe {
            init_wasm_func_once!(allocate_func, ctx, i32, i32, ALLOCATE_FUNC_NAME, 2);

            let call_id_ptr =
                call_wasm_func!(allocate_func, call_parameters.borrow().call_id.len() as i32);
            let user_name_ptr = call_wasm_func!(
                allocate_func,
                call_parameters.borrow().user_name.len() as i32
            );
            let application_id_ptr = call_wasm_func!(
                allocate_func,
                call_parameters.borrow().application_id.len() as i32
            );

            write_to_mem(
                ctx,
                call_id_ptr as usize,
                call_parameters.borrow().call_id.as_bytes(),
            );
            write_to_mem(
                ctx,
                user_name_ptr as usize,
                call_parameters.borrow().user_name.as_bytes(),
            );
            write_to_mem(
                ctx,
                application_id_ptr as usize,
                call_parameters.borrow().application_id.as_bytes(),
            );

            let mut serialized_call_parameters = Vec::new();
            serialized_call_parameters.push(call_id_ptr as u64);
            serialized_call_parameters.push(call_parameters.borrow().call_id.len() as u64);
            serialized_call_parameters.push(user_name_ptr as u64);
            serialized_call_parameters.push(call_parameters.borrow().user_name.len() as u64);
            serialized_call_parameters.push(application_id_ptr as u64);
            serialized_call_parameters.push(call_parameters.borrow().application_id.len() as u64);

            let serialized_call_parameters_ptr =
                call_wasm_func!(allocate_func, serialized_call_parameters.len() as i32);
            let serialized_call_parameters_bytes =
                safe_transmute::transmute_to_bytes::<u64>(&serialized_call_parameters);
            write_to_mem(
                ctx,
                serialized_call_parameters_ptr as usize,
                serialized_call_parameters_bytes,
            );

            vec![WValue::I32(serialized_call_parameters_ptr as _)]
        }
    };

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(vec![], vec![WType::I32])),
        func,
    )
}
