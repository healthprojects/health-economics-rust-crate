# Spec: Total Cost of Ownership (TCO)

- **Module**: [`src/total_cost_of_ownership.rs`](../src/total_cost_of_ownership.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/total-cost-of-ownership.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `TcoProfile`

Cost profile of one option over a fixed horizon. Year-0 costs (`initial_cost`
+ `integration_and_training`) are undiscounted; running costs accrue in years
1..=horizon; decommissioning falls at the end of the horizon.

- `initial_cost: f64` — year-0 build or licence cost.
- `integration_and_training: f64` — integration, data migration, and training
  (year 0).
- `annual_run_cost: f64` — annual running cost (operations, maintenance,
  support, infrastructure).
- `horizon_years: u32` — horizon in years over which annual running costs
  accrue.
- `decommission_cost: f64` — exit/decommissioning cost, incurred at the end
  of the horizon.

### `TcoProfile::undiscounted_tco(&self) -> f64`

- Formula: `initial_cost + integration_and_training + annual_run_cost * horizon_years + decommission_cost`
- Total function: never returns `None`.
- Worked example: for `TcoProfile { initial_cost: 250_000.0, integration_and_training: 180_000.0, annual_run_cost: 120_000.0, horizon_years: 5, decommission_cost: 60_000.0 }`,
  `undiscounted_tco() == 1_090_000.0`

### `TcoProfile::discounted_tco(&self, discount_rate: f64) -> f64`

- Formula: `initial_cost + integration_and_training + Σ_{t=1}^{horizon_years} annual_run_cost / (1 + discount_rate)^t + decommission_cost / (1 + discount_rate)^horizon_years`
- Total function: never returns `None`. At `discount_rate = 0.0` this equals
  `undiscounted_tco()`.
- Worked example: `saas.discounted_tco(0.0) == saas.undiscounted_tco()`, and
  `saas.discounted_tco(0.035) < saas.undiscounted_tco()`

### `TcoProfile::initial_cost_share(&self) -> Option<f64>`

- Formula: `initial_cost / undiscounted_tco()`
- Returns `None` iff `undiscounted_tco() == 0.0`
- Worked example: for the in-house build profile (`initial_cost: 900_000.0`,
  `undiscounted_tco() == 2_030_000.0`),
  `initial_cost_share() ≈ Some(0.44)` (tolerance 0.005)

### `tco_advantage(a: &TcoProfile, b: &TcoProfile) -> f64`

- Formula: `b.undiscounted_tco() - a.undiscounted_tco()` (positive means `a`
  is cheaper)
- Total function: never returns `None`.
- Worked example: `tco_advantage(&saas, &build) == 940_000.0` where `saas.undiscounted_tco() == 1_090_000.0` and `build.undiscounted_tco() == 2_030_000.0`

### `annual_maintenance_benchmark(build_cost: f64, maintenance_fraction: f64) -> f64`

- Formula: `build_cost * maintenance_fraction`
- Total function: never returns `None`.
- Worked example: `annual_maintenance_benchmark(1_000_000.0, 0.15) == 150_000.0`

## Invariants

- `tco_advantage` is a **cost-minimization** comparison: valid only when the
  two options deliver materially equivalent outcomes over the same
  `horizon_years` — comparing profiles over different horizons is itself
  flagged as a pitfall.
- Discounting preserves ranking in the module's worked example: both
  `saas.discounted_tco(0.035) < saas.undiscounted_tco()` and
  `build.discounted_tco(0.035) < build.undiscounted_tco()` hold, and the SaaS
  option remains cheaper than the build option under discounting.
