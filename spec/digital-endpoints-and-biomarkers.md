# Spec: Digital Endpoints and Biomarkers

- **Module**: [`src/digital_endpoints_and_biomarkers.rs`](../src/digital_endpoints_and_biomarkers.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/digital-endpoints-and-biomarkers.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `required_sample_size(variance: f64, detectable_effect: f64, k: f64) -> Option<f64>`

- Formula: `k * variance / (detectable_effect * detectable_effect)`
- Returns `None` iff `detectable_effect == 0.0` (a null hypothesis of no effect needs infinite N — undefined).
- Worked example: `required_sample_size(5.0, 1.0, 16.0) == Some(80.0)`; `required_sample_size(1.0, 0.0, 16.0) == None`

### `sampling_density_ratio(digital_measurements_per_year: f64, clinic_measurements_per_year: f64) -> Option<f64>`

- Formula: `digital_measurements_per_year / clinic_measurements_per_year`
- Returns `None` iff `clinic_measurements_per_year == 0.0`
- Worked example: `sampling_density_ratio(200.0, 4.0) == Some(50.0)`

### `detectable_effect_improvement(variance_reduction_fold: f64) -> f64`

- Formula: `variance_reduction_fold.sqrt()`
- Total function: never returns `None`.
- Worked example: `detectable_effect_improvement(5.0) ≈ 2.236`

### `sample_size_ratio(variance_new: f64, variance_old: f64) -> Option<f64>`

- Formula: `variance_new / variance_old`
- Returns `None` iff `variance_old == 0.0`
- Worked example: `sample_size_ratio(1.0, 5.0) == Some(0.2)`

### `trial_cost_saving(patients_cut: f64, cost_per_enrolled_patient: f64) -> f64`

- Formula: `patients_cut * cost_per_enrolled_patient`
- Total function: never returns `None`.
- Worked example: `trial_cost_saving(200.0, 25_000.0) == 5_000_000.0`

## Invariants

- `required_sample_size` and `sample_size_ratio` are two views of the same
  power relation N ∝ σ²/Δ²: at a fixed `detectable_effect`, the ratio of two
  `required_sample_size` calls equals `sample_size_ratio` of their
  variances — the module's worked example verifies
  `required_sample_size(1.0, 1.0, 16.0) / required_sample_size(5.0, 1.0, 16.0) == sample_size_ratio(1.0, 5.0)`.
- `detectable_effect_improvement` and `sample_size_ratio` are inverse-square
  views of the same variance change: `detectable_effect_improvement(fold) ==
  1.0 / sample_size_ratio(1.0, fold).unwrap().sqrt()` for `fold` equal to the
  variance reduction factor.
