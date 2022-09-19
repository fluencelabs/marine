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

use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::ExportContext;
use it_memory_traits::Memory;
use it_memory_traits::SequentialMemoryView;
use it_memory_traits::SequentialReader;
//use wasmer_core::vm::Ctx;
//use wasmer_core::memory::ptr::{Array, WasmPtr};
//use crate::IType::String;

pub(crate) fn log_utf8_string_closure<WB: WasmBackend>(
    logging_mask: i32,
    module: String,
) -> impl Fn(&mut <WB as WasmBackend>::ExportContext, (i32, i32, i32, i32)) {
    move |ctx, (level, target, msg_offset, msg_size)| {
        if target == 0 || target & logging_mask != 0 {
            log_utf8_string::<WB>(&module, ctx, level, msg_offset, msg_size)
        }
    }
}

pub(crate) fn log_utf8_string<WB: WasmBackend>(
    module: &str,
    ctx: &mut <WB as WasmBackend>::ExportContext,
    level: i32,
    msg_offset: i32,
    msg_size: i32,
) {
    let level = level_from_i32(level);
    let msg = read_string::<WB>(ctx, msg_offset, msg_size);

    match msg {
        Some(msg) => log::logger().log(
            &log::Record::builder()
                .args(format_args!("{}", msg))
                .level(level)
                .module_path(module.into())
                .target(module)
                .build(),
        ),
        None => log::warn!("logger: incorrect UTF8 string's been supplied to logger"),
    }
}

#[inline]
fn read_string<WB: WasmBackend>(
    ctx: &<WB as WasmBackend>::ExportContext,
    offset: i32,
    size: i32,
) -> Option<String> {
    let view = ctx.memory(0).view();
    let reader = view
        .sequential_reader(offset as u32 as usize, size as u32 as usize)
        .unwrap();
    let bytes = (0..size).map(|_| reader.read_byte()).collect();
    String::from_utf8(bytes).ok()
}

#[inline]
fn level_from_i32(level: i32) -> log::Level {
    match level {
        1 => log::Level::Error,
        2 => log::Level::Warn,
        3 => log::Level::Info,
        4 => log::Level::Debug,
        5 => log::Level::Trace,
        _ => log::Level::max(),
    }
}
