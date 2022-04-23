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

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

pub fn main() {
  WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub struct GreetingRecord {
    pub str: String,
    pub num: i32,
}

#[marine]
pub fn greeting_record() -> GreetingRecord {
    GreetingRecord {
        str: String::from("Hello, world!"),
        num: 42,
    }
}

#[marine]
pub fn logging() {
    log::info!("info");
    log::warn!("warn");
    log::error!("error");
    log::debug!("debug");
    log::trace!("trace");
}

#[marine]
pub fn void_fn() {}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn smoke_test(greeting: marine_test_env::greeting_record::ModuleInterface) {
        let actual = greeting.greeting_record();
        assert_eq!(actual.str, "Hello, world!");
        assert_eq!(actual.num, 42);
    }
}
