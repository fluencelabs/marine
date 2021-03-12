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

use crate::ManifestError;
use crate::ModuleManifest;

use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ByteEncoder {
    buffer: Vec<u8>,
}

impl ByteEncoder {
    pub fn new() -> Self {
        <_>::default()
    }

    pub fn add_u64(&mut self, number: u64) {
        use std::io::Write;

        let number_le_bytes = number.to_le_bytes();
        self.buffer
            .write(&number_le_bytes)
            .expect("writing to buffer should be successful");
    }

    pub fn add_utf8_string(&mut self, str: &str) {
        use std::io::Write;

        let str_as_bytes = str.as_bytes();
        self.buffer
            .write(&str_as_bytes)
            .expect("writing to buffer should be successful");
    }

    pub fn add_utf8_field(&mut self, field: &str) {
        let field_len = field.as_bytes().len();

        self.add_u64(field_len as u64);
        self.add_utf8_string(field);
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    #[allow(dead_code)]
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}

#[test]
fn test_reading_simple_config() {
    let authors = "authors".to_string();
    let version = semver::Version::from_str("0.1.0").unwrap();
    let description = "description".to_string();
    let repository = "repository".to_string();

    let mut array = ByteEncoder::new();

    array.add_utf8_field(&authors);
    array.add_utf8_field(&version.to_string());
    array.add_utf8_field(&description);
    array.add_utf8_field(&repository);

    let actual: ModuleManifest = array
        .as_bytes()
        .try_into()
        .expect("module manifest should be deserialized correctly");

    let expected = ModuleManifest {
        authors,
        version,
        description,
        repository,
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_too_big_field_len() {
    let mut array = ByteEncoder::new();

    array.add_utf8_field("authors");
    let incorrect_size = u64::MAX;
    array.add_u64(incorrect_size);
    array.add_utf8_string("version");
    array.add_utf8_field("description");
    array.add_utf8_field("repository");

    let actual: Result<ModuleManifest, _> = array.as_bytes().try_into();
    let expected = Err(ManifestError::TooBigFieldSize("version", incorrect_size));

    assert_eq!(actual, expected);
}

#[test]
fn test_without_one_field() {
    let mut array = ByteEncoder::new();

    array.add_utf8_field("authors");
    array.add_utf8_field("0.1.0");
    array.add_utf8_field("description");

    let actual: Result<ModuleManifest, _> = array.as_bytes().try_into();
    let expected = Err(ManifestError::NotEnoughBytesForPrefix("repository"));

    assert_eq!(actual, expected);
}

#[test]
fn test_with_empty_slice() {
    let actual: Result<ModuleManifest, _> = vec![].as_slice().try_into();
    let expected = Err(ManifestError::NotEnoughBytesForPrefix("authors"));

    assert_eq!(actual, expected);
}

#[test]
fn test_not_enough_data_for_field() {
    let mut array = ByteEncoder::new();

    array.add_utf8_field("authors");
    array.add_utf8_field("0.1.0");
    array.add_utf8_field("description");
    let too_big_size = 0xFF;
    array.add_u64(too_big_size);
    array.add_utf8_string("repository");

    let actual: Result<ModuleManifest, _> = array.as_bytes().try_into();
    let expected = Err(ManifestError::NotEnoughBytesForField(
        "repository",
        too_big_size as usize,
    ));

    assert_eq!(actual, expected);
}
