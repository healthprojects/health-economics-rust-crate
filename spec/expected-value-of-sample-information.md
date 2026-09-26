# Spec: Expected Value of Sample Information (EVSI)

- **Module**: [`src/expected_value_of_sample_information.rs`](../src/expected_value_of_sample_information.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/expected-value-of-sample-information.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `evsi_normal_approximation(evpi: f64, sample_size: f64, prior_equivalent_sample_size: f64) -> Option<f64>`

- Formula: `evpi × sample_size / (sample_size + prior_equivalent_sample_size)`
  — the closed-form normal approximation, valid for a single uncertain
  parameter under a (roughly) conjugate normal-normal model.
- Returns `None` iff `sample_size + prior_equivalent_sample_size == 0.0`
  (the ratio is undefined).
- Worked example: `evsi_normal_approximation(1_200_000.0, 50.0, 75.0) == Some(480_000.0)`;
  `evsi_normal_approximation(1_200_000.0, 0.0, 0.0).is_none()`

### `expected_net_benefit_of_sampling(evsi: f64, study_cost: f64) -> f64`

- Formula: `evsi − study_cost`
- Total function: never returns `None`. Positive means the study is worth
  funding.
- Worked example: `expected_net_benefit_of_sampling(480_000.0, 120_000.0) == 360_000.0`

### `population_evsi(per_decision_evsi: f64, decisions_affected: f64) -> f64`

- Formula: `per_decision_evsi × decisions_affected`
- Total function: never returns `None`.
- Worked example: `population_evsi(480_000.0, 3.0) == 1_440_000.0`

## Invariants

- `EVSI ≤ EVPI` always, by construction: in `evsi_normal_approximation`, the
  factor `sample_size / (sample_size + prior_equivalent_sample_size)` is
  always in `[0, 1]` for non-negative inputs, so the returned value can never
  exceed the `evpi` argument. The module's rustdoc states that a calculation
  producing `EVSI > EVPI` is a modeling bug, not a real result.
