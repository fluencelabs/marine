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

use fluence::fce;
use fluence::module_manifest;
use fluence::WasmLoggerBuilder;

use std::fs;
use std::path::PathBuf;

module_manifest!();

const SITES_DIR: &str = "/sites/";

/// Log level can be changed by `RUST_LOG` env as well.
pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

/// You can read or write files from the file system if there is permission to use directories described in `Config.toml`.
#[fce]
pub fn put(name: String, file_content: Vec<u8>) -> String {
    log::info!("put called with file name {}\n", name);
    unsafe {
        log::info!(
            "put track: {} {} {} {} {}",
            *ALLOCS.get_mut(),
            *DEALLOCS.get_mut(),
            *REALLOCSA.get_mut(),
            *REALLOCSD.get_mut(),
            *ALLOCS.get_mut() - *DEALLOCS.get_mut() - *REALLOCSD.get_mut() + *REALLOCSA.get_mut()
        );
    }

    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);

    let result = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = result {
        return format!("file can't be written: {}", e);
    }

    log::info!("put ended with OK\n");

    String::from("Ok")
}

#[fce]
pub fn get(file_name: String) -> Vec<u8> {
    log::info!("get called with file name: {}\n", file_name);
    unsafe {
        log::info!(
            "get track: {} {} {} {} {}",
            *ALLOCS.get_mut(),
            *DEALLOCS.get_mut(),
            *REALLOCSA.get_mut(),
            *REALLOCSD.get_mut(),
            *ALLOCS.get_mut() - *DEALLOCS.get_mut() - *REALLOCSD.get_mut() + *REALLOCSA.get_mut()
        );
    }

    let tmp_filepath = format!("{}{}", SITES_DIR, file_name);

    let res = fs::read(tmp_filepath).unwrap_or_else(|_| b"error while reading file".to_vec());

    log::info!("get ended with file name: {}\n", file_name);

    res
}

use std::alloc::{GlobalAlloc, System, Layout};

#[global_allocator]
static GLOBAL_ALLOCATOR: WasmTracingAllocator<System> = WasmTracingAllocator(System);

#[derive(Debug)]
pub struct WasmTracingAllocator<A>(pub A)
where
    A: GlobalAlloc;

use std::sync::atomic::AtomicUsize;

static mut ALLOCS: AtomicUsize = AtomicUsize::new(0);
static mut DEALLOCS: AtomicUsize = AtomicUsize::new(0);
static mut REALLOCSA: AtomicUsize = AtomicUsize::new(0);
static mut REALLOCSD: AtomicUsize = AtomicUsize::new(0);

unsafe impl<A> GlobalAlloc for WasmTracingAllocator<A>
where
    A: GlobalAlloc,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let t = *ALLOCS.get_mut();
        *ALLOCS.get_mut() = t + size;

        let pointer = self.0.alloc(layout);
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let size = layout.size();
        let t = *DEALLOCS.get_mut();
        *DEALLOCS.get_mut() = t + size;

        self.0.dealloc(pointer, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let t = *ALLOCS.get_mut();
        *ALLOCS.get_mut() = t + size;

        let pointer = self.0.alloc_zeroed(layout);

        pointer
    }

    unsafe fn realloc(&self, old_pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        let t = *REALLOCSD.get_mut();
        *REALLOCSD.get_mut() = t + old_size;

        let t = *REALLOCSA.get_mut();
        *REALLOCSA.get_mut() = t + layout.size();

        let new_pointer = self.0.realloc(old_pointer, layout, new_size);

        new_pointer
    }
}
