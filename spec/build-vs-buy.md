# Spec: Build vs Buy

- **Module**: [`src/build_vs_buy.rs`](../src/build_vs_buy.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/build-vs-buy.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `risk_adjusted_build_cost(estimated_cost: f64, overrun_factor: f64) -> f64`

- Formula: `estimated_cost * overrun_factor`
- Total function: never returns `None`.
- Worked example: `risk_adjusted_build_cost(600_000.0, 1.35) == 810_000.0`

### `risk_adjusted_time_to_value(estimated_months: f64, delay_uplift: f64) -> f64`

- Formula: `estimated_months * (1.0 + delay_uplift)`
- Total function: never returns `None`.
- Worked example: `risk_adjusted_time_to_value(12.0, 0.5) == 18.0`

### `total_cost_of_ownership(upfront_cost: f64, annual_running_cost: f64, years: f64) -> f64`

- Formula: `upfront_cost + annual_running_cost * years`
- Total function: never returns `None`.
- Worked example: `total_cost_of_ownership(0.0, 150_000.0, 5.0) == 750_000.0`; `total_cost_of_ownership(810_000.0, 120_000.0, 5.0) == 1_410_000.0`

### `cost_of_delay(value_per_month: f64, months_later: f64) -> f64`

- Formula: `value_per_month * months_later`
- Total function: never returns `None`.
- Worked example: `cost_of_delay(25_000.0, 15.0) == 375_000.0`

### `effective_cost(tco: f64, delay_cost: f64) -> f64`

- Formula: `tco + delay_cost`
- Total function: never returns `None`. The faster option carries a `delay_cost` of `0.0`.
- Worked example: `effective_cost(1_410_000.0, 375_000.0) == 1_785_000.0`

## Invariants

- `risk_adjusted_build_cost` should feed the `upfront_cost` argument of
  `total_cost_of_ownership` for the build option; `total_cost_of_ownership`
  itself does not apply the overrun factor.
- `cost_of_delay`'s `months_later` argument should be the difference between
  two `risk_adjusted_time_to_value` outputs (the slower option's
  risk-adjusted time-to-value minus the faster option's), not raw estimates.
- `effective_cost` is meant to be called once per option, with the faster
  option's `delay_cost` set to `0.0` and the slower option's `delay_cost` set
  to its `cost_of_delay` output — the worked example composes
  `total_cost_of_ownership` and `cost_of_delay` this way before comparing
  `effective_cost` totals.
