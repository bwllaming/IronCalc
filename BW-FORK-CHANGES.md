# BW Fork Changes

This fork carries Beyond Work formula-semantic adjustments that are required
for Conductor's IronCalc adapter evidence.

## Text Formula UTF-16 And TRIM Parity

- `base/src/functions/text/common.rs`
  - `FIND` returns one-based UTF-16 code-unit positions.
  - `LEN` counts UTF-16 code units.
  - `LEFT`, `RIGHT`, and `MID` slice by UTF-16 code-unit counts without
    emitting partial surrogate pairs.
  - `TRIM` removes/collapses ASCII space (`U+0020`) runs while preserving
    other whitespace such as NBSP.
