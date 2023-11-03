/*
 * Copyright 2023 Fluence Labs Limited
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

mod utils;

use marine::{CallParameters, IValue, Marine, MarineError};

use once_cell::sync::Lazy;

use marine_core::MError;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::InstantiationError;

use wasmer_it::errors::InstructionErrorKind;
use wasmer_it::errors::InstructionError;
use wasmer_it::interpreter::Instruction;

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
const KB: usize = 1024;
const MB: usize = 1024 * KB;
const WASM_PAGE_SIZE: usize = 64 * KB;

#[test]
pub fn triggered_on_instantiation() {
    let faas = Marine::with_raw_config(FAIL_ON_STARTUP_CONFIG.clone());

    match faas {
        Err(MarineError::EngineError(MError::WasmBackendError(
            WasmBackendError::InstantiationError(InstantiationError::Other(_)),
        ))) => return,
        Ok(_) => panic!("Expected instantiation error, but it succeed"),
        Err(e) => panic!("Expected isntantiation error, got: {:?}", e),
    }
}
#[test]
pub fn triggered_by_single_module() {
    let mut faas = Marine::with_raw_config(LIMIT_64_MIB.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas);

    let start_memory: usize = get_total_memory(&faas);
    let to_allocate = (64 * MB - start_memory) / WASM_PAGE_SIZE + 1;

    let result = faas.call_with_ivalues(
        FACADE_MODULE,
        "allocate_single_module_64KB_pieces",
        &[IValue::U32(to_allocate as u32)],
        CallParameters::default(),
    );

    assert_eq!(get_total_memory(&faas), 64 * MB);
    match result {
        Err(MarineError::EngineError(MError::ITInstructionError(InstructionError {
            instruction: Instruction::CallCore { .. },
            error_kind: InstructionErrorKind::LocalOrImportCall { .. },
        }))) => return,
        Err(e) => panic!("Expected LocalOrImport error, got different error: {:?}", e),
        Ok(_) => panic!("Expected Trap, got success"),
    }
}

#[test]
pub fn not_triggered_near_limit_single_module() {
    let mut faas = Marine::with_raw_config(LIMIT_64_MIB.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));

    // make sure there is no free space
    fill_start_memory(&mut faas);

    let start_memory: usize = get_total_memory(&faas);
    let to_allocate_pages = (64 * MB - start_memory) / WASM_PAGE_SIZE;

    let result = faas.call_with_ivalues(
        FACADE_MODULE,
        "allocate_single_module_64KB_pieces",
        &[IValue::U32(to_allocate_pages as u32)],
        CallParameters::default(),
    );

    assert_eq!(get_total_memory(&faas), 64 * MB);
    match result {
        Ok(_) => return,
        Err(e) => {
            panic!("Expected success, got error: {:?}", e)
        }
    }
}

#[test]
pub fn triggered_by_two_modules() {
    let mut faas = Marine::with_raw_config(LIMIT_64_MIB.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));


    // make sure there is no free space
    fill_start_memory(&mut faas);

    let start_memory: usize = get_total_memory(&faas);
    let to_allocate = (64 * MB - start_memory) / 2 / WASM_PAGE_SIZE + 1;

    let result = faas.call_with_ivalues(
        FACADE_MODULE,
        "allocate_two_modules_64KB_pieces",
        &[IValue::U32(to_allocate as u32)],
        CallParameters::default(),
    );

    assert_eq!(get_total_memory(&faas), 64 * MB);
    match result {
        Err(MarineError::EngineError(MError::ITInstructionError(InstructionError {
            instruction: Instruction::CallCore { .. },
            error_kind: InstructionErrorKind::LocalOrImportCall { .. },
        }))) => return,
        Err(e) => panic!("Expected LocalOrImport error, got different error: {:?}", e),
        Ok(_) => panic!("Expected Trap, got success"),
    }
}

#[test]
pub fn not_triggered_near_limit_two_modules() {
    let mut faas = Marine::with_raw_config(LIMIT_64_MIB.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));


    // make sure there is no free space
    fill_start_memory(&mut faas);

    let start_memory: usize = get_total_memory(&faas);

    let to_allocate = (64 * MB - start_memory) / 2 / WASM_PAGE_SIZE;

    let result = faas.call_with_ivalues(
        FACADE_MODULE,
        "allocate_two_modules_64KB_pieces",
        &[IValue::U32(to_allocate as u32)],
        CallParameters::default(),
    );

    assert_eq!(get_total_memory(&faas), 64 * MB);
    match result {
        Ok(_) => return,
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
pub fn triggered_by_large_allocation_single_module() {
    let mut faas = Marine::with_raw_config(LIMIT_64_MIB.clone())
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {}", e));


    // make sure there is no free space
    fill_start_memory(&mut faas);

    let start_memory = get_total_memory(&faas);
    let to_allocate = 128 * MB;

    let result = faas.call_with_ivalues(
        FACADE_MODULE,
        "allocate_single_module_single_piece",
        &[IValue::S64(to_allocate as i64)],
        CallParameters::default(),
    );

    assert_eq!(get_total_memory(&faas), start_memory);
    match result {
        Err(MarineError::EngineError(MError::ITInstructionError(InstructionError {
            instruction: Instruction::CallCore { .. },
            error_kind: InstructionErrorKind::LocalOrImportCall { .. },
        }))) => return,
        Err(e) => panic!("Expected LocalOrImport error, got different error: {:?}", e),
        Ok(_) => panic!("Expected Trap, got success"),
    }
}

fn get_total_memory(faas: &marine::Marine) -> usize {
    faas.module_memory_stats()
        .modules
        .iter()
        .map(|stats| stats.memory_size)
        .sum()
}

fn fill_start_memory(marine: &mut Marine) {
    let start_memory = get_total_memory(marine);
    let pages_to_allocate = (start_memory / 2) / WASM_PAGE_SIZE;
    let _ = marine.call_with_ivalues(
        FACADE_MODULE,
        "allocate_two_modules_64KB_pieces",
        &[IValue::U32(pages_to_allocate as u32)],
        CallParameters::default(),
    ).expect("Should successfully allocate");

    let new_memory = get_total_memory(marine);
    assert!(new_memory > start_memory)
}