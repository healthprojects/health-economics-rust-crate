# Spec: Discounting and Time Preference

- **Module**: [`src/discounting_and_time_preference.rs`](../src/discounting_and_time_preference.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/discounting-and-time-preference.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `present_value(future_value: f64, rate: f64, years: f64) -> f64`

- Formula: `future_value / (1.0 + rate).powf(years)`
- Total function: never returns `None`.
- Worked example: `present_value(100_000.0, NICE_REFERENCE_RATE, 1.0) ≈ 96_618.0`; `present_value(100_000.0, NICE_REFERENCE_RATE, 5.0) ≈ 84_197.0`

### `annuity_present_value(annual_benefit: f64, rate: f64, years: f64) -> f64`

- Formula: `annual_benefit * (1.0 - (1.0 + rate).powf(-years)) / rate`, except when `rate == 0.0`, which returns the r→0 limit `annual_benefit * years` instead (since the closed form is 0/0 at `rate = 0`)
- Total function: never returns `None`.
- Worked example: `annuity_present_value(100_000.0, NICE_REFERENCE_RATE, 5.0) ≈ 451_505.0`; `annuity_present_value(100_000.0, 0.0, 5.0) == 500_000.0`

### `delayed_present_value(undelayed_pv: f64, rate: f64, delay_years: f64) -> f64`

- Formula: `undelayed_pv / (1.0 + rate).powf(delay_years)`
- Total function: never returns `None`.
- Worked example: `delayed_present_value(451_505.0, NICE_REFERENCE_RATE, 1.0) ≈ 436_000.0`

### `NICE_REFERENCE_RATE: f64`

- Constant: `0.035` — the NICE reference-case / HM Treasury Green Book discount rate (3.5% per year), applied to both costs and health effects.

## Invariants

- `annuity_present_value(b, r, n)` equals the sum of `present_value(b, r, t)`
  for `t` in `1..=n` — the module's rustdoc states the closed form "equals
  the sum of `present_value` over years 1..=n," and a test verifies this
  equivalence directly.
- `delayed_present_value` composes with `annuity_present_value`: shifting
  every term of a benefit stream later by `delay_years` is equivalent to
  dividing the whole stream's present value by `(1 + rate)^delay_years` —
  "the discounting view of cost of delay," per the module's rustdoc.
