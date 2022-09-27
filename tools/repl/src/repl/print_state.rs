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

use marine_wasm_backend_traits::WasiState;

pub(super) fn print_envs(module_name: &str, wasi_state: &dyn WasiState) {
    let envs = wasi_state.envs();
    if envs.is_empty() {
        println!("{} don't have environment variables", module_name);
        return;
    }

    println!("Environment variables:");
    for env in envs.iter() {
        match String::from_utf8(env.clone()) {
            Ok(string) => println!("{}", string),
            Err(_) => println!("{:?}", env),
        }
    }
}

pub(super) fn print_fs_state(_wasi_state: &dyn WasiState) {
    println!("Printing WASI filesystem state is not supported now.");
    /*
    let wasi_fs = &wasi_state.fs;

    println!("preopened file descriptors:\n{:?}\n", wasi_fs.preopen_fds);

    println!("name map:");
    for (name, inode) in &wasi_fs.name_map {
        println!("{} - {:?}", name, inode);
    }

    println!("\nfile descriptors map:");
    for (id, fd) in &wasi_fs.fd_map {
        println!("{} - {:?}", id, fd);
    }

    println!("\norphan file descriptors:");
    for (fd, inode) in &wasi_fs.orphan_fds {
        println!("{:?} - {:?}", fd, inode);
    }

    println!("\ninodes:");
    for (id, inode) in wasi_fs.inodes.iter().enumerate() {
        println!("{}: {:?}", id, inode);
    }
    */
}
