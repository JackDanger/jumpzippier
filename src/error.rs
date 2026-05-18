use std::io;
use thiserror::Error;

/// All errors produced by bcjzippy.
#[derive(Error, Debug)]
pub enum BcjzippyError {
    /// Wraps an underlying IO error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Error from the lzma-rust2 BCJ2 backend.
    #[error("BCJ2 backend error: {0}")]
    Backend(String),

    /// The BCJ2 stream requires exactly 4 input streams.
    #[error("BCJ2 requires exactly 4 input streams, got {0}")]
    WrongStreamCount(usize),
}

impl BcjzippyError {
    /// Construct a [`Backend`](BcjzippyError::Backend) error.
    pub fn backend(msg: impl ToString) -> Self {
        BcjzippyError::Backend(msg.to_string())
    }
}

/// Convenience alias used throughout bcjzippy.
pub type BcjzippyResult<T> = Result<T, BcjzippyError>;
