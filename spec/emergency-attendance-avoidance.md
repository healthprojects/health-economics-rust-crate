# Spec: Emergency Attendance Avoidance

- **Module**: [`src/emergency_attendance_avoidance.rs`](../src/emergency_attendance_avoidance.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/emergency-attendance-avoidance.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `avoided_events(population: f64, baseline_rate: f64, intervention_rate: f64) -> f64`

- Formula: `population × (baseline_rate − intervention_rate)`
- Total function: never returns `None`. Negative when the intervention rate
  is higher than baseline.
- Worked example: `avoided_events(3_000.0, 0.9, 0.7) == 600.0` (and
  `avoided_events(3_000.0, 0.5, 0.42) == 240.0`)

### `gross_saving(avoided_events: f64, unit_cost: f64) -> f64`

- Formula: `avoided_events × unit_cost`
- Total function: never returns `None`.
- Worked example: `gross_saving(600.0, 300.0) == 180_000.0`

### `gross_saving_attendances_and_admissions(avoided_attendances: f64, attendance_unit_cost: f64, avoided_admissions: f64, admission_unit_cost: f64) -> f64`

- Formula: `avoided_attendances × attendance_unit_cost + avoided_admissions × admission_unit_cost`
- Total function: never returns `None`.
- Worked example: `gross_saving_attendances_and_admissions(600.0, 300.0, 240.0, 3_800.0) == 1_092_000.0`

### `net_saving(gross_saving: f64, intervention_cost: f64, new_pathway_cost: f64) -> f64`

- Formula: `gross_saving − intervention_cost − new_pathway_cost`
- Total function: never returns `None`. Negative means the intervention
  costs more than it saves in cash terms.
- Worked example: `net_saving(1_092_000.0, 600_000.0, 150_000.0) == 342_000.0`
