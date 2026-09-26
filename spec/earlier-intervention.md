# Spec: Earlier Intervention

- **Module**: [`src/earlier_intervention.rs`](../src/earlier_intervention.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/earlier-intervention.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `progression_events_avoided(patients: f64, annual_progression_rate: f64, acceleration_years: f64) -> f64`

- Formula: `patients * annual_progression_rate * acceleration_years` (a constant-hazard model)
- Total function: never returns `None` (returns a fractional expectation, not an integer count).
- Worked example: `progression_events_avoided(4_000.0, 0.02, 4.0 / 12.0) ≈ 26.666666`

### `value_per_avoided_progression(cost_late: f64, cost_early: f64, qalys_early: f64, qalys_late: f64, lambda: f64) -> f64`

- Formula: `(cost_late - cost_early) + (qalys_early - qalys_late) * lambda`
- Total function: never returns `None` (may be negative if late treatment were somehow cheaper and better).
- Worked example: `value_per_avoided_progression(4_000.0, 0.0, 0.8, 0.0, 20_000.0) == 20_000.0`

### `value_per_patient(cost_late: f64, cost_early: f64, qalys_early: f64, qalys_late: f64, lambda: f64, probability_of_progression: f64) -> f64`

- Formula: `value_per_avoided_progression(cost_late, cost_early, qalys_early, qalys_late, lambda) * probability_of_progression`
- Total function: never returns `None`.
- Worked example: for `p = 0.02 * (4.0 / 12.0)`, `value_per_patient(4_000.0, 0.0, 0.8, 0.0, 20_000.0, p) * 4_000.0 ≈ 533_333.33`

### `total_backlog_value(events_avoided: f64, value_per_event: f64) -> f64`

- Formula: `events_avoided * value_per_event`
- Total function: never returns `None`.
- Worked example: `total_backlog_value(27.0, 20_000.0) == 540_000.0`

## Invariants

- `value_per_patient` is `value_per_avoided_progression` scaled by
  `probability_of_progression` — the module's rustdoc frames this
  probability weighting as "the difference between analysis and advocacy":
  the full per-progression value must never be applied to every waiting
  patient, only to the expected fraction who would actually progress.
- `total_backlog_value(events_avoided, value_per_event)` is equivalent to
  summing `value_per_patient(...)` over every patient in the backlog, when
  `events_avoided` is computed via `progression_events_avoided` for the same
  cohort and `value_per_event` via `value_per_avoided_progression` — the
  module's own test cross-checks the two computation paths against each
  other.
