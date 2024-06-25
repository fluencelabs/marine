/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine_core::MarineCore;
use marine_core::MarineCoreConfig;
use marine_core::IValue;
use marine_wasm_backend_traits::WasmBackend;
use marine_wasmtime_backend::WasmtimeWasmBackend;

const REDIS_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/redis/releases/download/v0.14.0_w/redis.wasm";
const SQLITE_DOWNLOAD_URL: &str =
    "https://github.com/fluencelabs/sqlite/releases/download/v0.14.0_w/sqlite3.wasm";

pub async fn download(url: &str) -> bytes::Bytes {
    reqwest::get(url)
        .await
        .expect("failed to download redis")
        .bytes()
        .await
        .expect("failed to convert response to bytes")
}

#[tokio::test]
async fn redis() {
    let wasm_bytes = download(REDIS_DOWNLOAD_URL).await;

    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    let module_name = "redis";
    let config = <_>::default();

    marine_core
        .load_module(module_name, wasm_bytes.as_ref(), config)
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let result1 = marine_core
        .call_async(
            module_name,
            "invoke",
            &[IValue::String(String::from("SET A 10"))],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));
    let result2 = marine_core
        .call_async(
            module_name,
            "invoke",
            &[IValue::String(String::from("SADD B 20"))],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));
    let result3 = marine_core
        .call_async(
            module_name,
            "invoke",
            &[IValue::String(String::from("GET A"))],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));
    let result4 = marine_core
        .call_async(
            module_name,
            "invoke",
            &[IValue::String(String::from("SMEMBERS B"))],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));
    let result5 = marine_core
        .call_async(
            module_name,
            "invoke",
            &[IValue::String(String::from(
                "eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0",
            ))],
        )
        .await
        .expect("error while Marine invocation");

    assert_eq!(result1, vec![IValue::String(String::from("+OK\r\n"))]);
    assert_eq!(result2, vec![IValue::String(String::from(":1\r\n"))]);
    assert_eq!(result3, vec![IValue::String(String::from("$2\r\n10\r\n"))]);
    assert_eq!(
        result4,
        vec![IValue::String(String::from("*1\r\n$2\r\n20\r\n"))]
    );
    assert_eq!(result5, vec![IValue::String(String::from(":93\r\n"))]);
}

#[tokio::test]
async fn sqlite() {
    let wasm_bytes = download(SQLITE_DOWNLOAD_URL).await;

    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    let module_name = "sqlite";
    let config = <_>::default();

    marine_core
        .load_module(module_name, wasm_bytes.as_ref(), config)
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let mut result1 = marine_core
        .call_async(
            module_name,
            "sqlite3_open_v2",
            &[
                IValue::String(String::from(":memory:")),
                IValue::S32(6),
                IValue::String(String::new()),
            ],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));

    let mut record_values = match result1.remove(0) {
        IValue::Record(value) => value.into_vec(),
        _ => panic!("return result should have record type"),
    };

    let db_handle = match record_values.remove(1) {
        IValue::U32(value) => value,
        _ => panic!("db handle should have u32 type"),
    };

    let mut result1 = marine_core
        .call_async(
            module_name,
            "sqlite3_exec",
            &[
                IValue::U32(db_handle),
                IValue::String(String::from("CREATE VIRTUAL TABLE users USING FTS5(body)")),
                IValue::S32(0),
                IValue::S32(0),
            ],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));

    let mut result2 = marine_core
        .call_async(
            module_name,
            "sqlite3_exec",
            &[
                IValue::U32(db_handle),
                IValue::String(String::from(
                    "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')",
                )),
                IValue::S32(0),
                IValue::S32(0),
            ],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));

    let mut result3 = marine_core
        .call_async(
            module_name,
            "sqlite3_exec",
            &[
                IValue::U32(db_handle),
                IValue::String(String::from(
                    "SELECT * FROM users WHERE users MATCH 'A* OR B*'",
                )),
                IValue::S32(0),
                IValue::S32(0),
            ],
        )
        .await
        .unwrap_or_else(|e| panic!("error while Marine invocation: {:?}", e));

    let result1 = match result1.remove(0) {
        IValue::Record(value) => value.into_vec(),
        _ => panic!("result should have record type"),
    };
    assert_eq!(result1, vec![IValue::S32(0), IValue::String(String::new())]);

    let result2 = match result2.remove(0) {
        IValue::Record(value) => value.into_vec(),
        _ => panic!("result should have record type"),
    };
    assert_eq!(result2, vec![IValue::S32(0), IValue::String(String::new())]);

    let result3 = match result3.remove(0) {
        IValue::Record(value) => value.into_vec(),
        _ => panic!("result should have record type"),
    };
    assert_eq!(result3, vec![IValue::S32(0), IValue::String(String::new())]);
}
