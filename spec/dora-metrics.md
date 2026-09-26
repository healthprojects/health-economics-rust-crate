# Spec: DORA Metrics

- **Module**: [`src/dora_metrics.rs`](../src/dora_metrics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/dora-metrics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `deployment_frequency(deployments: f64, period: f64) -> Option<f64>`

- Formula: `deployments / period`
- Returns `None` iff `period == 0.0`
- Worked example: `deployment_frequency(12.0, 1.0) == Some(12.0)`; `deployment_frequency(1.0, 0.0) == None`

### `change_failure_rate_percent(failed_changes: f64, total_changes: f64) -> Option<f64>`

- Formula: `failed_changes / total_changes * 100.0`
- Returns `None` iff `total_changes == 0.0`
- Worked example: `change_failure_rate_percent(25.0, 100.0) == Some(25.0)`; `change_failure_rate_percent(1.0, 0.0) == None`

### `days_to_weeks(days: f64) -> f64`

- Formula: `days / 7.0`
- Total function: never returns `None`.
- Worked example: `days_to_weeks(4.0) ≈ 0.5714`

### `lead_time_reduction_weeks(before_weeks: f64, after_weeks: f64) -> f64`

- Formula: `before_weeks - after_weeks`
- Total function: never returns `None`.
- Worked example: `lead_time_reduction_weeks(6.0, days_to_weeks(4.0)) ≈ 5.4`

### `value_pulled_forward(improvements_per_year: f64, lead_time_reduction_weeks: f64, value_per_improvement_per_week: f64) -> f64`

- Formula: `improvements_per_year * lead_time_reduction_weeks * value_per_improvement_per_week`
- Total function: never returns `None`.
- Worked example: `value_pulled_forward(30.0, 5.4, 4_000.0) == 648_000.0`

### `failed_changes_avoided(changes_per_year: f64, cfr_before: f64, cfr_after: f64) -> f64`

- Formula: `changes_per_year * (cfr_before - cfr_after)`
- Total function: never returns `None`.
- Worked example: `failed_changes_avoided(30.0, 0.25, 0.08) ≈ 5.1`

### `failure_cost_avoided(changes_per_year: f64, cfr_before: f64, cfr_after: f64, cost_per_incident: f64) -> f64`

- Formula: `failed_changes_avoided(changes_per_year, cfr_before, cfr_after) * cost_per_incident`
- Total function: never returns `None`.
- Worked example: `failure_cost_avoided(30.0, 0.25, 0.08, 15_000.0) == 76_500.0`

### `downtime_harm(mttr_hours: f64, harm_per_hour: f64) -> f64`

- Formula: `mttr_hours * harm_per_hour`
- Total function: never returns `None`.
- Worked example: `downtime_harm(48.0, 1_000.0) / downtime_harm(2.0, 1_000.0) == 24.0`

### `reliability_adjusted_benefit(modeled_benefit: f64, slo_attainment: f64) -> f64`

- Formula: `modeled_benefit * slo_attainment`
- Total function: never returns `None`.
- Worked example: `reliability_adjusted_benefit(1.0, 0.99) ≈ 0.99`

## Invariants

- `failure_cost_avoided` composes with `failed_changes_avoided`: it is
  defined as `failed_changes_avoided(...) * cost_per_incident`, so the two
  functions always agree on the number of failed changes avoided.
- All rate arguments (`cfr_before`, `cfr_after`, `slo_attainment`) are
  fractions (0–1), not percentages, except in `change_failure_rate_percent`,
  which returns a percentage (0–100) — callers must not mix the two
  conventions across functions.
