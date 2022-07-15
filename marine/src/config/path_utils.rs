use crate::{MarineError, MarineResult};

use thiserror::private::PathAsDisplay;

use std::path::{Path, PathBuf};

pub fn adjust_path(base: &Path, path: &Path) -> MarineResult<PathBuf> {
    if path.is_absolute() {
        return Ok(PathBuf::from(path));
    }

    let path = [base, path].iter().collect::<PathBuf>();

    path.canonicalize().map_err(|e| {
        MarineError::IOError(format!(
            "Failed to canonicalize path {}: {}",
            path.as_path().as_display(),
            e
        ))
    })
}
