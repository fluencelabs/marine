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

pub fn main() {

}

#[fce]
fn get_n_save(url: String, file_name: String) -> String {
    let result = unsafe { curl(url) };
    println!("execution result {:?}", result);
    unsafe { file_put(file_name, result.into_bytes()); }


    "Ok".to_string()
}

#[fce]
#[link(wasm_import_module = "wasm_curl")]
extern "C" {
    #[link_name = "get"]
    pub fn curl(url: String) -> String;
}

#[fce]
#[link(wasm_import_module = "wasm_local_storage")]
extern "C" {
    #[link_name = "get"]
    pub fn file_get(file_name: String) -> Vec<u8>;

    #[link_name = "put"]
    pub fn file_put(name: String, file_content: Vec<u8>) -> String;
}
