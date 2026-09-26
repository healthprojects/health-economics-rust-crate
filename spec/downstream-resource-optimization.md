# Spec: Downstream Resource Optimization

- **Module**: [`src/downstream_resource_optimization.rs`](../src/downstream_resource_optimization.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/downstream-resource-optimization.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `DownstreamRelease`

- `blocked_hours_released: f64` — blocked hours released for this role over the period (e.g. hrs/year).
- `unit_cost_per_hour: f64` — unit cost of an hour of this role's time (£/hour).

### `value_of_unblocking(downstream_releases: &[DownstreamRelease], pathway_throughput_gain: f64, value_per_pathway_completion: f64) -> f64`

- Formula: `Σ (r.blocked_hours_released * r.unit_cost_per_hour) for r in downstream_releases, then + pathway_throughput_gain * value_per_pathway_completion`
- Total function: never returns `None` (returns 0.0 when there are no releases and no throughput gain — the non-gating-role contrast case).
- Worked example: `value_of_unblocking(&[DownstreamRelease { blocked_hours_released: 1_095.0, unit_cost_per_hour: 30.0 }], 1_460.0, 300.0) == 32_850.0 + 438_000.0`; `value_of_unblocking(&[], 0.0, 300.0) == 0.0`

### `gating_task_minutes_saved(minutes_before: f64, minutes_after: f64) -> f64`

- Formula: `minutes_before - minutes_after`
- Total function: never returns `None`.
- Worked example: `gating_task_minutes_saved(90.0, 20.0) == 70.0`

### `annualize(per_day: f64, days_per_year: f64) -> f64`

- Formula: `per_day * days_per_year`
- Total function: never returns `None`.
- Worked example: `annualize(3.0, 365.0) == 1_095.0`

### `bed_days_avoided_per_year(late_discharges_recovered_per_day: f64, days_per_year: f64) -> f64`

- Formula: `annualize(late_discharges_recovered_per_day, days_per_year)`
- Total function: never returns `None`.
- Worked example: `bed_days_avoided_per_year(4.0, 365.0) == 1_460.0`

## Invariants

- `bed_days_avoided_per_year` is defined purely as a call to `annualize`, so
  the two functions always agree for the same arguments.
- The module's central point is a magnitude claim, not a strict formula
  invariant: per the worked example, the value from `value_of_unblocking`
  (summing downstream staff release plus pathway throughput) dwarfs the
  value of the gating role's own time saved
  (`gating_task_minutes_saved`), even valued at a generous per-hour rate for
  that role.
