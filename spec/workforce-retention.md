# Spec: Workforce Retention

- **Module**: [`src/workforce_retention.rs`](../src/workforce_retention.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/workforce-retention.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `CostPerLeaver`

The cash components of replacing one leaver. All fields are in the same
currency.

- `recruitment: f64` — advertising, agency fees, interviews.
- `onboarding_ramp: f64` — months of reduced productivity, supervision.
- `vacancy_cover: f64` — agency/locum premium over the vacancy duration (see
  `vacancy_cover_cost`).

### `CostPerLeaver::total(&self) -> f64`

- Formula: `recruitment + onboarding_ramp + vacancy_cover`
- Total function: never returns `None`.
- Worked example: `CostPerLeaver { recruitment: 4_500.0, onboarding_ramp: 6_000.0, vacancy_cover: 8_000.0 }.total() == 18_500.0`

### `vacancy_cover_cost(agency_premium_per_month: f64, vacancy_months: f64, wte_fraction_covered: f64) -> f64`

- Formula: `agency_premium_per_month * vacancy_months * wte_fraction_covered`
- Total function: never returns `None`.
- Worked example: `vacancy_cover_cost(10_000.0 / 3.0, 4.0, 0.6) ≈ 8_000.0` (tolerance 1.0)

### `annual_turnover_cost(headcount: f64, turnover_rate: f64, cost_per_leaver: f64) -> f64`

- Formula: `headcount * turnover_rate * cost_per_leaver`
- Total function: never returns `None`.
- Worked example: `annual_turnover_cost(1_200.0, 0.11, 18_500.0) == 2_442_000.0`

### `retention_value(headcount: f64, turnover_rate_reduction: f64, cost_per_leaver: f64) -> f64`

- Formula: `headcount * turnover_rate_reduction * cost_per_leaver`
- Total function: never returns `None`.
- Worked example: `retention_value(1_200.0, 0.01, 18_500.0) == 222_000.0`

### `software_value(headcount: f64, turnover_rate_reduction: f64, cost_per_leaver: f64, sickness_days_avoided: f64, cover_cost_per_day: f64) -> f64`

- Formula: `retention_value(headcount, turnover_rate_reduction, cost_per_leaver) + sickness_days_avoided * cover_cost_per_day`
- Total function: never returns `None`.
- Worked example: `software_value(1_200.0, 0.01, 18_500.0, 100.0, 250.0) == 247_000.0`

## Invariants

- `software_value` with `sickness_days_avoided == 0.0` and
  `cover_cost_per_day == 0.0` reduces exactly to `retention_value` — the
  module's doctest asserts `software_value(1_200.0, 0.01, 18_500.0, 0.0, 0.0) == 222_000.0`,
  matching `retention_value(1_200.0, 0.01, 18_500.0)`.
- `vacancy_cover` (a `CostPerLeaver` field) and vacancy-cover-derived agency
  spend must not be double-counted against a separate agency-savings claim —
  the module's rustdoc flags this explicitly as a reconciliation pitfall
  between `software_value`'s components.
