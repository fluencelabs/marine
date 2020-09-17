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

mod storage;

use fluence::fce;
use fluence::WasmLogger;
use crate::storage::{init, user_exists, delete_user, update_name, add_user, get_all_users, update_relay};

const OWNER: &str = "owner_id";

pub fn main() {
    WasmLogger::init_with_level(log::Level::Info).unwrap();
    init();
}

#[fce]
fn join(user: String, relay: String, sig: String, name: String) -> String {
    add_user(user, relay, sig, name)
}

#[fce]
fn get_users() -> String {
    get_all_users()
}

#[fce]
fn change_name(user: String, name: String, signature: String) -> String {
    if user != signature {
        return "Error. Invalid signature.".to_string();
    }

    if !user_exists(user.as_str()) {
        return "Error. No such user.".to_string();
    }

    update_name(user, name);

    "Ok".to_string()
}

#[fce]
fn change_relay(user: String, relay: String, sig: String, signature: String) -> String {
    if user != signature {
        return "Error. Invalid signature.".to_string();
    }

    if !user_exists(user.as_str()) {
        return "Error. No such user.".to_string();
    }

    update_relay(user, relay, sig);

    "Ok".to_string()
}

#[fce]
fn delete(user: String, signature: String) -> String {
    let owner = std::env::var(OWNER).unwrap_or_else(|_| "".to_string());
    if user != signature && owner != signature {
        return "Error. You cannot delete this user.".to_string();
    }

    if !user_exists(user.as_str()) {
        return "Error. No such user.".to_string();
    }

    delete_user(user.as_str())
}

#[fce]
fn is_exists(user: String) -> bool {
    true
}
