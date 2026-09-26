# Spec: Readmission Rate

- **Module**: [`src/readmission_rate.rs`](../src/readmission_rate.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/readmission-rate.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `readmission_rate_percent(emergency_readmissions_within_30_days: f64, index_discharges: f64) -> Option<f64>`

- Formula: `emergency_readmissions_within_30_days / index_discharges × 100`
- Returns `None` iff `index_discharges == 0.0`
- Worked example: `readmission_rate_percent(360.0, 2_000.0) == Some(18.0)`

### `observed_vs_expected(observed: f64, expected: f64) -> Option<f64>`

- Formula: `observed / expected`
- Returns `None` iff `expected == 0.0`
- Worked example: `observed_vs_expected(360.0, 360.0) == Some(1.0)`

### `avoided_readmissions(discharges: f64, baseline_rate: f64, new_rate: f64) -> f64`

- Formula: `discharges × (baseline_rate − new_rate)`
- Total function: never returns `None`; negative if the new rate is worse.
- Worked example: `avoided_readmissions(2_000.0, 0.18, 0.14) == 80.0`
  (approximately; doctest checks within `1e-9`)

### `value_of_avoidance(avoided_readmissions: f64, cost_per_readmission_spell: f64, penalty_exposure_per_readmission: f64) -> f64`

- Formula: `avoided_readmissions × (cost_per_readmission_spell + penalty_exposure_per_readmission)`
- Total function: never returns `None`.
- Worked example: `value_of_avoidance(80.0, 3_500.0, 0.0) == 280_000.0`;
  `value_of_avoidance(80.0, 3_500.0, 500.0) == 320_000.0`

### `program_cost(discharges: f64, cost_per_discharge: f64) -> f64`

- Formula: `discharges × cost_per_discharge`
- Total function: never returns `None`.
- Worked example: `program_cost(2_000.0, 60.0) == 120_000.0`

### `net_benefit(value_of_avoidance: f64, program_cost: f64) -> f64`

- Formula: `value_of_avoidance − program_cost`
- Total function: never returns `None`.
- Worked example: `net_benefit(280_000.0, 120_000.0) == 160_000.0`
