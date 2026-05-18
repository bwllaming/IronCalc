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

## NUMBERVALUE Text Formula

- `base/src/functions/mod.rs`
  - Registers the English `NUMBERVALUE` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/text/common.rs`
  - Adds scalar `NUMBERVALUE(text, [decimal_separator], [group_separator])`
    parsing for explicit one-character decimal/group separators and percent
    suffixes.

## ADDRESS Lookup/Reference Formula

- `base/src/functions/mod.rs`
  - Registers the English `ADDRESS` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/lookup_and_reference/mod.rs`
  - Adds scalar `ADDRESS(row_num, column_num, [abs_num], [a1], [sheet_text])`
    evaluation for A1/R1C1 text output, bounds checks, and sheet-name quoting.
