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

use fluence_faas::FluenceFaaS;
use fluence_faas::IValue;

#[test]
#[ignore]
pub fn records() {
    let records_config_path = "../examples/records/Config.toml";

    let records_config_raw = std::fs::read(records_config_path)
        .expect("../examples/records/Config.toml should presence");

    let mut records_config: fluence_faas::RawModulesConfig =
        toml::from_slice(&records_config_raw).expect("records config should be well-formed");
    records_config.modules_dir = Some(String::from("../examples/records/artifacts/wasm_modules/"));

    let mut faas = FluenceFaaS::with_raw_config(records_config)
        .unwrap_or_else(|e| panic!("can't create Fluence FaaS instance: {:?}", e));

    let result = faas
        .call("pure", "invoke", &[], <_>::default())
        .unwrap_or_else(|e| panic!("can't invoke pure: {:?}", e));

    assert_eq!(
        result,
        vec![IValue::Record(
            wasmer_wit::vec1::Vec1::new(vec![
                IValue::I32(1),
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
