use cargo_toml::{Error as CargoTomlError, Manifest};
use toml::de::Error as TomlError;
use semver::Version;
use thiserror::Error as ThisError;

use std::path::Path;

const SDK_CRATE_NAME: &str = "marine-rs-sdk";
const LOCKFILE_NAME: &str = "Cargo.lock";

#[derive(Debug, ThisError)]
pub enum ManifestError {
    #[error("Non-marine wasm, is to be skipped")]
    NonMarineWasm,
    #[error("Cannot load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cannot parse file: {0}")]
    ParseError(#[from] TomlError),
    #[error("Cannot process cargo manifest because of: {0}")]
    CannotProcessManifest(String),
    #[error("Cannot find lockfile in any parent directories for {0}")]
    CannotFindLockfile(String),
    #[error("No package in manifest {0}")]
    NoPackageInManifest(String),
}

pub(crate) fn extract_sdk_version(path: &Path) -> Result<Version, ManifestError> {
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

    // TODO: find & load lockfile once per `marine build` run
    path.ancestors()
        .find_map(|path| extract_sdk_version_from_lockfile(&package, &path.join(LOCKFILE_NAME)))
        .ok_or_else(|| ManifestError::CannotFindLockfile(format!("{}", path.display())))
}

fn extract_sdk_version_from_lockfile(
    target_package: &cargo_toml::Package,
    path: &Path,
) -> Option<semver::Version> {
    log::debug!("Trying to load lockfile from {}", path.display());
    let lockfile = cargo_lock::Lockfile::load(path).ok()?;
    log::debug!(
        "Lockfile loaded. Looking for entry for {}@{}",
        target_package.name.as_str(),
        target_package.version()
    );
    lockfile
        .packages
        .iter()
        .find_map(|package| {
            if package.name.as_str() == target_package.name.as_str()
                && &package.version.to_string() == target_package.version()
            {
                log::debug!("Found entry. Looking for marine-rs-sdk dependency");
                Some(package)
            } else {
                None
            }
        })?
        .dependencies
        .iter()
        .find_map(|dependency| {
            if dependency.name.as_str() == SDK_CRATE_NAME {
                log::debug!(
                    "Found marine-re-sdk dependency version {}",
                    dependency.version
                );
                Some(dependency.version.clone())
            } else {
                None
            }
        })
}
