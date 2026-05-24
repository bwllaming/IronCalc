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

## PROPER Text Formula

- `base/src/functions/mod.rs`
  - Registers the English `PROPER` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/text/common.rs`
  - Adds scalar `PROPER(text)` title-casing for text, numeric, boolean, and
    blank inputs through the existing text coercion path.

## VALUETOTEXT Text Formula

- `base/src/functions/text/common.rs`
  - Accepts the optional `format` argument for scalar `VALUETOTEXT(value,
    [format])` calls so Conductor can route the checked expansion evidence
    through the IronCalc adapter. Strict formatting remains gated by Conductor
    migration-safe routing until Excel oracle coverage is added.

## REPLACE Text Formula

- `base/src/functions/mod.rs`
  - Registers the English `REPLACE` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/text/common.rs`
  - Adds scalar `REPLACE(old_text, start_num, num_chars, new_text)` evaluation
    with Excel-style one-based positions, truncating numeric offsets and
    appending replacement text when `start_num` is past the source text.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `REPLACE` as four scalar arguments with scalar output.

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

## FORECAST.ETS.SEASONALITY Statistical Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for the checked unprefixed
    `FORECAST.ETS.SEASONALITY` stat/compat representative so `CalcEngine`
    returns Excel's OOXML-compatible `#NAME?` posture while true ETS
    seasonality semantics remain unimplemented in the fork.

## TREND Statistical Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded local-reference
    `TREND(known_y, known_x, new_x)` and `INDEX(TREND(...), row, column)` cases
    so `CalcEngine` can match the checked stat-database representative while
    upstream IronCalc still marks `TREND` unimplemented.

## QUARTILE Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English legacy `QUARTILE` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar legacy `QUARTILE(array, quart)` evaluation using Excel's
    inclusive percentile interpolation for quart values 0 through 4.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `QUARTILE` as vector-plus-scalar input with scalar output.

## QUARTILE.EXC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `QUARTILE.EXC` function name for Conductor
    stat/compat formula evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `QUARTILE.EXC(array, quart)` evaluation by delegating quartile
    positions 1 through 3 to Excel's exclusive percentile interpolation.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `QUARTILE.EXC` as vector-plus-scalar input with scalar output.

## QUARTILE.INC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers `_xlfn.QUARTILE.INC` for Conductor stat/compat formula evidence
    while leaving unprefixed `QUARTILE.INC` unresolved to match Excel's checked
    OOXML `#NAME?` posture.
- `base/src/functions/statistical/count_and_average.rs`
  - Reuses the scalar inclusive quartile interpolation used by legacy
    `QUARTILE(array, quart)` for quart values 0 through 4.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `_xlfn.QUARTILE.INC` as vector-plus-scalar input with scalar
    output.

## MODE.SNGL Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `_xlfn.MODE.SNGL` / `MODE.SNGL` function name for
    Conductor stat/compat formula evidence without changing localized language
    payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `MODE.SNGL(number1, [number2], ...)` evaluation over numeric
    scalar, array, and range inputs, returning `#N/A` when no duplicate numeric
    value exists.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `MODE.SNGL` as vector input with scalar output.

## PERCENTILE.INC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers `_xlfn.PERCENTILE.INC` for Conductor formula expansion evidence
    while leaving unprefixed `PERCENTILE.INC` unresolved to match Excel's
    checked OOXML `#NAME?` posture.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `PERCENTILE.INC(array, k)` evaluation using Excel's inclusive
    percentile interpolation for `k` values from 0 through 1.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `_xlfn.PERCENTILE.INC` as vector-plus-scalar input with scalar
  output.

## PERCENTRANK.INC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers `_xlfn.PERCENTRANK.INC` for Conductor stat/compat formula
    evidence while leaving unprefixed `PERCENTRANK.INC` unresolved to match
    Excel's checked OOXML `#NAME?` posture.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `PERCENTRANK.INC(array, x, [significance])` evaluation using
    Excel's inclusive percent-rank interpolation and significance rounding.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `_xlfn.PERCENTRANK.INC` as vector-plus-scalar input with scalar
    output and optional scalar significance.

## PERCENTILE.EXC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `PERCENTILE.EXC` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `PERCENTILE.EXC(array, k)` evaluation using Excel's exclusive
    percentile interpolation and `#NUM!` bounds for `k`.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `PERCENTILE.EXC` as vector-plus-scalar input with scalar output.

## PERMUTATIONA Statistical Formula

- `base/src/functions/mod.rs`
  - Registers `_xlfn.PERMUTATIONA` for Conductor stat/compat formula evidence
    while leaving unprefixed `PERMUTATIONA` unresolved to match Excel's checked
    OOXML `#NAME?` posture.
