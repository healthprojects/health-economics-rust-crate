# Spec: Prevention Economics

- **Module**: [`src/prevention_economics.rs`](../src/prevention_economics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/prevention-economics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `program_cost(population: f64, annual_cost_per_person: f64, discounted_years_factor: f64) -> f64`

- Formula: `population × annual_cost_per_person × discounted_years_factor`
- Total function: never returns `None`.
- Worked example: `program_cost(100_000.0, 25.0, 8.3) == 20_750_000.0`

### `downstream_offsets(cases_prevented: f64, avoided_cost_per_case: f64) -> f64`

- Formula: `cases_prevented × avoided_cost_per_case`
- Total function: never returns `None`.
- Worked example: `downstream_offsets(400.0, 45_000.0) == 18_000_000.0`

### `net_cost(program_cost: f64, downstream_offsets: f64) -> f64`

- Formula: `program_cost − downstream_offsets`
- Total function: never returns `None`.
- Worked example: `net_cost(20_750_000.0, 18_000_000.0) == 2_750_000.0`

### `is_cost_saving(net_cost: f64) -> bool`

- Formula: `net_cost < 0.0` (exactly zero counts as *not* cost-saving)
- Returns a `bool`, not an `Option`; total function.
- Worked example: `is_cost_saving(2_750_000.0) == false`;
  `is_cost_saving(-500_000.0) == true`

### `per_person_cost_saving_condition(intervention_cost_per_person: f64, probability_of_progression: f64, avoided_cost_per_case: f64, discount_factor: f64) -> bool`

- Formula: `intervention_cost_per_person < probability_of_progression × avoided_cost_per_case × discount_factor`
- Returns a `bool`, not an `Option`; total function.
- Worked example: `per_person_cost_saving_condition(25.0 * 8.3, 0.004, 45_000.0, 1.0) == false`

### `qalys_gained(cases_prevented: f64, qalys_lost_per_case: f64) -> f64`

- Formula: `cases_prevented × qalys_lost_per_case`
- Total function: never returns `None`.
- Worked example: `qalys_gained(400.0, 3.0) == 1_200.0`

### `cost_per_qaly(net_cost: f64, qalys_gained: f64) -> Option<f64>`

- Formula: `net_cost / qalys_gained`
- Returns `None` iff `qalys_gained == 0.0`
- Worked example: `cost_per_qaly(2_750_000.0, 1_200.0)` is approximately
  `Some(2_300.0)` (module doctest asserts `(cpq - 2_300.0).abs() < 50.0`)

### `is_cost_effective(cost_per_qaly: f64, threshold_per_qaly: f64) -> bool`

- Formula: `cost_per_qaly < threshold_per_qaly`
- Returns a `bool`, not an `Option`; total function.
- Worked example: `is_cost_effective(2_300.0, 20_000.0) == true`;
  `is_cost_effective(35_000.0, 30_000.0) == false`

## Invariants

- Fewer than 20% of preventive interventions are net cost-saving (`net_cost
  < 0.0`); most are cost-effective (`cost_per_qaly < threshold`) without
  being cost-saving — the module's rustdoc states this is the defining
  distinction of the "prevention paradox," and the worked example
  demonstrates a program that is cost-effective (`is_cost_effective` true)
  while not cost-saving (`is_cost_saving` false) on the same inputs.
