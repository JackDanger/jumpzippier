use std::io;
use thiserror::Error;

/// All errors produced by jumpzippier.
#[derive(Error, Debug)]
pub enum JumpzippierError {
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

impl JumpzippierError {
    /// Construct a [`Backend`](JumpzippierError::Backend) error.
    pub fn backend(msg: impl ToString) -> Self {
        JumpzippierError::Backend(msg.to_string())
    }
}

/// Convenience alias used throughout jumpzippier.
pub type JumpzippierResult<T> = Result<T, JumpzippierError>;
