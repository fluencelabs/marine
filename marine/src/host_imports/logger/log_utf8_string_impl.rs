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

use marine_wasm_backend_traits::AsContextMut;
use marine_wasm_backend_traits::ImportCallContext;
use marine_wasm_backend_traits::WasmBackend;

use it_memory_traits::Memory;
use it_memory_traits::MemoryReadable;

pub(crate) fn log_utf8_string_closure<WB: WasmBackend>(
    logging_mask: i32,
    module: String,
) -> impl Fn(<WB as WasmBackend>::ImportCallContext<'_>, i32, i32, i32, i32) {
    move |ctx, level, target, msg_offset, msg_size| {
        if target == 0 || target & logging_mask != 0 {
            log_utf8_string::<WB>(&module, ctx, level, msg_offset, msg_size)
        }
    }
}

pub(crate) fn log_utf8_string<WB: WasmBackend>(
    module: &str,
    mut ctx: <WB as WasmBackend>::ImportCallContext<'_>,
    level: i32,
    msg_offset: i32,
    msg_size: i32,
) {
    let level = level_from_i32(level);
    let msg = read_string::<WB>(&mut ctx, msg_offset, msg_size);

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
    ctx: &mut <WB as WasmBackend>::ImportCallContext<'_>,
    offset: i32,
    size: i32,
) -> Option<String> {
    let view = ctx.memory(0).unwrap().view(); // TODO handle error
    let bytes = view.read_vec(&mut ctx.as_context_mut(), offset as u32, size as u32);
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
