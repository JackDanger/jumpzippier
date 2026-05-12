# jumpzippier STATUS

**Current focus:** Phase 1 wrapper complete (lzma-rust2 bcj2 backend). Phase 2: native BCJ2 4-stream impl.

| Piece | Status |
|---|---|
| 4-stream decode (wrapper) | ✅ (lzma-rust2 Bcj2Reader) |
| oracle (round-trip vs 7zz) | ✅ (via 7zippy layer5_cross) |
| encoder | ⬜ (Phase 2 — splits 1 stream into 4) |
| streaming | ⬜ |
| decode bench | ⬜ (needs pre-split fixture) |
| fuzz | ⬜ |

**Phase 1 backend:** `lzma-rust2 v0.16` (`filter::bcj2::Bcj2Reader`).
**Phase 2:** Replace with jumpzippier's own native 4-stream coordination.

Symbols: ⬜ not started, 🟡 in progress, ✅ done, ❌ blocked.
