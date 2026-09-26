# Spec: Currency-Safe Cost Rollup

- **Module**: [`src/currency_safe_cost_rollup.rs`](../src/currency_safe_cost_rollup.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/currency-safe-cost-rollup.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `sum_line_items(items: &[Money<'static, iso::Currency>]) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: folds `items` with `Money::add`, exact regardless of item count or summation order.
- Returns `Err(MoneyError::InvalidAmount)` iff `items` is empty (there is no well-defined total, and no currency to report one in). Also propagates any `MoneyError` `add` returns while folding — notably `Err(MoneyError::CurrencyMismatch)` if the items aren't all in the same currency, or `Err(MoneyError::Overflow)` on arithmetic overflow.
- Worked example: `sum_line_items(&vec![Money::from_minor(123_456, iso::USD); 12]).unwrap() == Money::from_minor(1_481_472, iso::USD)` (twelve $1,234.56 invoices sum to exactly $14,814.72)

### `apply_multiplier(total: Money<'static, iso::Currency>, multiplier: Decimal) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `total.mul(multiplier)` — an exact `Decimal` product that may carry more decimal places than the currency's minor-unit exponent; the caller must round explicitly afterward with a stated rounding rule.
- Returns `Err(MoneyError::Overflow)` iff the multiplication overflows.
- Worked example: `apply_multiplier(Money::from_minor(1_481_472, iso::USD), Decimal::new(105, 2)).unwrap()` then `.round(2, Round::HalfEven) == Money::from_minor(1_555_546, iso::USD)` ($14,814.72 × 1.05 = exact $15,555.456, rounds half-even to $15,555.46)

## Invariants

- `sum_line_items` is exact and order-independent by construction (it uses
  `Decimal`-backed `Money::add`), in contrast to summing the equivalent
  `f64` values, whose result the module's rustdoc states can drift with
  summation order.
- `apply_multiplier`'s result is not automatically rounded to the currency's
  minor-unit exponent — the module's rustdoc explicitly warns that the
  rounding rule (e.g. half-up vs half-even) must be a stated, auditable
  convention applied by the caller via `Money::round`, not an implicit
  default of `apply_multiplier` itself.
