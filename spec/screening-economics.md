# Spec: Screening Economics

- **Module**: [`src/screening_economics.rs`](../src/screening_economics.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/screening-economics.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `positive_predictive_value(sensitivity: f64, specificity: f64, prevalence: f64) -> Option<f64>`

- Formula: `(sensitivity * prevalence) / (sensitivity * prevalence + (1 - specificity) * (1 - prevalence))`
- Returns `None` iff the denominator (true-positive rate + false-positive
  rate, i.e. the overall positive rate) is `0.0`
- Worked example: `positive_predictive_value(0.90, 0.95, 0.005) ≈ Some(0.083)` (tolerance 0.001)

### `true_positives(population: f64, prevalence: f64, sensitivity: f64) -> f64`

- Formula: `population * prevalence * sensitivity`
- Total function: never returns `None`.
- Worked example: `true_positives(100_000.0, 0.005, 0.90) == 450.0`

### `false_positives(population: f64, prevalence: f64, specificity: f64) -> f64`

- Formula: `population * (1 - prevalence) * (1 - specificity)`
- Total function: never returns `None`.
- Worked example: `false_positives(100_000.0, 0.005, 0.95) == 4_975.0`

### `total_programme_cost(population: f64, screening_cost_per_person: f64, workup_cost_per_positive: f64, true_positives: f64, false_positives: f64) -> f64`

- Formula: `population * screening_cost_per_person + (true_positives + false_positives) * workup_cost_per_positive`
- Total function: never returns `None`.
- Worked example: `total_programme_cost(100_000.0, 15.0, 400.0, 450.0, 4_975.0) == 3_670_000.0`

### `cost_per_true_case(total_programme_cost: f64, true_positives: f64) -> Option<f64>`

- Formula: `total_programme_cost / true_positives`
- Returns `None` iff `true_positives == 0.0`
- Worked example: `cost_per_true_case(3_670_000.0, 450.0) ≈ Some(8_156.0)` (tolerance 1.0)

### `net_value_per_case_found(earlier_intervention_value_per_case: f64, overdiagnosis_harm_per_case: f64) -> f64`

- Formula: `earlier_intervention_value_per_case - overdiagnosis_harm_per_case`
- Total function: never returns `None` (can be negative if overdiagnosis harm
  dominates).
- Worked example: `net_value_per_case_found(20_000.0, 0.0) == 20_000.0`, which
  the module's doctest asserts is `> 8_156.0` (the worked example's cost per
  true case)

## Invariants

- `total_programme_cost` and `cost_per_true_case` compose end-to-end with
  `true_positives`/`false_positives`: the module's worked example chains
  `true_positives(...)` and `false_positives(...)` straight into
  `total_programme_cost(...)` and then `cost_per_true_case(...)`.
- A screening programme is worthwhile only when `net_value_per_case_found`
  clears `cost_per_true_case` — the module's rustdoc states this as the
  benchmark relationship between the two functions, without defining a
  combined function for it.
