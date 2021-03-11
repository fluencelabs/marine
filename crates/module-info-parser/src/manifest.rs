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

/// Describes manifest of a Wasm module in the Fluence network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleManifest {
    pub authors: String,
    pub version: semver::Version,
    pub description: String,
    pub repository: String,
}

use crate::ManifestParserError;
use crate::Result;

use std::convert::TryFrom;
use std::str::FromStr;

impl TryFrom<&[u8]> for ModuleManifest {
    type Error = ManifestParserError;

    fn try_from(value: &[u8]) -> Result<Self> {
        let (authors_as_bytes, read_len) = extract_prefixed_field(value, "authors")?;
        let authors = try_to_string(authors_as_bytes)?;

        let offset = read_len;
        let (version_as_bytes, read_len) = extract_prefixed_field(&value[offset..], "version")?;
        let version_as_str = try_to_str(version_as_bytes)?;
        let version = semver::Version::from_str(version_as_str)?;

        let offset = offset + read_len;
        let (description_as_bytes, read_len) = extract_prefixed_field(&value[offset..], "description")?;
        let description = try_to_string(description_as_bytes)?;

        let offset = offset + read_len;
        let (repository_as_bytes, read_len) = extract_prefixed_field(&value[offset..], "repository")?;
        let repository = try_to_string(repository_as_bytes)?;

        if offset + read_len != value.len() {
            unimplemented!();
        }

        let manifest = ModuleManifest {
            authors,
            version,
            description,
            repository,
        };

        Ok(manifest)
    }
}

fn extract_prefixed_field<'a>(array: &'a [u8], field_name: &'static str) -> Result<(&'a [u8], usize)> {
    const PREFIX_SIZE: usize = std::mem::size_of::<u64>();
    println!("array: {}, {:x?}", array.len(), array);

    if array.len() < PREFIX_SIZE {
        return Err(ManifestParserError::ManifestCorrupted(field_name));
    }

    let mut field_len = [0u8; PREFIX_SIZE];
    field_len.copy_from_slice(&array[0..PREFIX_SIZE]);
    println!("field_len bytes: {:?}", field_len);

    let field_len = u64::from_le_bytes(field_len);
    if field_len.checked_add(PREFIX_SIZE as u64).is_none() || usize::try_from(field_len).is_err() {
        return Err(ManifestParserError::ManifestCorrupted(field_name));
    }

    // it's safe because it's been checked
    let field_len = field_len as usize;

    println!("field_len: {}", field_len);

    if array.len() < PREFIX_SIZE + field_len {
        return Err(ManifestParserError::ManifestCorrupted(field_name));
    }

    let field = &array[PREFIX_SIZE..PREFIX_SIZE + field_len];
    let read_size = PREFIX_SIZE + field.len();
    Ok((field, read_size))
}

fn try_to_string(value: &[u8]) -> Result<String> {
    match std::str::from_utf8(value) {
        Ok(str) => Ok(str.to_string()),
        Err(e) => Err(ManifestParserError::VersionNotValidUtf8(e)),
    }
}

fn try_to_str(value: &[u8]) -> Result<&str> {
    match std::str::from_utf8(value) {
        Ok(s) => Ok(s),
        Err(e) => Err(ManifestParserError::VersionNotValidUtf8(e)),
    }
}
