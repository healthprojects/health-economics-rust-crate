# Spec: Practitioner Time

- **Module**: [`src/practitioner_time.rs`](../src/practitioner_time.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/practitioner-time.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `daily_minutes_saved(minutes_saved_per_consultation: f64, consultations_per_day: f64) -> f64`

- Formula: `minutes_saved_per_consultation × consultations_per_day`
- Total function: never returns `None`.
- Worked example: `daily_minutes_saved(2.0, 30.0) == 60.0`

### `annual_hours_saved(daily_minutes_saved: f64, working_days_per_year: f64) -> f64`

- Formula: `daily_minutes_saved × working_days_per_year / 60.0`
- Total function: never returns `None`.
- Worked example: `annual_hours_saved(60.0, 220.0) == 220.0`

### `wage_basis_value(hours_saved: f64, loaded_hourly_rate: f64) -> f64`

- Formula: `hours_saved × loaded_hourly_rate` (Level 1 — wage basis)
- Total function: never returns `None`.
- Worked example: `wage_basis_value(220.0, 80.0) == 17_600.0`

### `extra_appointments_per_day(daily_minutes_saved: f64, minutes_per_appointment: f64) -> Option<f64>`

- Formula: `daily_minutes_saved / minutes_per_appointment`
- Returns `None` iff `minutes_per_appointment == 0.0`
- Worked example: `extra_appointments_per_day(60.0, 12.0) == Some(5.0)`

### `annual_extra_appointments(extra_appointments_per_day: f64, working_days_per_year: f64) -> f64`

- Formula: `extra_appointments_per_day × working_days_per_year`
- Total function: never returns `None`.
- Worked example: `annual_extra_appointments(5.0, 220.0) == 1_100.0`

### `output_basis_value(annual_extra_appointments: f64, value_per_appointment: f64) -> f64`

- Formula: `annual_extra_appointments × value_per_appointment` (Level 2 —
  output basis)
- Total function: never returns `None`.
- Worked example: `output_basis_value(1_100.0, 42.0) == 46_200.0`

### `bottleneck_basis_value(hours_saved: f64, pathway_value_per_hour: f64) -> f64`

- Formula: `hours_saved × pathway_value_per_hour` (Level 3 — bottleneck basis)
- Total function: never returns `None`.
- Worked example: `bottleneck_basis_value(220.0, 500.0) == 110_000.0`

### `fragmentation_adjusted_value(raw_value: f64, utilization_factor: f64) -> f64`

- Formula: `raw_value × utilization_factor`
- Total function: never returns `None`.
- Worked example: `fragmentation_adjusted_value(46_200.0, 0.5) == 23_100.0`

## Invariants

- The three valuation bases (wage, output, bottleneck) are increasing in
  honesty per the module's rustdoc, but are not ordered by magnitude in
  general — they price the same released time by three different economic
  mechanisms (cost, output, and pathway throughput) and can rank in any
  order depending on the inputs supplied.
