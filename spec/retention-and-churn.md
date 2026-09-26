# Spec: Retention and Churn

- **Module**: [`src/retention_and_churn.rs`](../src/retention_and_churn.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/retention-and-churn.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `retention_percent(users_active_on_day_n: f64, cohort_size: f64) -> Option<f64>`

- Formula: `users_active_on_day_n / cohort_size * 100`
- Returns `None` iff `cohort_size == 0.0`
- Worked example: `retention_percent(8_000.0, 100_000.0) == Some(8.0)`

### `churn_rate_percent(users_lost_in_period: f64, users_at_period_start: f64) -> Option<f64>`

- Formula: `users_lost_in_period / users_at_period_start * 100`
- Returns `None` iff `users_at_period_start == 0.0`
- Worked example: `churn_rate_percent(92_000.0, 100_000.0) == Some(92.0)`

### `expected_benefit_per_acquired_user(retention_fractions: &[f64], benefit_rates: &[f64]) -> f64`

- Formula: `Σ_t retention_fractions[t] * benefit_rates[t]` (zipped; extra
  elements in the longer slice are ignored)
- Total function: never returns `None`.
- Worked example: `expected_benefit_per_acquired_user(&[1.0, 0.25, 0.08, 0.04], &[4.0, 4.0, 4.0, 4.0]) == 5.48`

### `cost_per_retained_user(cac: f64, retention_fraction: f64) -> Option<f64>`

- Formula: `cac / retention_fraction`
- Returns `None` iff `retention_fraction == 0.0`
- Worked example: `cost_per_retained_user(5.0, 0.04) == Some(125.0)`

### `completers(cohort_size: f64, completion_fraction: f64) -> f64`

- Formula: `cohort_size * completion_fraction`
- Total function: never returns `None`.
- Worked example: `completers(100_000.0, 0.04) == 4_000.0`

### `qalys_delivered(completers: f64, qalys_per_completer: f64) -> f64`

- Formula: `completers * qalys_per_completer`
- Total function: never returns `None`.
- Worked example: `qalys_delivered(4_000.0, 0.02) == 80.0`

### `monetized_health_value(qalys: f64, value_per_qaly: f64) -> f64`

- Formula: `qalys * value_per_qaly`
- Total function: never returns `None`.
- Worked example: `monetized_health_value(80.0, 20_000.0) == 1_600_000.0`

### `health_value_per_download(total_value: f64, cohort_size: f64) -> Option<f64>`

- Formula: `total_value / cohort_size`
- Returns `None` iff `cohort_size == 0.0`
- Worked example: `health_value_per_download(1_600_000.0, 100_000.0) == Some(16.0)`

### `retention_improvement_value(cohort_size: f64, from_fraction: f64, to_fraction: f64, qalys_per_completer: f64, value_per_qaly: f64) -> f64`

- Formula: `cohort_size * (to_fraction - from_fraction) * qalys_per_completer * value_per_qaly`
- Total function: never returns `None` (negative if retention falls).
- Worked example: `retention_improvement_value(100_000.0, 0.04, 0.06, 0.02, 20_000.0) == 800_000.0`

## Invariants

- `qalys_delivered` and `monetized_health_value` must be applied to
  `completers`, not to the raw `cohort_size` — the module's entire point is
  that benefits accrue only to users still present, and its worked example
  contrasts the correct chained result (80 QALYs / £1.6M) against the naive
  claim of applying the trial effect to the whole cohort (2,000 QALYs / £40M).
- `retention_improvement_value(cohort, f0, f1, q, v)` is the closed form of
  `monetized_health_value(qalys_delivered(completers(cohort, f1), q), v) -
  monetized_health_value(qalys_delivered(completers(cohort, f0), q), v)`.
