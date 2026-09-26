# Spec: Cost-Minimization Analysis (CMA)

- **Module**: [`src/cost_minimization_analysis.rs`](../src/cost_minimization_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-minimization-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Selection`

- Enum with variants `OptionA` (option A is cheapest, or wins the tie-break on equal cost) and `OptionB` (option B is strictly cheapest).

### `outcomes_equivalent(effect_a: f64, effect_b: f64, margin: f64) -> bool`

- Formula: `(effect_a - effect_b).abs() <= margin`
- Returns a `bool`, not an `Option`: `true` if the absolute difference is within the pre-specified margin δ.
- Worked example: `outcomes_equivalent(94.1, 93.8, 2.0) == true`; `outcomes_equivalent(94.1, 91.0, 2.0) == false`

### `CostLines`

- `licences: f64` — licence costs over the horizon.
- `integration: f64` — integration/migration (switching) costs.
- `training_support: f64` — training and support costs over the horizon.

### `CostLines::total(&self) -> f64`

- Formula: `self.licences + self.integration + self.training_support`
- Total function: never returns `None`.
- Worked example: `CostLines { licences: 210_000.0, integration: 150_000.0, training_support: 90_000.0 }.total() == 450_000.0`

### `cost_minimization(cost_a: f64, cost_b: f64, equivalence_evidenced: bool) -> Option<Selection>`

- Formula: if `equivalence_evidenced`, `Some(if cost_b < cost_a { OptionB } else { OptionA })`
- Returns `None` iff `equivalence_evidenced == false` (CMA is invalid without evidenced equivalence — use CEA/CUA instead).
- Worked example: `cost_minimization(500_000.0, 450_000.0, true) == Some(Selection::OptionB)`; `cost_minimization(500_000.0, 450_000.0, false).is_none()`

### `cost_saving(cost_a: f64, cost_b: f64) -> f64`

- Formula: `(cost_a - cost_b).abs()`
- Total function: never returns `None` (always non-negative).
- Worked example: `cost_saving(500_000.0, 450_000.0) == 50_000.0`

## Invariants

- The order of operations is the point of the module: `cost_minimization`
  gates the cost comparison entirely on `equivalence_evidenced` — the cost
  values themselves are never consulted unless equivalence was evidenced
  first (per the module rustdoc, "if equivalence cannot be evidenced, CMA is
  invalid").
- Ties in `cost_minimization` are broken in favor of `OptionA`.
