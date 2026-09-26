# Spec: Quality-Adjusted Life Year (QALY)

- **Module**: [`src/quality_adjusted_life_year.rs`](../src/quality_adjusted_life_year.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/quality-adjusted-life-year.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `HealthState`

- `duration_years: f64` — years spent in this state (e.g. `0.5` for six
  months).
- `utility: f64` — quality weight of this state (1 = perfect health, 0 =
  dead, negative = worse than death).

### `qalys(states: &[HealthState]) -> f64`

- Formula: `Σ_i (duration_i × utility_i)`
- Total function: never returns `None`; returns `0.0` for an empty stream.
- Worked example: for
  `states = [(0.5, 0.6), (0.5, 0.85)]`, `qalys(&states) == 0.725`

### `qaly_loss_from_delay(delay_years: f64, utility_waiting: f64, utility_treated: f64) -> f64`

- Formula: `delay_years × (utility_treated − utility_waiting)`
- Total function: never returns `None`.
- Worked example: `qaly_loss_from_delay(0.5, 0.6, 0.85) == 0.125`

### `qaly_gain(qalys_with_intervention: f64, qalys_without_intervention: f64) -> f64`

- Formula: `qalys_with_intervention − qalys_without_intervention`
- Total function: never returns `None`.
- Worked example: `qaly_gain(0.85, 0.725) == 0.125` (approximately; doctest
  checks within `1e-9`)

### `population_qalys(qalys_per_patient: f64, patients: f64) -> f64`

- Formula: `qalys_per_patient × patients`
- Total function: never returns `None`.
- Worked example: `population_qalys(0.125, 400.0) == 50.0`

### `monetized_value(qalys: f64, threshold_per_qaly: f64) -> f64`

- Formula: `qalys × threshold_per_qaly`
- Total function: never returns `None`.
- Worked example: `monetized_value(50.0, 20_000.0) == 1_000_000.0`;
  `monetized_value(50.0, 30_000.0) == 1_500_000.0`

### `discount_factor(rate: f64, year: f64) -> f64`

- Formula: `1 / (1 + rate)^year`
- Total function: never returns `None`; year 0 is undiscounted (factor
  `1.0`).
- Worked example: `discount_factor(0.035, 0.0) == 1.0`;
  `discount_factor(0.035, 1.0) == 1.0 / 1.035`
