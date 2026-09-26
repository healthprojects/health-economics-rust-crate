# Spec: Patient-Reported Outcomes (PROMs, PREMs, MCID)

- **Module**: [`src/patient_reported_outcomes.rs`](../src/patient_reported_outcomes.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/patient-reported-outcomes.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `PHQ9_MCID: f64`

Constant, `5.0`: PHQ-9 (depression, score range 0–27) minimal clinically
important difference, in score points.

### `GAD7_MCID: f64`

Constant, `4.0`: GAD-7 (anxiety, score range 0–21) minimal clinically
important difference, in score points.

### `instrument_sum_score(item_responses: &[f64]) -> f64`

- Formula: `Σ item_responses` (e.g. PHQ-9 = sum of 9 items each 0–3)
- Total function: never returns `None`; returns `0.0` for an empty slice.
- Worked example: `instrument_sum_score(&[3.0, 3.0, 2.0, 2.0, 1.0, 1.0, 0.0, 0.0, 3.0]) == 15.0`

### `distribution_based_mcid(baseline_score_sd: f64) -> f64`

- Formula: `0.5 × baseline_score_sd`
- Total function: never returns `None`.
- Worked example: `distribution_based_mcid(8.0) == 4.0`

### `adjusted_difference(treatment_change: f64, control_change: f64) -> f64`

- Formula: `treatment_change − control_change`
- Total function: never returns `None`.
- Worked example: `adjusted_difference(-6.2, -2.1) == -4.1` (approximately;
  doctest checks within `1e-9`)

### `clears_mcid(mean_difference: f64, mcid: f64) -> bool`

- Formula: `|mean_difference| >= mcid`
- Returns a `bool`, not an `Option`; total function.
- Worked example: `clears_mcid(-4.1, PHQ9_MCID) == false`;
  `clears_mcid(-5.5, PHQ9_MCID) == true`

### `absolute_risk_reduction(responder_rate_treatment: f64, responder_rate_control: f64) -> f64`

- Formula: `responder_rate_treatment − responder_rate_control`
- Total function: never returns `None`.
- Worked example: `absolute_risk_reduction(0.48, 0.22) == 0.26`

### `number_needed_to_treat(absolute_risk_reduction: f64) -> Option<f64>`

- Formula: `1 / ARR`
- Returns `None` iff `absolute_risk_reduction == 0.0` (the arms do not
  differ, so NNT is undefined/infinite); a negative ARR yields a negative
  value (a number needed to harm).
- Worked example: `number_needed_to_treat(0.26)` is approximately `Some(4.0)`
  (module doctest asserts `(nnt - 4.0).abs() < 0.2`)

### `extra_responders(cohort_size: f64, absolute_risk_reduction: f64) -> f64`

- Formula: `cohort_size × ARR`
- Total function: never returns `None`.
- Worked example: `extra_responders(1_000.0, 0.26) == 260.0`

### `qalys_from_utility_gain(utility_gain: f64, duration_years: f64) -> f64`

- Formula: `utility_gain × duration_years`
- Total function: never returns `None`.
- Worked example: `qalys_from_utility_gain(0.06, 0.5) == 0.03`

### `cohort_qalys(extra_responders: f64, qalys_per_responder: f64) -> f64`

- Formula: `extra_responders × qalys_per_responder`
- Total function: never returns `None`.
- Worked example: `cohort_qalys(260.0, 0.03) == 7.8`

### `monetized_value(qalys: f64, threshold_per_qaly: f64) -> f64`

- Formula: `qalys × threshold_per_qaly`
- Total function: never returns `None`.
- Worked example: `monetized_value(7.8, 20_000.0) == 156_000.0`;
  `monetized_value(7.8, 30_000.0) == 234_000.0`
