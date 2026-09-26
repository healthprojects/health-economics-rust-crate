# Spec: Budget Impact Analysis (BIA)

- **Module**: [`src/budget_impact_analysis.rs`](../src/budget_impact_analysis.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/budget-impact-analysis.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `net_cost_per_patient(intervention_cost: f64, displaced_care_cost: f64, induced_care_cost: f64) -> f64`

- Formula: `intervention_cost - displaced_care_cost + induced_care_cost`
- Total function: never returns `None`. May be negative if displacement exceeds the intervention's price.
- Worked example: `net_cost_per_patient(300.0, 120.0, 0.0) == 180.0`; `net_cost_per_patient(300.0, 120.0, 40.0) == 220.0`

### `PatientGroup`

One patient group in a budget-impact scenario for a single year; a scenario
for year t is the sum over such groups.

- `eligible_population: f64` — number of payer members eligible for the intervention this year.
- `uptake: f64` — fraction of the eligible population actually using it this year (0–1).
- `net_cost_per_patient: f64` — net cost per patient this year (intervention − displaced + induced).

### `PatientGroup::cost(&self) -> f64`

- Formula: `self.eligible_population * self.uptake * self.net_cost_per_patient`
- Total function: never returns `None`. Undiscounted, per BIA convention.
- Worked example: for `PatientGroup { eligible_population: 30_000.0, uptake: 0.20, net_cost_per_patient: 180.0 }`, `group.cost() == 1_080_000.0`

### `scenario_cost(groups: &[PatientGroup]) -> f64`

- Formula: `sum over groups of group.cost()`
- Total function: never returns `None`; returns `0.0` for an empty slice.
- Worked example: `scenario_cost(&[PatientGroup { eligible_population: 30_000.0, uptake: 0.60, net_cost_per_patient: 180.0 }]) == 3_240_000.0`

### `budget_impact(cost_scenario_with_new: f64, cost_scenario_current: f64) -> f64`

- Formula: `cost_scenario_with_new - cost_scenario_current`
- Total function: never returns `None`. Undiscounted by design. Positive means new money the payer must find.
- Worked example: `budget_impact(1_080_000.0, 0.0) == 1_080_000.0`

## Invariants

- `budget_impact` is meant to be called with `cost_scenario_with_new` computed
  via `scenario_cost` over the year's `PatientGroup`s, and
  `cost_scenario_current` computed the same way for the comparator scenario.
- `PatientGroup::cost` and `scenario_cost` are undiscounted by design — BIA
  reports annual cash flows as the payer will experience them, unlike
  cost-effectiveness analysis's discounted horizon.
- The worked example's uptake ramp (0.20 → 0.40 → 0.60 over three years, same
  `eligible_population` and `net_cost_per_patient`) makes `PatientGroup::cost`
  scale linearly with `uptake`: £1.08M → £2.16M → £3.24M.
