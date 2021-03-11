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

use crate::ModuleInfoError;
use crate::Result;

use std::convert::TryFrom;
use std::str::FromStr;

impl TryFrom<&[u8]> for ModuleManifest {
    type Error = ModuleInfoError;

    #[rustfmt::skip]
    fn try_from(value: &[u8]) -> Result<Self> {
        let (authors, next_offset) = try_extract_field_as_string(value, 0, "authors")?;
        let (version, next_offset) = try_extract_field_as_version(value, next_offset, "version")?;
        let (description, next_offset) = try_extract_field_as_string(value, next_offset, "description")?;
        let (repository, next_offset) = try_extract_field_as_string(value, next_offset, "repository")?;

        if next_offset != value.len() {
            return Err(ModuleInfoError::ManifestRemainderNotEmpty)
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

fn try_extract_field_as_string(
    raw_manifest: &[u8],
    offset: usize,
    field_name: &'static str,
) -> Result<(String, usize)> {
    let (field_as_bytes, read_len) =
        try_extract_prefixed_field(&raw_manifest[offset..], field_name)?;
    let field_as_string = try_to_string(field_as_bytes)?;

    Ok((field_as_string, offset + read_len))
}

fn try_extract_field_as_version(
    raw_manifest: &[u8],
    offset: usize,
    field_name: &'static str,
) -> Result<(semver::Version, usize)> {
    let (field_as_bytes, read_len) =
        try_extract_prefixed_field(&raw_manifest[offset..], field_name)?;
    let field_as_str = try_to_str(field_as_bytes)?;
    let version = semver::Version::from_str(field_as_str)?;

    Ok((version, offset + read_len))
}

const PREFIX_SIZE: usize = std::mem::size_of::<u64>();

fn try_extract_prefixed_field<'a>(
    array: &'a [u8],
    field_name: &'static str,
) -> Result<(&'a [u8], usize)> {
    let field_len = try_extract_field_len(array, field_name)?;
    let field = try_extract_field(array, field_len, field_name)?;

    let read_size = PREFIX_SIZE + field.len();
    Ok((field, read_size))
}

fn try_extract_field_len(array: &[u8], field_name: &'static str) -> Result<usize> {
    if array.len() < PREFIX_SIZE {
        return Err(ModuleInfoError::ManifestCorrupted(field_name));
    }

    let mut field_len = [0u8; PREFIX_SIZE];
    field_len.copy_from_slice(&array[0..PREFIX_SIZE]);

    let field_len = u64::from_le_bytes(field_len);
    // TODO: Until we use Wasm32 and compiles our node to x86_64, converting to usize is sound
    if field_len.checked_add(PREFIX_SIZE as u64).is_none()
        || usize::try_from(field_len + PREFIX_SIZE as u64).is_err()
    {
        return Err(ModuleInfoError::ManifestCorrupted(field_name));
    }

    // it's safe to convert it to usize because it's been checked
    Ok(field_len as usize)
}

fn try_extract_field<'a>(
    array: &'a [u8],
    field_len: usize,
    field_name: &'static str,
) -> Result<&'a [u8]> {
    if array.len() < PREFIX_SIZE + field_len {
        return Err(ModuleInfoError::ManifestCorrupted(field_name));
    }

    let field = &array[PREFIX_SIZE..PREFIX_SIZE + field_len];
    Ok(field)
}

fn try_to_string(value: &[u8]) -> Result<String> {
    match std::str::from_utf8(value) {
        Ok(str) => Ok(str.to_string()),
        Err(e) => Err(ModuleInfoError::VersionNotValidUtf8(e)),
    }
}

fn try_to_str(value: &[u8]) -> Result<&str> {
    match std::str::from_utf8(value) {
        Ok(s) => Ok(s),
        Err(e) => Err(ModuleInfoError::VersionNotValidUtf8(e)),
    }
}

use std::fmt;

impl fmt::Display for ModuleManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "authors: {}", self.authors)?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "description: {}", self.description)?;
        write!(f, "repository: {}", self.repository)
    }
}
