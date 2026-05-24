---
layout: doc
outline: deep
lang: en-US
---

# COUPDAYS

Returns the number of days in the coupon period that contains the settlement
date.

## Syntax

```text
COUPDAYS(settlement, maturity, frequency, [basis])
```

## Arguments

- `settlement`: security settlement date as an Excel serial date.
- `maturity`: security maturity date as an Excel serial date.
- `frequency`: coupon frequency, either annual (`1`), semiannual (`2`), or
  quarterly (`4`).
- `basis`: optional day-count basis from `0` to `4`. Defaults to `0`.
