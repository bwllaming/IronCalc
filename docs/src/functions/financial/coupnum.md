---
layout: doc
outline: deep
lang: en-US
---

# COUPNUM

Returns the number of coupons payable between the settlement and maturity dates.

## Syntax

```text
COUPNUM(settlement, maturity, frequency, [basis])
```

## Example

```text
=COUPNUM(DATE(2024,3,1),DATE(2026,7,1),2,0)
```
