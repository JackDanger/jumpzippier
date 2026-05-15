//! Minimal range encoder for the BCJ2 range-coder stream.
//!
//! BCJ2 only needs single-bit probability encoding, not the full LZMA encoder.
//! This is a self-contained implementation of the range coder used exclusively
//! for the BCJ2 RC output stream.
//!
//! ## Format compatibility
//!
//! The BCJ2 decoder initializes by reading the first 5 bytes of the RC stream
//! into `code` (bytes 0-3 are shifted in, range advances from 0 to 5 then sets
//! range = 0xFFFFFFFF). The encoder must therefore:
//! 1. Emit a leading `0x00` byte (matching the LZMA range encoder's standard
//!    initial-byte convention; the decoder checks `range==1 && code!=0` would
//!    fail if this byte is non-zero at the right stage).
//! 2. Write the range-coded bits.
//! 3. Flush 5 final bytes.
//!
//! The lzma-rust2 `RangeEncoder` follows exactly this convention: `cache=0`,
//! `cache_size=1` on reset, and the first byte written is always 0x00.

const NUM_MODEL_BITS: u32 = 11;
const BIT_MODEL_TOTAL: u32 = 1 << NUM_MODEL_BITS;
const NUM_MOVE_BITS: u32 = 5;
const K_TOP_VALUE: u32 = 1 << 24;

/// Initial probability value (half total = unbiased).
pub(crate) const PROB_INIT: u16 = (BIT_MODEL_TOTAL >> 1) as u16;

/// Range encoder that writes to an internal `Vec<u8>`.
pub(crate) struct RangeEncoder {
    low: u64,
    range: u32,
    cache: u8,
    cache_size: u64,
    buf: Vec<u8>,
}

impl RangeEncoder {
    pub(crate) fn new() -> Self {
        Self {
            low: 0,
            range: 0xFFFF_FFFFu32,
            cache: 0x00,
            cache_size: 1,
            buf: Vec::new(),
        }
    }

    fn shift_low(&mut self) {
        let low_hi = (self.low >> 32) as u32;
        if low_hi != 0 || self.low < 0xFF00_0000u64 {
            let mut temp = self.cache;
            while self.cache_size > 0 {
                self.buf.push(temp.wrapping_add(low_hi as u8));
                temp = 0xFF;
                self.cache_size -= 1;
            }
            self.cache = (self.low >> 24) as u8;
        }
        self.cache_size += 1;
        self.low = (self.low & 0x00FF_FFFF) << 8;
    }

    /// Encode a single bit with adaptive probability `prob`.
    ///
    /// `bit = 0` means "not a branch"; `bit = 1` means "is a branch".
    pub(crate) fn encode_bit(&mut self, prob: &mut u16, bit: u32) {
        let ttt = *prob as u32;
        let bound = (self.range >> NUM_MODEL_BITS) * ttt;
        if bit == 0 {
            self.range = bound;
            *prob = (ttt + ((BIT_MODEL_TOTAL - ttt) >> NUM_MOVE_BITS)) as u16;
        } else {
            self.low += bound as u64;
            self.range -= bound;
            *prob = (ttt - (ttt >> NUM_MOVE_BITS)) as u16;
        }
        if self.range < K_TOP_VALUE {
            self.range <<= 8;
            self.shift_low();
        }
    }

    /// Flush the remaining bits and return the RC stream bytes.
    pub(crate) fn finish(mut self) -> Vec<u8> {
        // Standard LZMA range encoder flush: shift_low 5 times.
        for _ in 0..5 {
            self.shift_low();
        }
        self.buf
    }
}
