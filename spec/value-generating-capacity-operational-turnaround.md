# Spec: Value-Generating Capacity (Operational Turnaround)

- **Module**: [`src/value_generating_capacity_operational_turnaround.rs`](../src/value_generating_capacity_operational_turnaround.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/value-generating-capacity-operational-turnaround.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `extra_activity_units(staff_count: f64, units_per_staff_per_day: f64, working_days_per_year: f64) -> f64`

- Formula: `staff_count * units_per_staff_per_day * working_days_per_year`
- Total function: never returns `None`.
- Worked example: `extra_activity_units(25.0, 2.0, 250.0) == 12_500.0`

### `capacity_value(extra_activity_units: f64, scheme_value_per_unit: f64) -> f64`

- Formula: `extra_activity_units * scheme_value_per_unit`
- Total function: never returns `None`.
- Worked example: `capacity_value(12_500.0, 120.0) == 1_500_000.0`

### `annual_capacity_value(staff_count: f64, units_per_staff_per_day: f64, working_days_per_year: f64, scheme_value_per_unit: f64) -> f64`

- Formula: `capacity_value(extra_activity_units(staff_count, units_per_staff_per_day, working_days_per_year), scheme_value_per_unit)`
- Total function: never returns `None`.
- Worked example: `annual_capacity_value(25.0, 2.0, 250.0, 120.0) == 1_500_000.0`

## Invariants

- `annual_capacity_value` is the composed form of
  `capacity_value(extra_activity_units(staff_count, units_per_staff_per_day, working_days_per_year), scheme_value_per_unit)`
  — the module's doctest asserts both routes give the same £1.5M figure.
- The result is a **non-cash-releasing capacity value**, not cash income:
  under blended payment, extra activity may not bring extra revenue — the
  module's rustdoc requires this to be labeled, not presented as bankable
  cash.
