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

## HYPERLINK Lookup/Reference Formula

- `base/src/functions/mod.rs`
  - Registers the English `HYPERLINK` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/lookup_and_reference/mod.rs`
  - Adds scalar `HYPERLINK(link_location, [friendly_name])` display-value
    evaluation. The fork does not follow, fetch, validate, or preserve external
    link targets through formula evaluation.

## NA Information Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for no-argument `NA()` so
    `CalcEngine` returns the checked Excel `#N/A` value while the upstream
    IronCalc function registry remains missing the function.

## QUARTILE Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English legacy `QUARTILE` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar legacy `QUARTILE(array, quart)` evaluation using Excel's
    inclusive percentile interpolation for quart values 0 through 4.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `QUARTILE` as vector-plus-scalar input with scalar output.

## MINVERSE Matrix Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded local square-matrix
    `MINVERSE` evaluation and `INDEX(MINVERSE(...), row, column)` scalar
    extraction so `CalcEngine` can compare the checked Excel expansion
    representative and full spill output while upstream IronCalc remains
    missing the function.
