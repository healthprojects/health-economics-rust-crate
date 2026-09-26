# Spec: Cost of Delay (CoD)

- **Module**: [`src/cost_of_delay.rs`](../src/cost_of_delay.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-of-delay.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `operational_cost_of_delay(saving_per_patient: f64, patients_per_week: f64) -> f64`

- Formula: `saving_per_patient * patients_per_week`
- Total function: never returns `None`.
- Worked example: `operational_cost_of_delay(200.0, 50.0) == 10_000.0`

### `total_delay_loss(cost_of_delay_per_week: f64, delay_weeks: f64) -> f64`

- Formula: `cost_of_delay_per_week * delay_weeks`
- Total function: never returns `None`.
- Worked example: `total_delay_loss(10_000.0, 10.0) == 100_000.0`

### `qaly_gain_per_patient(waiting_weeks_removed: f64, utility_gain: f64) -> f64`

- Formula: `(waiting_weeks_removed / 52.0) * utility_gain`
- Total function: never returns `None`.
- Worked example: `qaly_gain_per_patient(5.0, 0.12) ≈ 0.0115`

### `cost_of_delay_health(patients_per_week: f64, qaly_gain_per_patient: f64) -> f64`

- Formula: `patients_per_week * qaly_gain_per_patient`
- Total function: never returns `None`.
- Worked example: `cost_of_delay_health(100.0, 0.0115...) ≈ 1.15`

### `cost_of_delay_money(cost_of_delay_health_qalys_per_week: f64, willingness_to_pay_per_qaly: f64, operational_savings_per_week: f64) -> f64`

- Formula: `cost_of_delay_health_qalys_per_week * willingness_to_pay_per_qaly + operational_savings_per_week`
- Total function: never returns `None`.
- Worked example: `cost_of_delay_money(1.15, 20_000.0, 0.0) ≈ 23_000.0`

## Invariants

- `total_delay_loss` is denomination-agnostic: it works identically for
  money (£/week) and health (QALYs/week) `CoD` values, as long as both
  arguments use the same time unit (the module's convention is weeks).
- The health-to-money pipeline composes: `cost_of_delay_health` (fed by
  `qaly_gain_per_patient`) produces a QALYs/week figure that
  `cost_of_delay_money` then monetizes at λ, per the module's worked example.
