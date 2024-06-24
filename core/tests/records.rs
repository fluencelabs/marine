/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine_core::MarineCore;
use marine_core::MarineCoreConfig;
use marine_core::IValue;
use marine_wasm_backend_traits::WasmBackend;
use marine_wasmtime_backend::WasmtimeWasmBackend;

#[tokio::test]
pub async fn records() {
    let effector_wasm_bytes = std::fs::read("../examples/records/artifacts/records_effector.wasm")
        .expect("../examples/records/artifacts/records_effector.wasm should presence");

    let pure_wasm_bytes = std::fs::read("../examples/records/artifacts/records_pure.wasm")
        .expect("../examples/records/artifacts/records_pure.wasm should presence");

    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    let load_result = marine_core
        .load_module("pure", &pure_wasm_bytes, <_>::default())
        .await;
    assert!(load_result.is_err());

    marine_core
        .load_module("records_effector", &effector_wasm_bytes, <_>::default())
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    marine_core
        .load_module("records_pure", &pure_wasm_bytes, <_>::default())
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let result = marine_core
        .call_async("records_pure", "invoke", &[])
        .await
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::Record(
            wasmer_it::NEVec::new(vec![
                IValue::Boolean(true),
                IValue::S8(1),
                IValue::S16(2),
                IValue::S32(3),
                IValue::S64(4),
                IValue::U8(5),
                IValue::U16(6),
                IValue::U32(7),
                IValue::U64(8),
                IValue::F32(9.0),
                IValue::F64(10.0),
                IValue::String(String::from("field_11")),
                IValue::ByteArray(vec![0x13, 0x37])
            ])
            .unwrap()
        )]
    );
}
