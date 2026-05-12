# jumpzippier

Pure-Rust BCJ2 (Branch/Call/Jump filter v2) decoder, part of the
[8z](https://github.com/JackDanger/7zippy) umbrella of pure-Rust compression codecs.

BCJ2 is a 4-stream branch converter for 32-bit x86 executables (7z method ID `[0x03,0x03,0x01,0x1B]`).
It is always paired with LZMA or LZMA2 in 7z archives.

See [STATUS.md](./STATUS.md) for the current implementation state.

## Build & Test

```sh
cargo build
cargo test
cargo bench --no-run   # verify bench targets compile
```

## 4-stream topology

BCJ2 requires exactly 4 input streams:
1. `main`: raw bytes that are neither CALL nor JMP targets
2. `call`: 4-byte relative addresses for CALL instructions
3. `jump`: 4-byte relative addresses for JMP/Jcc instructions
4. `range_coder`: range-coder probabilities for instruction classification

In 7z archives, streams 1-3 are typically LZMA-compressed while stream 4
(range_coder) is stored raw.
