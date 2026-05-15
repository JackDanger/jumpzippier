//! jumpzippier — Pure-Rust BCJ2 (Branch/Call/Jump filter v2) codec, part of the 8z umbrella.
//!
//! BCJ2 is a 4-stream branch converter for 32-bit x86 executables. It splits
//! the input into:
//! - Stream 0 (`main`): bytes that are neither CALL nor JMP targets
//! - Stream 1 (`call`): 4-byte relative addresses from CALL instructions
//! - Stream 2 (`jump`): 4-byte relative addresses from JMP/Jcc instructions
//! - Stream 3 (`range_coder`): range-coder probabilities for predicting instruction types
//!
//! 7z method ID: `[0x03, 0x03, 0x01, 0x1B]`, `NumInStreams=4`, `NumOutStreams=1`.
//!
//! ## Encode
//!
//! [`encode::encode_4streams`] splits a flat byte stream into the 4 BCJ2 sub-streams
//! ready for storing in a 7z archive. The main stream should then be LZMA-compressed
//! by the caller (7zippy's pipeline layer).
//!
//! ## Decode
//!
//! [`decode::decode_4streams`] reassembles 4 packed streams back into the original
//! byte sequence. Phase 1 wraps `lzma-rust2::filter::bcj2::Bcj2Reader`; Phase 2
//! will replace this with a native implementation.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod decode;
pub mod encode;
pub mod error;

#[cfg(test)]
mod tests;
