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

mod downloader;

use fce::{Config, FCEService, FCE};

const REDIS_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/redis/releases/download/0.8.0_w/redis.wasm";
const SQLITE_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/sqlite/releases/download/0.4.0_w/sqlite3.wasm";

#[tokio::test]
async fn redis() {
    let wasm_bytes = downloader::download(REDIS_DOWNLOAD_URL).await;

    let mut fce = FCE::new();
    let module_name = "redis";
    let config = Config::default();

    fce.register_module(module_name, wasm_bytes.as_ref(), config)
        .unwrap_or_else(|e| panic!("can't create FCE: {:?}", e));

    let result1 = fce
        .invoke(module_name, "SET A 10".as_bytes())
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result2 = fce
        .invoke(module_name, "SADD B 20".as_bytes())
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result3 = fce
        .invoke(module_name, "GET A".as_bytes())
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result4 = fce
        .invoke(module_name, "SMEMBERS B".as_bytes())
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result5 = fce
        .invoke(
            module_name,
            "eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0".as_bytes(),
        )
        .expect("error while FCE invocation");

    let result1 = String::from_utf8(result1.outcome).expect("incorrect result obtained");
    let result2 = String::from_utf8(result2.outcome).expect("incorrect result obtained");
    let result3 = String::from_utf8(result3.outcome).expect("incorrect result obtained");
    let result4 = String::from_utf8(result4.outcome).expect("incorrect result obtained");
    let result5 = String::from_utf8(result5.outcome).expect("incorrect result obtained");

    assert_eq!(result1, "+OK\r\n");
    assert_eq!(result2, ":1\r\n");
    assert_eq!(result3, "$2\r\n10\r\n");
    assert_eq!(result4, "*1\r\n$2\r\n20\r\n");
    assert_eq!(result5, ":93\r\n");
}

#[tokio::test]
async fn sqlite() {
    let wasm_bytes = downloader::download(SQLITE_DOWNLOAD_URL).await;

    let mut fce = FCE::new();
    let module_name = "sqlite";
    let config = Config::default();

    fce.register_module(module_name, wasm_bytes.as_ref(), config)
        .unwrap_or_else(|e| panic!("can't create FCE: {:?}", e));

    let result1 = fce
        .invoke(
            module_name,
            "CREATE VIRTUAL TABLE users USING FTS5(body)".as_bytes(),
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result2 = fce
        .invoke(
            module_name,
            "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')".as_bytes(),
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result3 = fce
        .invoke(
            module_name,
            "SELECT * FROM users WHERE users MATCH 'A* OR B*'".as_bytes(),
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));

    let result1 = String::from_utf8(result1.outcome).expect("incorrect result obtained");
    let result2 = String::from_utf8(result2.outcome).expect("incorrect result obtained");
    let result3 = String::from_utf8(result3.outcome).expect("incorrect result obtained");

    assert_eq!(result1, "OK");
    assert_eq!(result2, "OK");
    assert_eq!(result3, "AB|BC");
}
