use crate::errors::CLIError;

use cargo_toml::{Dependency, Error, Manifest};
use semver::Version;

use std::path::Path;
use std::str::FromStr;

const SKD_CRATE_NAME: &str = "marine-rs-sdk";

pub(crate) fn extract_sdk_version(path: String) -> Result<Version, CLIError> {
    let path = Path::new(&path);
    let manifest = Manifest::from_path(&path).map_err(|e| match e {
        Error::Parse(e) => CLIError::ManifestParseError(e.to_string()),
        Error::Io(e) => e.into(),
    })?;
    let dependency =
        manifest
            .dependencies
            .get(SKD_CRATE_NAME)
            .ok_or(CLIError::ManifestParseError(format!(
                "Cannot find marine-rs-sdk dependency in {}",
                path.display()
            )))?;

    let version = match dependency {
        Dependency::Simple(version) => version,
        Dependency::Detailed(detail) => {
            detail
                .version
                .as_ref()
                .ok_or(CLIError::ManifestParseError(format!(
                    "No version found for marine-rs-sdk dependency in {}",
                    path.display()
                )))?
        }
    };

    Version::from_str(&version).map_err(|e| e.into())
}
