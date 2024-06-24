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
}
