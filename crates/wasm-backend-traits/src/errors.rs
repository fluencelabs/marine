use thiserror::Error;


/*
    General error design goals:
        * expose as much detail as possible
        * make as much domain-specific errors as possible implmentation-independent

    So, Error enums should follow this principle:
        * errors fully expressible without implementation info should have implementation-independent view
        * errors not fully expressible without implementation info should have some common view and a way to get implmententation-specific details
        * "Other" type for all errors not suited for listed options
 */



#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error("{0}")]
    ResolveError(#[from] ResolveError),

    #[error("{0}")]
    RuntimeError(#[from] RuntimeError),

    #[error("{0}")]
    CompilationError(#[from] CompilationError),

    #[error("{0}")]
    ImportError(#[from] ImportError),

    #[error("{0}")]
    InstantiationError(String),
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("{0}")]
    FailedToCompileWasm(anyhow::Error),
    #[error("{0}")]
    FailedToExtractCustomSections(String),
    #[error("{0}")]
    Other(anyhow::Error),
}

pub type CompilationResult<T> = Result<T, CompilationError>;

#[derive(Debug, Error)]
pub enum CallError {
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type CallResult<T> = Result<T, CallError>;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Duplcae import")]
    DuplicateImport,

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type ImportResult<T> = Result<T, ImportError>;

#[derive(Debug, Error)]
pub enum InstantiationError {
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type InstantiationResult<T> = Result<T, InstantiationError>;
