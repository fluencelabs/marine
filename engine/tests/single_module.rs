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
use fce::{FCE, IValue};

const REDIS_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/redis/releases/download/0.9.0_w/redis.wasm";
const SQLITE_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/sqlite/releases/download/0.4.0_w/sqlite3.wasm";

#[tokio::test]
async fn redis() {
    let wasm_bytes = downloader::download(REDIS_DOWNLOAD_URL).await;

    let mut fce = FCE::new();
    let module_name = "redis";
    let config = <_>::default();

    fce.load_module(module_name, wasm_bytes.as_ref(), config)
        .unwrap_or_else(|e| panic!("can't create FCE: {:?}", e));

    let result1 = fce
        .call(module_name, "invoke", &[IValue::String(String::from("SET A 10"))])
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result2 = fce
        .call(module_name, "invoke", &[IValue::String(String::from("SADD B 20"))])
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result3 = fce
        .call(module_name, "invoke", &[IValue::String(String::from("GET A"))])
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result4 = fce
        .call(module_name, "invoke", &[IValue::String(String::from("SMEMBERS B"))])
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result5 = fce
        .call(module_name, "invoke", &[IValue::String(String::from("eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0"))])
        .expect("error while FCE invocation");

    assert_eq!(result1, vec![IValue::String(String::from("+OK\r\n"))]);
    assert_eq!(result2, vec![IValue::String(String::from(":1\r\n"))]);
    assert_eq!(result3, vec![IValue::String(String::from("$2\r\n10\r\n"))]);
    assert_eq!(result4, vec![IValue::String(String::from("*1\r\n$2\r\n20\r\n"))]);
    assert_eq!(result5, vec![IValue::String(String::from(":93\r\n"))]);
}

#[tokio::test]
#[ignore]
async fn sqlite() {
    let wasm_bytes = downloader::download(SQLITE_DOWNLOAD_URL).await;

    let mut fce = FCE::new();
    let module_name = "sqlite";
    let config = <_>::default();

    fce.load_module(module_name, wasm_bytes.as_ref(), config)
        .unwrap_or_else(|e| panic!("can't create FCE: {:?}", e));

    let result1 = fce
        .call(
            module_name,
            "invoke",
            &[IValue::String(String::from("CREATE VIRTUAL TABLE users USING FTS5(body)"))],
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result2 = fce
        .call(
            module_name,
            "invoke",
            &[IValue::String(String::from("INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')"))],
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));
    let result3 = fce
        .call(
            module_name,
            "invoke",
            &[IValue::String(String::from("SELECT * FROM users WHERE users MATCH 'A* OR B*"))],
        )
        .unwrap_or_else(|e| panic!("error while FCE invocation: {:?}", e));

    assert_eq!(result1, vec![IValue::String(String::from("OK"))]);
    assert_eq!(result2, vec![IValue::String(String::from("OK"))]);
    assert_eq!(result3, vec![IValue::String(String::from("AB|BC"))]);
}
