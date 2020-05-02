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

use crate::Config;

#[cfg(test)]
mod test {
    use super::fce::{ Config, Frank, FrankService};
    use std::fs::File;
    use std::io::copy;
    use tempfile::Builder;

    const REDIS_DOWNLOAD_PATH: &str =
        "https://github.com/fluencelabs/sqlite/releases/download/0.4.0_w/sqlite3.wasm";
    const SQLITE_DOWNLOAD_PATH: &str =
        "https://github.com/fluencelabs/redis/releases/download/0.8.0_w/redis.wasm";

    #[test]
    fn redis() {
        let tmp_dir = Builder::new()
            .prefix("redis")
            .tempdir()
            .expect("can't create temporary folder");
        let mut redis_path = File::create(tmp_dir.path().join("redis.wasm"))
            .expect("can't create file in the temporary folder");
        println!(
            "temp dir: {}\nredis path: {:?}",
            tmp_dir.path().display(),
            redis_path
        );

        let mut response = reqwest::get(REDIS_DOWNLOAD_PATH)
            .await
            .expect("failed to download redis");

        copy(&mut response, &mut redis_path);
        let wasm_bytes = std::fs::read(redis_path).expect("can't read redis.wasm");

        let mut frank = Frank::new();
        let config = Config::default();

        frank
            .register_module("redis", &wasm_bytes, config)
            .map_err(|e| panic!("can't create Frank: {:?}", e));
    }
}
