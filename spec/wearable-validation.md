# Spec: Wearable Validation

- **Module**: [`src/wearable_validation.rs`](../src/wearable_validation.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/wearable-validation.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `mape_percent(measured: &[f64], reference: &[f64]) -> Option<f64>`

- Formula: `(1/n) * Σ |measured_i - reference_i| / reference_i * 100`
- Returns `None` iff `measured` is empty, `measured.len() != reference.len()`,
  or any `reference` value is `0.0`
- Worked example: for `measured = [102.0, 97.0, 101.0]`, `reference = [100.0, 100.0, 100.0]`,
  `mape_percent(&measured, &reference) == Some(2.0)`

### `concordance_correlation(measured: &[f64], reference: &[f64]) -> Option<f64>`

- Formula: `2 * s_xy / (s_x² + s_y² + (x̄ - ȳ)²)` using population (1/n) moments
- Returns `None` iff `measured` is empty, `measured.len() != reference.len()`,
  or the denominator `s_x² + s_y² + (x̄ - ȳ)²` is `0.0` (both series constant
  with equal means)
- Worked example: for `reference = [60.0, 70.0, 80.0, 90.0, 100.0]`,
  `concordance_correlation(&reference, &reference) == Some(1.0)`; with a
  constant +10 bias, `concordance_correlation(&biased, &reference) == Some(0.8)`

### `BlandAltman`

Bland–Altman agreement summary.

- `mean_bias: f64` — mean of the paired differences (measured − reference);
  positive means the device reads high on average.
- `lower_limit: f64` — lower 95% limit of agreement: `bias - 1.96 * SD`.
- `upper_limit: f64` — upper 95% limit of agreement: `bias + 1.96 * SD`.

### `bland_altman(measured: &[f64], reference: &[f64]) -> Option<BlandAltman>`

- Formula: `mean_bias = mean(measured_i - reference_i)`;
  `lower_limit = mean_bias - 1.96 * sd`; `upper_limit = mean_bias + 1.96 * sd`,
  where `sd` is the sample standard deviation (n − 1 denominator) of the
  paired differences
- Returns `None` iff `measured.len() != reference.len()` or there are fewer
  than 2 pairs (`n < 2`, since the sample SD needs `n >= 2`)
- Worked example: for `measured = [70.0, 80.0, 90.0]`, `reference = [60.0, 70.0, 80.0]`,
  `bland_altman(&measured, &reference) == Some(BlandAltman { mean_bias: 10.0, lower_limit: 10.0, upper_limit: 10.0 })`

### `wear_time_compliance_percent(time_worn: f64, protocol_time: f64) -> Option<f64>`

- Formula: `time_worn / protocol_time * 100`
- Returns `None` iff `protocol_time == 0.0`
- Worked example: `wear_time_compliance_percent(16.0, 30.0) ≈ Some(53.33)` (tolerance 0.01)

### `data_completeness_percent(observed_points: f64, expected_points: f64) -> Option<f64>`

- Formula: `observed_points / expected_points * 100`
- Returns `None` iff `expected_points == 0.0`
- Worked example: `data_completeness_percent(900.0, 1_000.0) == Some(90.0)`

### `HeartRateMapeGrade`

Heart-rate MAPE grade against the field's accepted thresholds: `Strict`
(MAPE ≤ 5%), `Lenient` (5% < MAPE ≤ 10%), `Fail` (MAPE > 10%).

### `classify_heart_rate_mape(mape_percent: f64) -> HeartRateMapeGrade`

- Formula: `Strict` if `mape_percent <= 5.0`, `Lenient` if `<= 10.0`,
  otherwise `Fail`
- Total function: never returns `None`.
- Worked example: `classify_heart_rate_mape(2.1) == HeartRateMapeGrade::Strict`;
  `classify_heart_rate_mape(11.4) == HeartRateMapeGrade::Fail`

### `absolute_error_at(mape_percent: f64, true_value: f64) -> f64`

- Formula: `mape_percent / 100 * true_value`
- Total function: never returns `None`.
- Worked example: `absolute_error_at(11.4, 100.0) == 11.4`

### `annual_false_alert_cost(patient_count: f64, extra_false_alerts_per_patient_per_week: f64, cost_per_alert: f64) -> f64`

- Formula: `patient_count * extra_false_alerts_per_patient_per_week * cost_per_alert * 52.0`
- Total function: never returns `None`.
- Worked example: `annual_false_alert_cost(500.0, 2.0, 40.0) == 2_080_000.0`

## Invariants

- Validation must be reported per activity condition (rest, motion, sleep)
  and per population, not in aggregate — the module's worked example shows
  candidate A passing on a headline rest MAPE (2.1%, `Strict`) while failing
  at the alert-relevant motion condition (11.4%, `Fail`); `classify_heart_rate_mape`
  itself has no notion of "aggregate," so this discipline is a calling
  convention, not something the function enforces.
- `concordance_correlation` is bounded to `[-1, 1]` and is always `<=` the
  Pearson correlation of the same two series when there is a location or
  scale shift: a constant bias keeps Pearson `r = 1` but pulls CCC below 1,
  per the module's rustdoc.
