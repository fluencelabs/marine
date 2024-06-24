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

/// Describes manifest of a Wasm module in the Fluence network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleManifest {
    pub authors: String,
    pub version: semver::Version,
    pub description: String,
    pub repository: String,
    pub build_time: chrono::DateTime<chrono::FixedOffset>,
}

use super::ManifestError;

use std::convert::TryFrom;
use std::str::FromStr;

type Result<T> = std::result::Result<T, ManifestError>;

impl TryFrom<&[u8]> for ModuleManifest {
    type Error = ManifestError;

    #[rustfmt::skip]
    fn try_from(value: &[u8]) -> Result<Self> {
        let (authors, next_offset) = try_extract_field_as_string(value, 0, "authors")?;
        let (version, next_offset) = try_extract_field_as_version(value, next_offset, "version")?;
        let (description, next_offset) = try_extract_field_as_string(value, next_offset, "description")?;
        let (repository, next_offset) = try_extract_field_as_string(value, next_offset, "repository")?;
        let (build_time, next_offset) = try_extract_field_as_string(value, next_offset, "build time")?;

        if next_offset != value.len() {
            return Err(ManifestError::ManifestRemainderNotEmpty)
        }

        let build_time = chrono::DateTime::parse_from_rfc3339(&build_time)?;

        let manifest = ModuleManifest {
            authors,
            version,
            description,
            repository,
            build_time
        };

        Ok(manifest)
    }
}

fn try_extract_field_as_string(
    raw_manifest: &[u8],
    offset: usize,
    field_name: &'static str,
) -> Result<(String, usize)> {
    let raw_manifest = &raw_manifest[offset..];
    let (field_as_bytes, read_len) = try_extract_prefixed_field(raw_manifest, field_name)?;
    let field_as_string = try_to_str(field_as_bytes, field_name)?.to_string();

    Ok((field_as_string, offset + read_len))
}

fn try_extract_field_as_version(
    raw_manifest: &[u8],
    offset: usize,
    field_name: &'static str,
) -> Result<(semver::Version, usize)> {
    let raw_manifest = &raw_manifest[offset..];
    let (field_as_bytes, read_len) = try_extract_prefixed_field(raw_manifest, field_name)?;
    let field_as_str = try_to_str(field_as_bytes, field_name)?;
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
        return Err(ManifestError::NotEnoughBytesForPrefix(field_name));
    }

    let mut field_len = [0u8; PREFIX_SIZE];
    field_len.copy_from_slice(&array[0..PREFIX_SIZE]);

    let field_len = u64::from_le_bytes(field_len);
    // TODO: Until we use Wasm32 and compiles our node to x86_64, converting to usize is sound
    if field_len.checked_add(PREFIX_SIZE as u64).is_none()
        || usize::try_from(field_len + PREFIX_SIZE as u64).is_err()
    {
        return Err(ManifestError::TooBigFieldSize(field_name, field_len));
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
        return Err(ManifestError::NotEnoughBytesForField(field_name, field_len));
    }

    let field = &array[PREFIX_SIZE..PREFIX_SIZE + field_len];
    Ok(field)
}

fn try_to_str<'v>(value: &'v [u8], field_name: &'static str) -> Result<&'v str> {
    match std::str::from_utf8(value) {
        Ok(s) => Ok(s),
        Err(e) => Err(ManifestError::FieldNotValidUtf8(field_name, e)),
    }
}

use std::fmt;

impl fmt::Display for ModuleManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "authors:     {}", self.authors)?;
        writeln!(f, "version:     {}", self.version)?;
        writeln!(f, "description: {}", self.description)?;
        writeln!(f, "repository:  {}", self.repository)?;
        write!(f, "build time:  {} UTC", self.build_time)
    }
}
