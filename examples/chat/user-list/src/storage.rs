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

pub fn init() {
    unsafe {
        invoke("CREATE TABLE IF NOT EXISTS users(user_id INTEGER PRIMARY KEY, user TEXT NOT NULL, relay TEXT NOT NULL, sig TEXT NOT NULL, name TEXT NOT NULL);".to_string());
    }
    log::info!("tables created");
}

pub fn user_exists(user: &str) -> bool {
    let req = format!("SELECT * FROM users WHERE user = '{}'", user);
    let result = unsafe {
        invoke(req)
    };
    log::info!("deletion result:");
    log::info!("{}", result.as_str());
    if result.is_empty() || result == "OK" {
        return false;
    }

    return true;
}

pub fn update_name(user: String, name: String) -> String {
    unsafe {
        invoke(format!("UPDATE users SET name = '{}' WHERE user = '{}'", name, user))
    }
}

pub fn update_relay(user: String, relay: String, sig: String) -> String {
    unsafe {
        invoke(format!("UPDATE users SET relay = '{}', sig = '{}' WHERE user = '{}'", relay, sig, user))
    }
}

pub fn get_all_users() -> String {
    unsafe {
        invoke(format!("SELECT * FROM users"))
    }
}

pub fn add_user(user: String, relay: String, sig: String, name: String) -> String {
    unsafe {
        invoke(format!("INSERT INTO users (user,relay,sig,name) VALUES ('{}','{}','{}','{}')", user, relay, sig, name))
    }
}

pub fn delete_user(user: &str) -> String {
    unsafe {
        invoke(format!("DELETE FROM users WHERE user = '{}';", user))
    }
}

#[fce]
#[link(wasm_import_module = "sqlite")]
extern "C" {
    #[link_name = "invoke"]
    pub fn invoke(cmd: String) -> String;

}