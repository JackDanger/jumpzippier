//! BCJ2 decoder — Phase 1 wrapper around `lzma-rust2::filter::bcj2`.
//!
//! BCJ2 is a multi-stream filter that reassembles 4 separate input streams
//! into a single x86 executable byte stream. The 4 streams are:
//!
//! - Stream 0 (`main`): raw bytes that are neither CALL nor JMP instruction targets
//! - Stream 1 (`call`): 4-byte relative addresses for `CALL` instructions
//! - Stream 2 (`jump`): 4-byte relative addresses for `JMP`/`Jcc` instructions
//! - Stream 3 (`range_coder`): range-coder probabilities used to classify bytes
//!
//! ## 7z folder topology for BCJ2
//!
//! In a 7z archive, BCJ2 is always paired with LZMA or LZMA2 in a multi-coder
//! folder. The typical topology (from the 7z SDK) is:
//!
//! ```text
//! packed[0] → LZMA decoder → BCJ2 input[1]  (main stream, LZMA-compressed)
//! packed[1] → BCJ2 input[2]                 (call stream, often raw)
//! packed[2] → BCJ2 input[3]                 (jump stream, often raw)
//! packed[3] → BCJ2 input[0]                 (range_coder stream, raw)
//!             BCJ2 output → decoded bytes
//! ```
//!
//! 7zippy's `decode_bcj2_folder` in `src/pipeline/bcj2_folder.rs` handles this
//! topology at the 7zippy level. This module provides the raw 4-stream decode.

use std::io::Read;

use lzma_rust2::filter::bcj2::Bcj2Reader;

use crate::error::{BcjzippyError, BcjzippyResult};

/// Decode a BCJ2-filtered stream from 4 separate input byte slices.
///
/// The caller is responsible for decompressing individual streams (e.g. with
/// LZMA) before calling this function; this function only handles BCJ2
/// reassembly.
///
/// # Arguments
///
/// - `streams`: exactly 4 byte slices in order: `[main, call, jump, range_coder]`
/// - `uncompressed_size`: the expected output size in bytes
///
/// # Errors
///
/// Returns `BcjzippyError::Io` or `Backend` on decompression failure.
pub fn decode_4streams(streams: [&[u8]; 4], uncompressed_size: u64) -> BcjzippyResult<Vec<u8>> {
    let readers: Vec<std::io::Cursor<&[u8]>> =
        streams.iter().map(|s| std::io::Cursor::new(*s)).collect();

    let mut reader = Bcj2Reader::new(readers, uncompressed_size);
    let mut out = Vec::with_capacity(uncompressed_size as usize);
    reader
        .read_to_end(&mut out)
        .map_err(BcjzippyError::backend)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that BCJ2 decode on the trivial empty case works.
    /// (All 4 streams empty → empty output)
    #[test]
    fn decode_empty_streams() {
        let result = decode_4streams([&[], &[], &[], &[]], 0);
        assert!(result.is_ok(), "empty streams should succeed: {result:?}");
        assert_eq!(result.unwrap().len(), 0);
    }
}
