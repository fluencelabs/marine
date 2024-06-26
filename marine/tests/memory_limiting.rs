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

mod utils;

use marine::CallParameters;
use marine::IValue;
use marine::Marine;
use marine::MarineError;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use marine_wasm_backend_traits::WasmBackend;

use bytesize::KIB;
use bytesize::MIB;
use once_cell::sync::Lazy;

static FAIL_ON_STARTUP_CONFIG: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load(
        "./tests/wasm_tests/memory_limiting/2MiB_limit_fail_on_startup.toml",
    )
    .expect("toml faas config should be created")
});

static LIMIT_64_MIB: Lazy<marine::TomlMarineConfig> = Lazy::new(|| {
    marine::TomlMarineConfig::load("./tests/wasm_tests/memory_limiting/64MiB_limit.toml")
        .expect("toml faas config should be created")
});
const FACADE_MODULE: &str = "memory_limiting_pure";
const WASM_PAGE_SIZE: u64 = 64 * KIB;

#[tokio::test]
pub async fn triggered_on_instantiation() {
    let faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        FAIL_ON_STARTUP_CONFIG.clone(),
    )
    .await;

    match faas {
        Err(MarineError::HighProbabilityOOM {
            allocation_stats, ..
        }) if allocation_stats.allocation_rejects > 0 => return,
        Ok(_) => panic!("Expected HighProbabilityOOM instantiation error, but it succeed"),
        Err(e) => panic!(
            "Expected HighProbabilityOOM instantiation error, got: {:?}",
            e
        ),
    }
}
#[tokio::test]
pub async fn triggered_by_single_module() {
    let mut faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        LIMIT_64_MIB.clone(),
    )
    .await
    .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas).await;

    let start_memory = get_total_memory(&faas);
    let to_allocate = (64 * MIB - start_memory) / WASM_PAGE_SIZE + 1;

    let result = faas
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_single_module_64KB_pieces",
            &[IValue::U32(to_allocate as u32)],
            CallParameters::default(),
        )
        .await;

    // a module can allocate 1 page less because of tables memory
    assert_eq!(get_total_memory(&faas), 64 * MIB - WASM_PAGE_SIZE);
    match result {
        Err(MarineError::HighProbabilityOOM {
            allocation_stats, ..
        }) if allocation_stats.allocation_rejects > 0 => return,
        Err(e) => panic!(
            "Expected HighProbabilityOOM error, got different error: {:?}",
            e
        ),
        Ok(_) => panic!("Expected HighProbabilityOOM error, got success"),
    }
}

#[tokio::test]
pub async fn not_triggered_near_limit_single_module() {
    let mut faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        LIMIT_64_MIB.clone(),
    )
    .await
    .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas).await;

    let start_memory = get_total_memory(&faas);
    // 1 page removed because of tables memory
    let to_allocate_pages = (64 * MIB - start_memory) / WASM_PAGE_SIZE - 1;

    let result = faas
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_single_module_64KB_pieces",
            &[IValue::U32(to_allocate_pages as u32)],
            CallParameters::default(),
        )
        .await;

    let expected_memory = start_memory + to_allocate_pages * WASM_PAGE_SIZE;
    assert_eq!(get_total_memory(&faas), expected_memory);
    match result {
        Ok(_) => return,
        Err(e) => {
            panic!("Expected success, got error: {:?}", e)
        }
    }
}

#[tokio::test]
pub async fn triggered_by_two_modules() {
    let mut faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        LIMIT_64_MIB.clone(),
    )
    .await
    .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas).await;

    let start_memory = get_total_memory(&faas);
    let to_allocate = (64 * MIB - start_memory) / 2 / WASM_PAGE_SIZE + 1;

    let result = faas
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_two_modules_64KB_pieces",
            &[IValue::U32(to_allocate as u32)],
            CallParameters::default(),
        )
        .await;

    // the service can allocate 1 page less because of tables memory
    assert_eq!(get_total_memory(&faas), 64 * MIB - WASM_PAGE_SIZE);
    match result {
        Err(MarineError::HighProbabilityOOM {
            allocation_stats, ..
        }) if allocation_stats.allocation_rejects > 0 => return,
        Err(e) => panic!(
            "Expected HighProbabilityOOM error, got different error: {:?}",
            e
        ),
        Ok(_) => panic!("Expected HighProbabilityOOM, got success"),
    }
}

#[tokio::test]
pub async fn not_triggered_near_limit_two_modules() {
    let mut faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        LIMIT_64_MIB.clone(),
    )
    .await
    .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas).await;

    let start_memory = get_total_memory(&faas);

    // two pages removed because of table memory
    let to_allocate = (64 * MIB - start_memory) / 2 / WASM_PAGE_SIZE - 2;

    let result = faas
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_two_modules_64KB_pieces",
            &[IValue::U32(to_allocate as u32)],
            CallParameters::default(),
        )
        .await;

    let expected_memory = start_memory + to_allocate * WASM_PAGE_SIZE * 2;
    assert_eq!(get_total_memory(&faas), expected_memory);
    match result {
        Ok(_) => return,
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[tokio::test]
pub async fn triggered_by_large_allocation_single_module() {
    let mut faas = Marine::with_raw_config(
        WasmtimeWasmBackend::new_async().unwrap(),
        LIMIT_64_MIB.clone(),
    )
    .await
    .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas).await;

    let start_memory = get_total_memory(&faas);
    let to_allocate = 128 * MIB;

    let result = faas
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_single_module_single_piece",
            &[IValue::S64(to_allocate as i64)],
            CallParameters::default(),
        )
        .await;

    assert_eq!(get_total_memory(&faas), start_memory);
    match result {
        Err(MarineError::HighProbabilityOOM {
            allocation_stats, ..
        }) if allocation_stats.allocation_rejects > 0 => return,
        Err(e) => panic!(
            "Expected HighProbabilityOOM error, got different error: {:?}",
            e
        ),
        Ok(_) => panic!("Expected HighProbabilityOOM error, got success"),
    }
}

fn get_total_memory(faas: &marine::Marine) -> u64 {
    faas.module_memory_stats()
        .modules
        .iter()
        .map(|stats| stats.memory_size as u64)
        .sum()
}

async fn fill_start_memory(marine: &mut Marine) {
    let start_memory = get_total_memory(marine);
    let pages_to_allocate = (start_memory / 2) / WASM_PAGE_SIZE;
    let _ = marine
        .call_with_ivalues_async(
            FACADE_MODULE,
            "allocate_two_modules_64KB_pieces",
            &[IValue::U32(pages_to_allocate as u32)],
            CallParameters::default(),
        )
        .await
        .expect("Should successfully allocate");

    let new_memory = get_total_memory(marine);
    assert!(new_memory > start_memory)
}
