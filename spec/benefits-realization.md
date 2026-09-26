# Spec: Benefits Realization

- **Module**: [`src/benefits_realization.rs`](../src/benefits_realization.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/benefits-realization.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `BenefitClass`

Classification of a claimed benefit, per NHS/Green Book benefit frameworks.
An enum with variants:

- `CashReleasing` — reduces actual expenditure (a budget line gets smaller).
- `NonCashReleasing` — frees time or capacity that is reused rather than banked.
- `Qualitative` — described, not scored (e.g. compliance, experience).

### `Benefit`

One benefit line from a business case, with its forecast and the value
realized (measured against a pre-go-live baseline). `forecast` and
`realized` share the benefit's own units.

- `name: String` — human-readable benefit name.
- `class: BenefitClass` — benefit class; classes are tracked and reported separately.
- `forecast: f64` — value promised in the business case, in the benefit's own units.
- `realized: f64` — value evidenced post-go-live, in the same units.

### `Benefit::realization_rate(&self) -> Option<f64>`

- Formula: `self.realized / self.forecast` (delegates to `realization_rate(self.realized, self.forecast)`)
- Returns `None` iff `self.forecast == 0.0`
- Worked example: for `Benefit { forecast: 450_000.0, realized: 287_000.0, .. }`, `b.realization_rate() ≈ Some(0.64)`

### `realization_rate(realized: f64, forecast: f64) -> Option<f64>`

- Formula: `realized / forecast`
- Returns `None` iff `forecast == 0.0`
- Worked example: `realization_rate(5_100.0, 8_000.0) == Some(0.6375)`; `realization_rate(12.0, 10.0) == Some(1.20)`

### `optimism_error(forecast: f64, realized: f64) -> Option<f64>`

- Formula: `(forecast - realized) / forecast`
- Returns `None` iff `forecast == 0.0`
- Worked example: `optimism_error(450_000.0, 287_000.0) == Some(163_000.0 / 450_000.0)` (≈0.362)

### `optimism_adjusted_forecast(raw_forecast: f64, optimism_error_rate: f64) -> f64`

- Formula: `raw_forecast * (1.0 - optimism_error_rate)`
- Total function: never returns `None`.
- Worked example: `optimism_adjusted_forecast(100_000.0, 0.30) == 70_000.0`

## Invariants

- `Benefit::realization_rate` is a thin wrapper delegating to the free
  function `realization_rate(self.realized, self.forecast)` — the two must
  always agree for the same benefit.
- `optimism_error` and `realization_rate` are complementary views of the same
  gap: `optimism_error(forecast, realized) == 1.0 - realization_rate(realized, forecast).unwrap()`
  whenever both are defined (`forecast != 0.0`).
- `optimism_adjusted_forecast` is meant to be fed a `optimism_error_rate`
  taken from a prior period's `optimism_error` result, so that a team's
  historical over-forecasting haircuts its next raw forecast (the Green Book
  feedback loop).
