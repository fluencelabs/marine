use cargo_toml::Error as CargoTomlError;
use cargo_toml::Manifest;
use toml::de::Error as TomlError;
use semver::Version;
use thiserror::Error as ThisError;

use std::path::Path;

const SDK_CRATE_NAME: &str = "marine-rs-sdk";

#[derive(Debug, ThisError)]
pub enum ManifestError {
    #[error("No {SDK_CRATE_NAME} dependency found, wasm will be skipped")]
    NonMarineWasm,
    #[error("Cannot load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cannot parse file: {0}")]
    ParseError(#[from] TomlError),
    #[error("Cannot process cargo manifest because of: {0}")]
    CannotProcessManifest(String),
    #[error("Cannot find entry for {package_name}@{package_version} in Cargo.lock")]
    PackageNotFoundInLockfile {
        package_name: String,
        package_version: String,
    },
    #[error("Cannot find {SDK_CRATE_NAME} dependency for {package_name}@{package_version} in Cargo.lock")]
    SdkDependencyNotFoundInLockfile {
        package_name: String,
        package_version: String,
    },
    #[error("No package in manifest {0}")]
    NoPackageInManifest(String),
}

pub(crate) fn extract_sdk_version(
    path: &Path,
    lockfile: &cargo_lock::Lockfile,
) -> Result<Version, ManifestError> {
    let path = Path::new(&path);
    let manifest = Manifest::from_path(path).map_err(|e| -> ManifestError {
        match e {
            CargoTomlError::Parse(e) => e.into(),
            CargoTomlError::Io(e) => e.into(),
            CargoTomlError::InheritedUnknownValue => {
                ManifestError::CannotProcessManifest("inherited unknown value".to_string())
            }
            CargoTomlError::WorkspaceIntegrity(reason) => {
                ManifestError::CannotProcessManifest(reason)
            }
            CargoTomlError::Other(reason) => {
                ManifestError::CannotProcessManifest(reason.to_string())
            }
            _ => ManifestError::CannotProcessManifest("Unknown".to_string()),
        }
    })?;

    if !manifest.dependencies.contains_key(SDK_CRATE_NAME) {
        return Err(ManifestError::NonMarineWasm);
    }

    let package = manifest
        .package
        .ok_or_else(|| ManifestError::NoPackageInManifest(format!("{}", path.display())))?;

    extract_sdk_version_from_lockfile(&package, lockfile)
}

fn extract_sdk_version_from_lockfile(
    target_package: &cargo_toml::Package,
    lockfile: &cargo_lock::Lockfile,
) -> Result<Version, ManifestError> {
    let package = lockfile
        .packages
        .iter()
        .find(|package| {
            package.name.as_str() == target_package.name.as_str()
                && package.version.to_string() == target_package.version()
        })
        .ok_or_else(|| ManifestError::PackageNotFoundInLockfile {
            package_name: target_package.name.clone(),
            package_version: target_package.version().to_string(),
        })?;

    package
        .dependencies
        .iter()
        .find(|dependency| dependency.name.as_str() == SDK_CRATE_NAME)
        .map(|dependency| dependency.version.clone())
        .ok_or_else(|| ManifestError::SdkDependencyNotFoundInLockfile {
            package_name: target_package.name.clone(),
            package_version: target_package.version().to_string(),
        })
}
