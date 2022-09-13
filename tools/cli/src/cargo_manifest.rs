use cargo_toml::{Dependency, Error as CargoTomlError, Manifest};
use toml::de::Error as TomlError;
use semver::Version;
use thiserror::Error as ThisError;

use std::path::Path;
use std::str::FromStr;

const SKD_CRATE_NAME: &str = "marine-rs-sdk";

#[derive(Debug, ThisError)]
pub enum ManifestError {
    #[error("Cannot load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cannot parse file: {0}")]
    ParseError(#[from] TomlError),
    #[error("Cannot find marine-rs-sdk dependency")]
    NoSdkDependencyError,
    #[error("Cannot find version of marine-rs-sdk dependency")]
    NoSdkVersionError,
    #[error("Cannot parse marine-rs-sdk version: {0}")]
    VersionParseError(#[from] semver::Error),
}

pub(crate) fn extract_sdk_version(path: &Path) -> Result<Version, ManifestError> {
    let path = Path::new(&path);
    let manifest = Manifest::from_path(&path).map_err(|e| -> ManifestError {
        match e {
            CargoTomlError::Parse(e) => e.into(),
            CargoTomlError::Io(e) => e.into(),
        }
    })?;

    let dependency = manifest
        .dependencies
        .get(SKD_CRATE_NAME)
        .ok_or(ManifestError::NoSdkDependencyError)?;

    let version = match dependency {
        Dependency::Simple(version) => version,
        Dependency::Detailed(detail) => detail
            .version
            .as_ref()
            .ok_or(ManifestError::NoSdkVersionError)?,
    };

    Version::from_str(&version).map_err(Into::into)
}
