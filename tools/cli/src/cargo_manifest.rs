use cargo_toml::{Dependency, Error as CargoTomlError, Manifest};
use toml::de::Error as TomlError;
use semver::Version;
use thiserror::Error as ThisError;

use std::path::Path;
use std::str::FromStr;
use crate::cargo_manifest::ManifestError::CannotProcessManifest;

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
    #[error("Inherited dependencies are not supported yet")]
    InheritedDependencyUnsupported,
    #[error("Cannot process cargo manifest because of: {0}")]
    CannotProcessManifest(String),
}

pub(crate) fn extract_sdk_version(path: &Path) -> Result<Version, ManifestError> {
    let path = Path::new(&path);
    let manifest = Manifest::from_path(path).map_err(|e| -> ManifestError {
        match e {
            CargoTomlError::Parse(e) => e.into(),
            CargoTomlError::Io(e) => e.into(),
            CargoTomlError::InheritedUnknownValue => {
                CannotProcessManifest("inherited unknown value".to_string())
            }
            CargoTomlError::WorkspaceIntegrity(reason) => CannotProcessManifest(reason),
            CargoTomlError::Other(reason) => CannotProcessManifest(reason.to_string()),
            _ => CannotProcessManifest("Unknown".to_string()),
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
        Dependency::Inherited(_) => return Err(ManifestError::InheritedDependencyUnsupported),
    };

    Version::from_str(version).map_err(Into::into)
}
