# Spec: Expected Value of Perfect Information (EVPI)

- **Module**: [`src/expected_value_of_perfect_information.rs`](../src/expected_value_of_perfect_information.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/expected-value-of-perfect-information.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Scenario`

One possible world: its probability and the net monetary benefit of each
decision option in that world.

- `probability: f64` — probability of this world; scenario probabilities
  should sum to 1.
- `option_nmbs: Vec<f64>` — net monetary benefit of each option, given this
  world (one entry per option, same order in every scenario).

### `expected_nmb_of_best_option(scenarios: &[Scenario]) -> Option<f64>`

- Formula: `max_j Σ_i (scenarios[i].probability × scenarios[i].option_nmbs[j])`
  — expectation per option across worlds, then the max over options
  ("decide now").
- Returns `None` iff `scenarios` is empty, the first scenario has no
  options, or any scenario's option list has a different length than the
  first (every scenario must offer the same options).
- Worked example:
  `expected_nmb_of_best_option(&[Scenario { probability: 0.6, option_nmbs: vec![8.0, 0.0] }, Scenario { probability: 0.4, option_nmbs: vec![-3.0, 0.0] }]) == Some(3.6)`

### `expected_nmb_with_perfect_information(scenarios: &[Scenario]) -> Option<f64>`

- Formula: `Σ_i (scenarios[i].probability × max_j scenarios[i].option_nmbs[j])`
  — max over options inside each world ("perfect foresight"), then the
  probability-weighted sum across worlds.
- Returns `None` iff `scenarios` is empty or any scenario has an empty
  option list.
- Worked example: same scenarios as above → `Some(4.8)`

### `evpi(scenarios: &[Scenario]) -> Option<f64>`

- Formula: `expected_nmb_with_perfect_information(scenarios) − expected_nmb_of_best_option(scenarios)`
- Returns `None` iff either of the two calls above returns `None` (i.e.
  `scenarios` is empty or options are missing). Always ≥ 0; exactly 0 when
  the same option wins in every world.
- Worked example: same scenarios as above → `Some(1.2)`

### `evpi_from_psa_draws(draws: &[Vec<f64>]) -> Option<f64>`

- Formula: builds one `Scenario` per draw with equal probability `1 / draws.len()`,
  then computes `evpi` on those scenarios.
- Returns `None` iff `draws` is empty (or, transitively through `evpi`, if a
  draw has no options).
- Worked example: 6 draws of `[8.0, 0.0]` and 4 draws of `[-3.0, 0.0]`
  (reproducing the `p = 0.6/0.4` worlds) → `Some(1.2)`

### `population_evpi(evpi_per_decision: f64, decisions_affected: f64) -> f64`

- Formula: `evpi_per_decision × decisions_affected`
- Total function: never returns `None`.
- Worked example: `population_evpi(1.2, 10.0) == 12.0`

## Invariants

- `EVPI = E[max] − max E[] ≥ 0` always, because `E_θ[max_j NMB(j,θ)] ≥ max_j E_θ[NMB(j,θ)]`
  — the module's rustdoc states this as the defining property of the metric,
  not just an example outcome.
- `evpi_from_psa_draws` with `n` equally-weighted draws is equivalent to
  calling `evpi` on the corresponding discrete-scenario representation with
  probability `1/n` per draw.
