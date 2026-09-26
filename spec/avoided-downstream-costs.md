# Spec: Avoided Downstream Costs

- **Module**: [`src/avoided_downstream_costs.rs`](../src/avoided_downstream_costs.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/avoided-downstream-costs.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `attributable_events_avoided(population: f64, baseline_event_rate: f64, intervention_event_rate: f64) -> f64`

- Formula: `population * (baseline_event_rate - intervention_event_rate)`
- Total function: never returns `None`. Negative if the intervention arm has more events.
- Worked example: `attributable_events_avoided(5_000.0, 0.040, 0.031) == 45.0`

### `offset_value(events_avoided: f64, marginal_cost_per_event: f64) -> f64`

- Formula: `events_avoided * marginal_cost_per_event`
- Total function: never returns `None`.
- Worked example: `offset_value(45.0, 3_200.0) == 144_000.0`

### `intervention_cost(population: f64, cost_per_person: f64) -> f64`

- Formula: `population * cost_per_person`
- Total function: never returns `None`.
- Worked example: `intervention_cost(5_000.0, 20.0) == 100_000.0`

### `net_cost(intervention_cost: f64, offsets: &[f64]) -> f64`

- Formula: `intervention_cost - sum(offsets)`
- Total function: never returns `None`. Negative means genuinely cost-saving.
- Worked example: `net_cost(100_000.0, &[144_000.0]) == -44_000.0`

### `discount_factor(annual_discount_rate: f64, years: f64) -> f64`

- Formula: `1.0 / (1.0 + annual_discount_rate).powf(years)`
- Total function: never returns `None`.
- Worked example: `discount_factor(0.035, 4.0) == 1.0 / 1.035_f64.powi(4)` (≈0.871)

### `probability_weighted_offset(probability_of_future_event: f64, counterfactual_cost: f64, discount_factor: f64) -> f64`

- Formula: `probability_of_future_event * counterfactual_cost * discount_factor`
- Total function: never returns `None`.
- Worked example: for `df = discount_factor(0.035, 4.0)`, `probability_weighted_offset(0.6, 500_000.0, df) == 0.6 * 500_000.0 * df`

## Invariants

- `net_cost` is meant to be computed from `intervention_cost` and a slice of
  `offset_value` results, as in the worked example
  (£100,000 − [£144,000] = −£44,000).
- `probability_weighted_offset` composes with `discount_factor`: the
  discount factor for a future event is computed separately and passed in,
  rather than being derived internally.
- A valid offset (per the module's rustdoc) must be attributable (via
  `attributable_events_avoided`, using comparator evidence), marginal (via
  `offset_value` using marginal, not average, cost per event),
  probability-weighted and discounted (via `probability_weighted_offset` and
  `discount_factor`), and unique (counted once in `net_cost`'s offsets slice)
  — these functions provide the pieces but do not themselves enforce
  uniqueness or attribution.
- `discount_factor` is monotonically decreasing in both `annual_discount_rate`
  and `years` for positive rates: a higher rate or a longer horizon yields a
  smaller factor.
