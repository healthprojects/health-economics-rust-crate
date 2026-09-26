# Spec: Cross-Currency ICER Comparison

- **Module**: [`src/cross_currency_icer_comparison.rs`](../src/cross_currency_icer_comparison.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cross-currency-icer-comparison.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `convert(rate: &ExchangeRate<'static, iso::Currency>, amount: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `rate.convert(&amount)` — converts `amount` into another currency using an explicit, auditable exchange rate (a PPP conversion factor for comparing real economic value across countries, or a market FX rate only for modelling actual cross-border cash payment flows).
- Returns `Err(MoneyError::CurrencyMismatch)` iff `amount`'s currency does not match `rate`'s `from` currency; returns `Err(MoneyError::Overflow)` iff the conversion arithmetic overflows.
- Worked example: converting `Money::from_major(45_000, iso::USD)` with `ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(72, 2))` gives `Ok(Money::from_major(32_400, iso::GBP))`

### `adopt_at_threshold(icer_in_threshold_currency: &Money<'static, iso::Currency>, threshold: &Money<'static, iso::Currency>) -> Result<bool, MoneyError>`

- Formula: `icer_in_threshold_currency.lt(threshold)` — `Ok(true)` (adopt) if the ICER is strictly less than the threshold, `Ok(false)` (reject) otherwise.
- Returns `Err(MoneyError::CurrencyMismatch)` iff `icer_in_threshold_currency` and `threshold` are in different currencies (by design: a caller that passes an unconverted ICER is caught, not silently miscompared).
- Worked example: `adopt_at_threshold(&Money::from_major(32_400, iso::GBP), &Money::from_major(34_000, iso::GBP)) == Ok(true)`; the same ICER converted at a market rate instead of PPP gives `Money::from_major(35_550, iso::GBP)`, and `adopt_at_threshold` on that value against the same threshold gives `Ok(false)`

## Invariants

- The module's central point is that the *choice* of conversion factor
  (PPP vs market FX), not the threshold rule itself, can flip the adopt/reject
  decision: the worked example converts the identical $45,000/QALY ICER two
  ways and gets `Ok(true)` via PPP (0.72) and `Ok(false)` via market FX
  (0.79) against the same £34,000 threshold.
- `adopt_at_threshold` never silently compares mismatched currencies — this
  is enforced by `Money`'s own comparison methods (`lt`, `gt`, `eq`)
  returning `Result`, not by any check added in this module.
