# Spec: Waiting List Impact

- **Module**: [`src/waiting_list_impact.rs`](../src/waiting_list_impact.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/waiting-list-impact.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `hours_released(staff_count: f64, hours_saved_per_day: f64, working_days_per_year: f64) -> f64`

- Formula: `staff_count * hours_saved_per_day * working_days_per_year`
- Total function: never returns `None`.
- Worked example: `hours_released(20.0, 0.75, 250.0) == 3_750.0`

### `extra_slots(hours_released: f64, slot_duration_hours: f64, utilization: f64) -> Option<f64>`

- Formula: `hours_released / slot_duration_hours * utilization`
- Returns `None` iff `slot_duration_hours == 0.0`
- Worked example: `extra_slots(3_750.0, 0.5, 0.85) == Some(6_375.0)`

### `patients_seen(extra_slots: f64, dna_rate: f64) -> f64`

- Formula: `extra_slots * (1 - dna_rate)`
- Total function: never returns `None`.
- Worked example: `patients_seen(6_375.0, 0.07) ≈ 5_929.0` (exact `5_928.75`, tolerance 0.5)

### `list_reduction(patients_seen: f64, induced_new_demand: f64) -> f64`

- Formula: `patients_seen - induced_new_demand`
- Total function: never returns `None` (can be negative if induced demand
  exceeds the capacity gain).
- Worked example: `list_reduction(5_929.0, 1_000.0) == 4_929.0`

### `waiting_time_gain(backlog_reduction: f64, service_rate: f64) -> Option<f64>`

- Formula: `backlog_reduction / service_rate`
- Returns `None` iff `service_rate == 0.0`
- Worked example: `waiting_time_gain(5_929.0, 24_000.0) ≈ Some(0.247)` (tolerance 0.001)

### `wait_reduction_fraction(extra_appointments: f64, annual_appointment_capacity: f64) -> Option<f64>`

- Formula: `extra_appointments / annual_appointment_capacity`
- Returns `None` iff `annual_appointment_capacity == 0.0`
- Worked example: `wait_reduction_fraction(5_928.75, 24_000.0) ≈ Some(0.25)` (exact ≈ 0.2470, tolerance 0.005)

### `activity_value(patients_seen: f64, scheme_value_per_attendance: f64) -> f64`

- Formula: `patients_seen * scheme_value_per_attendance`
- Total function: never returns `None`.
- Worked example: `activity_value(5_928.75, 160.0) ≈ 949_000.0` (exact `948_600.0`, tolerance 500.0)

## Invariants

- `hours_released` chains into `extra_slots`, which chains into
  `patients_seen`, which chains into both `wait_reduction_fraction` and
  `activity_value` — the module's worked example composes all five functions
  end-to-end from the same 20-nurse scenario.
- `list_reduction` and `activity_value` measure different things and must not
  be added: `list_reduction` is patient-count impact net of induced demand,
  while `activity_value` is the gross tariff value of patients seen (a
  non-cash-releasing capacity figure) — the module's rustdoc explicitly
  separates the waiting-list framing from any cash claim.
