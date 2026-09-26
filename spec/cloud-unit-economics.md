# Spec: Cloud Unit Economics (FinOps)

- **Module**: [`src/cloud_unit_economics.rs`](../src/cloud_unit_economics.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/cloud-unit-economics.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `CloudSpend`

A period's fully-allocated cloud spend, including the shared-platform
allocation that unit costs must not omit. All three lines are in the same
currency for the same period (typically a month).

- `compute: f64` — compute spend for the period.
- `data: f64` — data (storage/transfer) spend for the period.
- `shared_platform: f64` — allocated share of platform/security/on-call costs for the period.

### `CloudSpend::total(&self) -> f64`

- Formula: `self.compute + self.data + self.shared_platform`
- Total function: never returns `None`.
- Worked example: for `CloudSpend { compute: 30_000.0, data: 18_000.0, shared_platform: 14_000.0 }`, `spend.total() == 62_000.0`

### `unit_cost(total_allocated_cost: f64, units_delivered: f64) -> Option<f64>`

- Formula: `total_allocated_cost / units_delivered`
- Returns `None` iff `units_delivered == 0.0`
- Worked example: `unit_cost(62_000.0, 380_000.0) ≈ Some(0.163)`

### `unit_cost_ratio(unit_cost_a: f64, unit_cost_b: f64) -> Option<f64>`

- Formula: `unit_cost_a / unit_cost_b`
- Returns `None` iff `unit_cost_b == 0.0`
- Worked example: `unit_cost_ratio(62_000.0 / 380_000.0, 8.0) ≈ Some(0.02)`

### `unit_cost_change(previous: f64, current: f64) -> Option<f64>`

- Formula: `(current - previous) / previous`
- Returns `None` iff `previous == 0.0`
- Worked example: `unit_cost_change(0.21, 62_000.0 / 380_000.0) < Some(0.0)` (negative, i.e. improving)

## Invariants

- `unit_cost`'s numerator (`total_allocated_cost`) is meant to come from
  `CloudSpend::total()`, which always includes the `shared_platform` line —
  the module's rustdoc warns that omitting shared costs understates unit
  cost by 30–50%.
- `unit_cost_ratio` and `unit_cost_change` are both meant to be called with
  `unit_cost` outputs as their arguments (comparing one unit cost to
  another, or the same unit cost across two periods), rather than raw spend
  or volume figures.
- `unit_cost_change` returns a negative value when unit cost is falling
  (improving scale economics as fixed platform costs amortize over more
  units) and a positive value when it is rising.
