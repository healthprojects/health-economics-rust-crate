# Spec: Real Options Valuation

- **Module**: [`src/real_options_valuation.rs`](../src/real_options_valuation.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/real-options-valuation.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `risk_neutral_probability(risk_free_rate: f64, down_factor: f64, up_factor: f64) -> Option<f64>`

- Formula: `((1.0 + risk_free_rate) - down_factor) / (up_factor - down_factor)`
- Returns `None` iff `up_factor == down_factor` (the two binomial states are indistinguishable — a deliberate exact-equality guard on the degenerate input, not a rounding-sensitive comparison).
- Worked example: `risk_neutral_probability(0.08, 0.5, 1.5) == Some(0.58)`

### `option_to_expand_value(project_value: f64, up_factor: f64, down_factor: f64, expansion_cost: f64, risk_free_rate: f64) -> Option<f64>`

- Formula: computes `p` via `risk_neutral_probability`, then `payoff_up = max(project_value * up_factor - expansion_cost, 0.0)`, `payoff_down = max(project_value * down_factor - expansion_cost, 0.0)`, then `(p * payoff_up + (1.0 - p) * payoff_down) / (1.0 + risk_free_rate)`.
- Returns `None` iff `risk_neutral_probability` returns `None` (i.e. `up_factor == down_factor`).
- Worked example: `option_to_expand_value(1_000_000.0, 1.5, 0.5, 600_000.0, 0.08) ≈ Some(483_333.333_333)` (522,000 / 1.08, a non-terminating decimal — verified by running the code, not hand-derived).

### `expanded_npv(static_npv: f64, option_value: f64) -> f64`

- Formula: `static_npv + option_value`
- Total function: never returns `None`.
- Worked example: `expanded_npv(200_000.0, 483_333.333_333) ≈ 683_333.333_333`

## Invariants

- `option_to_expand_value` is always `>= 0.0` (both payoff terms are floored
  at zero before weighting) — a project's option to expand can never have
  negative value, since expansion is optional, not obligatory.
- `expanded_npv` is additive by construction: real options value is meant
  to be added to, never substituted for, a static NPV baseline.
