//! jumpzippier — Pure-Rust BCJ2 (Branch/Call/Jump filter v2) decoder, part of the 8z umbrella.
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
//! ## Phase 1 vs Phase 2
//!
//! Phase 1 wraps `lzma-rust2::filter::bcj2::Bcj2Reader`. Phase 2 will replace
//! this with jumpzippier's own native 4-stream coordination.
//!
//! ## Encode
//!
//! BCJ2 encoder is not implemented in Phase 1. The encoder splits a single byte
//! stream into 4 streams — this requires separate encode paths for each stream type.
//! 7-Zip itself always pairs BCJ2 with LZMA for new archives; Phase 2 will add
//! the encoder when native BCJ2 is implemented.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod decode;
pub mod error;

#[cfg(test)]
mod tests;
