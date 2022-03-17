use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error("{0}")]
    ResolveError(ResolveError),

    #[error("{0}")]
    RuntimeError(RuntimeError),

    #[error("{0}")]
    CompilationError(CompilationError),

    #[error("{0}")]
    InstantiationError(String),
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("{0}")]
    Message(String),
}

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0}")]
    Message(String),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("{0}")]
    Message(String),
}

pub type CompilationResult<T> = Result<T, CompilationError>;

#[derive(Debug, Error)]
pub enum CallError {
    #[error("{0}")]
    Message(String),
}

pub type CallResult<T> = Result<T, CallError>;
