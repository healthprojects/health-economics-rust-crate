# Spec: Referral to Treatment (RTT)

- **Module**: [`src/referral_to_treatment.rs`](../src/referral_to_treatment.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/referral-to-treatment.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `RTT_STANDARD_PERCENT: f64`

Constant, `92.0`: the NHS Constitution RTT standard — 92% of patients should
start treatment within 18 weeks.

### `rtt_performance_percent(treated_within_18_weeks: f64, total_treated: f64) -> Option<f64>`

- Formula: `treated_within_18_weeks / total_treated × 100`
- Returns `None` iff `total_treated == 0.0`
- Worked example: `rtt_performance_percent(4_600.0, 5_000.0) == Some(92.0)`

### `meets_rtt_standard(performance_percent: f64) -> bool`

- Formula: `performance_percent >= RTT_STANDARD_PERCENT` (exactly 92% meets
  the standard)
- Returns a `bool`, not an `Option`; total function.
- Worked example: `meets_rtt_standard(92.0) == true`;
  `meets_rtt_standard(91.9) == false`

### `waiting_health_cost_qalys(wait_duration_years: f64, utility_treated: f64, utility_waiting: f64) -> f64`

- Formula: `wait_duration_years × (utility_treated − utility_waiting)`
- Total function: never returns `None`.
- Worked example: `waiting_health_cost_qalys(5.0 / 52.0, 0.80, 0.68)` equals
  `5.0 / 52.0 * 0.12` (approximately; doctest checks within `1e-9`)

### `qaly_gain_from_wait_reduction(patients_per_year: f64, weeks_removed: f64, utility_treated: f64, utility_waiting: f64) -> f64`

- Formula: `patients_per_year × (weeks_removed / 52.0) × (utility_treated − utility_waiting)`
- Total function: never returns `None`.
- Worked example: `qaly_gain_from_wait_reduction(5_000.0, 5.0, 0.80, 0.68)` is
  approximately `57.7` (module doctest asserts `(q - 57.7).abs() < 0.05`)

### `monetized_value(qalys: f64, threshold_per_qaly: f64) -> f64`

- Formula: `qalys × threshold_per_qaly`
- Total function: never returns `None`.
- Worked example: `monetized_value(57.7, 20_000.0)` is approximately
  `1_154_000.0`; `monetized_value(57.7, 30_000.0)` is approximately
  `1_731_000.0` (both exact given the literal `57.7` input, per the module
  doctest)

### `total_pathway_duration(stage_durations: &[f64]) -> f64`

- Formula: `Σ stage_durations`
- Total function: never returns `None`; returns `0.0` for an empty pathway.
- Worked example: `total_pathway_duration(&[1.0, 6.0, 9.0, 2.0, 6.0]) == 24.0`

### `longest_stage(stage_durations: &[f64]) -> Option<f64>`

- Formula: the maximum of `stage_durations`
- Returns `None` iff `stage_durations` is empty.
- Worked example: `longest_stage(&[1.0, 6.0, 9.0, 2.0, 6.0]) == Some(9.0)`

## Invariants

- `waiting_health_cost_qalys(wait_duration_years, utility_treated, utility_waiting) × patients`
  equals `qaly_gain_from_wait_reduction(patients, wait_duration_years × 52.0, utility_treated, utility_waiting)`
  — the per-patient and cohort formulas agree, since
  `qaly_gain_from_wait_reduction` is `patients × (weeks_removed / 52.0) × (Δutility)`,
  the same per-patient factor scaled by population.
- Improving pathway performance means shortening `longest_stage`, not just
  any stage — the module's rustdoc states this explicitly ("improve the
  longest queue, not the busiest stage").
