---
layout: doc
outline: deep
lang: en-US
---

# COUPDAYSNC

Returns the number of days from the settlement date to the next coupon date.

## Syntax

```plaintext
COUPDAYSNC(settlement, maturity, frequency, [basis])
```

## Arguments

- `settlement`: The security settlement date.
- `maturity`: The security maturity date.
- `frequency`: The number of coupon payments per year. Supported values are
  `1`, `2`, and `4`.
- `basis`: Optional day-count basis. Defaults to `0`.

## Example

```plaintext
=COUPDAYSNC(DATE(2024,3,1),DATE(2026,7,1),2,0)
```

Returns `120`.