- `base/src/functions/math_and_trigonometry/mathematical.rs`
  - Adds scalar `PERMUTATIONA(number, number_chosen)` evaluation for
    non-negative numeric arguments using Excel-style integer truncation.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `_xlfn.PERMUTATIONA` as two scalar inputs with scalar output.

## PERMUT Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `PERMUT` function name for Conductor stat/compat
    formula evidence without changing localized language payloads.
- `base/src/functions/math_and_trigonometry/mathematical.rs`
  - Adds scalar `PERMUT(number, number_chosen)` evaluation with Excel-style
    integer truncation and `#NUM!` bounds checks.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `PERMUT` as two scalar inputs with scalar output.

## PERCENTRANK.EXC Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `PERCENTRANK.EXC` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `PERCENTRANK.EXC(array, x, [significance])` evaluation using
    Excel's exclusive percent-rank interpolation and checked `#N/A`/`#NUM!`
    error postures.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `PERCENTRANK.EXC` as vector-plus-scalar input with optional
    scalar significance and scalar output.

## RATE Financial Formula

- `base/src/functions/financial.rs`
  - Accepts the sixth optional `guess` argument for `RATE(nper, pmt, pv, [fv],
    [type], [guess])`. The implementation already parsed the value but rejected
    six-argument calls before evaluation.

## FORECAST.LINEAR Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `FORECAST.LINEAR` / `_xlfn.FORECAST.LINEAR`
    function name for Conductor stat/compat formula evidence without changing
    localized language payloads.
- `base/src/functions/statistical/correl.rs`
  - Adds scalar `FORECAST.LINEAR(x, known_y's, known_x's)` evaluation using
    the same paired linear-regression accumulators as `SLOPE` and `INTERCEPT`.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `FORECAST.LINEAR` as scalar-plus-two-vector input with scalar
    output.

## PROB Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `PROB` function name for Conductor formula expansion
    evidence without changing localized language payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `PROB(x_range, prob_range, lower_limit, [upper_limit])`
    evaluation over numeric range and array inputs, with checked probability
    sum and shape errors.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `PROB` as vector probability inputs plus scalar limits with
    scalar output.

## MINVERSE Matrix Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded local square-matrix
    `MINVERSE` evaluation and `INDEX(MINVERSE(...), row, column)` scalar
    extraction so `CalcEngine` can compare the checked Excel expansion
    representative and full spill output while upstream IronCalc remains
    missing the function.

## FREQUENCY Statistical Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded local-reference
    `FREQUENCY(data_array, bins_array)` evaluation and
    `INDEX(FREQUENCY(...), row)` scalar extraction so `CalcEngine` can compare
    the checked stat/compat Excel representative and full vertical spill
    output while upstream IronCalc remains missing the function.

## LINEST Statistical Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded same-shape
    local-reference `LINEST(known_y, known_x)` one-variable regression
    coefficient output and `INDEX(LINEST(...), row, column)` scalar extraction
    so `CalcEngine` can compare the checked stat-database Excel representative
    and full 1x2 coefficient spill while upstream IronCalc remains missing the
    function.

## LOGEST Statistical Formula

- `fork/sheets-engine/src/lib.rs`
  - Adds a narrow Conductor adapter extension for bounded same-shape
    local-reference `LOGEST(known_y, known_x)` one-variable exponential
    regression coefficient output and `INDEX(LOGEST(...), row, column)` scalar
    extraction so `CalcEngine` can compare the checked stat-database Excel
    representative and full 1x2 coefficient spill while upstream IronCalc
    remains missing the function.

## GROWTH Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `GROWTH` function name for Conductor formula
    expansion evidence without changing localized language payloads.
- `base/src/functions/statistical/growth.rs`
  - Adds Excel-compatible one-dimensional exponential regression for
    `GROWTH(known_y, [known_x], [new_x], [const])`, returning a dynamic array
    shaped like `new_x`.
- `base/src/functions/lookup_and_reference/mod.rs`
  - Allows `INDEX` to select a scalar from dynamic-array expression results so
    `INDEX(GROWTH(...), row, column)` follows the checked Excel representative.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `GROWTH` as vector regression inputs plus optional scalar
    intercept flag with dynamic-array output.

## TRIMMEAN Statistical Formula

- `base/src/functions/mod.rs`
  - Registers the English `TRIMMEAN` function name for Conductor
    stat-database formula evidence without changing localized language
    payloads.
- `base/src/functions/statistical/count_and_average.rs`
  - Adds scalar `TRIMMEAN(array, percent)` evaluation over numeric scalar,
    array, and range inputs using Excel's even two-sided trim count.
- `base/src/expressions/parser/static_analysis.rs`
  - Classifies `TRIMMEAN` as vector-plus-scalar input with scalar output.
