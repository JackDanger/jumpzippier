# bcjzippy

> **Part of the [7-zippy](https://github.com/JackDanger/7zippy) family** — pure-Rust compression tooling.
> Full suite: `cargo add sevenzippy`  |  This crate: `cargo add bcjzippy`

Pure-Rust BCJ2 (Branch/Call/Jump filter v2) encoder/decoder for 32-bit x86 executables.
7z method ID: `[0x03, 0x03, 0x01, 0x1B]`.

**BCJ2 is not the same as BCJ.** Simple BCJ is a stateless 1-stream address
rewriter for 6 architectures (x86, ARM, ARM-Thumb, PPC, IA-64, SPARC) and is
folded into 7-zippy's in-tree pipeline. BCJ2 is a complex x86-only 4-stream
filter with a range encoder — it needs its own crate for Phase 2 native
optimization work.

## Use as a library

```toml
[dependencies]
bcjzippy = "0.0.3"
```

```rust
// Encode: split x86 binary into 4 streams for 7z storage
let [main, call, jump, rc] = bcjzippy::encode::encode_4streams(&binary_bytes);

// Decode: reassemble 4 streams back into x86 binary
let binary = bcjzippy::decode::decode_4streams([&main, &call, &jump, &rc], original_size)?;
```

## Build & Test

```sh
cargo build
cargo test
cargo bench --no-run   # verify bench targets compile
```

## 4-stream topology

BCJ2 splits a single x86 byte stream into 4 streams:

1. `main` — bytes that are neither CALL nor JMP targets
2. `call` — 4-byte relative addresses from `CALL` (`0xE8`) instructions
3. `jump` — 4-byte relative addresses from `JMP` (`0xE9`) and `Jcc` (`0x0F 0x8x`) instructions
4. `range_coder` — range-coder bits that classify each potential branch opcode

In a 7z archive the `main` stream is LZMA-compressed; streams 2–4 are stored raw.

See [STATUS.md](./STATUS.md) for the current implementation state.
