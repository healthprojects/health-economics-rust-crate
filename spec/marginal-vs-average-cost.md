# Spec: Marginal vs Average Cost

- **Module**: [`src/marginal_vs_average_cost.rs`](../src/marginal_vs_average_cost.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/marginal-vs-average-cost.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `average_cost(total_cost: f64, quantity: f64) -> Option<f64>`

- Formula: `AC = TC / Q`
- Returns `None` iff `quantity == 0.0`
- Worked example: `average_cost(400_000.0, 1_000.0) == Some(400.0)`

### `naive_average_cost_saving(units_freed: f64, average_cost_per_unit: f64) -> f64`

- Formula: `units_freed × average_cost_per_unit`
- Total function: never returns `None`.
- Worked example: `naive_average_cost_saving(1_000.0, 400.0) == 400_000.0`

### `marginal_saving(units_freed: f64, marginal_cost_per_unit: f64) -> f64`

- Formula: `ΔQ × MC` (units freed × marginal/variable cost per unit)
- Total function: never returns `None`.
- Worked example: `marginal_saving(1_000.0, 120.0) == 120_000.0`

### `step_change_saving(total_cost_before: f64, total_cost_after: f64) -> f64`

- Formula: `total_cost_before − total_cost_after`
- Total function: never returns `None`.
- Worked example: `step_change_saving(10_000_000.0, 8_500_000.0) == 1_500_000.0`

### `ward_bed_days_per_year(beds: f64) -> f64`

- Formula: `beds × 365`
- Total function: never returns `None`.
- Worked example: `ward_bed_days_per_year(20.0) == 7_300.0`

### `crosses_capacity_step(units_freed: f64, units_per_capacity_step: f64) -> bool`

- Formula: `units_freed >= units_per_capacity_step`
- Returns a `bool`, not an `Option`; total function.
- Worked example: with `step = ward_bed_days_per_year(20.0)` (`7_300.0`),
  `crosses_capacity_step(7_300.0, step) == true` and
  `crosses_capacity_step(1_000.0, step) == false`

## Invariants

- With fixed costs present, `marginal_saving` < `naive_average_cost_saving`
  for the same units freed and a realistic marginal cost below the average
  cost (the module's rustdoc: "Fixed costs make MC < AC for capacity
  reductions"), e.g. £120/unit marginal vs £400/unit average in the worked
  example.
- `step_change_saving` is the only one of the three saving functions that is
  defensible once `crosses_capacity_step` returns `true`; below the
  capacity step, only `marginal_saving` is defensible (per the module's
  rustdoc on `crosses_capacity_step`).
