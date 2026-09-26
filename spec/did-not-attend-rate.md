# Spec: Did-Not-Attend (DNA) Rate

- **Module**: [`src/did_not_attend_rate.rs`](../src/did_not_attend_rate.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/did-not-attend-rate.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `dna_rate_percent(dnas: f64, booked_appointments: f64) -> Option<f64>`

- Formula: `dnas / booked_appointments * 100.0`
- Returns `None` iff `booked_appointments == 0.0`
- Worked example: `dna_rate_percent(16_000.0, 200_000.0) == Some(8.0)`; `dna_rate_percent(0.0, 0.0) == None`

### `recovered_slots(appointments: f64, dna_rate_reduction_percentage_points: f64) -> f64`

- Formula: `appointments * dna_rate_reduction_percentage_points / 100.0`
- Total function: never returns `None`.
- Worked example: `recovered_slots(200_000.0, 2.5) == 5_000.0`

### `value_of_reduction(recovered_slots: f64, value_per_recovered_slot: f64) -> f64`

- Formula: `recovered_slots * value_per_recovered_slot`
- Total function: never returns `None`.
- Worked example: `value_of_reduction(5_000.0, 160.0) == 800_000.0`

### `service_cost(appointments: f64, cost_per_appointment: f64) -> f64`

- Formula: `appointments * cost_per_appointment`
- Total function: never returns `None`.
- Worked example: `service_cost(200_000.0, 0.40) == 80_000.0`

### `return_ratio(recovered_value: f64, service_cost: f64) -> Option<f64>`

- Formula: `recovered_value / service_cost`
- Returns `None` iff `service_cost == 0.0`
- Worked example: `return_ratio(800_000.0, 80_000.0) == Some(10.0)`; `return_ratio(800_000.0, 0.0) == None`

### `relative_reduction(dna_rate_before: f64, dna_rate_after: f64) -> Option<f64>`

- Formula: `(dna_rate_before - dna_rate_after) / dna_rate_before`
- Returns `None` iff `dna_rate_before == 0.0`
- Worked example: `relative_reduction(8.0, 5.5).unwrap() ≈ 0.3125`

## Invariants

- `value_of_reduction` values slots only under the assumption that recovered
  slots are genuinely refilled (e.g. from the waiting list) — the module's
  rustdoc warns that an unfilled recovered slot is still worth £0, a
  condition the function itself cannot detect from its arguments alone.
