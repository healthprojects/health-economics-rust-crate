# Spec: Number Needed to Screen (NNS)

- **Module**: [`src/number_needed_to_screen.rs`](../src/number_needed_to_screen.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/number-needed-to-screen.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `number_needed_to_screen(baseline_risk: f64, relative_risk_reduction: f64) -> Option<f64>`

- Formula: `1.0 / (baseline_risk * relative_risk_reduction)`
- Returns `None` iff `baseline_risk * relative_risk_reduction == 0.0`.
- Worked example: `number_needed_to_screen(0.02, 0.25) == Some(200.0)`

### `screening_program_cost_per_outcome_prevented(nns: f64, cost_per_screen: f64) -> f64`

- Formula: `nns * cost_per_screen`
- Total function: never returns `None`.
- Worked example: `screening_program_cost_per_outcome_prevented(200.0, 50.0) == 10_000.0`

## Invariants

- For a fixed `relative_risk_reduction`, `number_needed_to_screen` is
  monotonically decreasing in `baseline_risk` — a lower baseline risk
  always inflates NNS (and thus programme cost per outcome prevented),
  verified by a dedicated unit test.
