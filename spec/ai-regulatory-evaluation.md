# Spec: AI Regulatory Evaluation

- **Module**: [`src/ai_regulatory_evaluation.rs`](../src/ai_regulatory_evaluation.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/ai-regulatory-evaluation.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `traditional_update_cost(submission_cost: f64, review_months: f64, cost_of_delay_per_month: f64) -> f64`

- Formula: `submission_cost + review_months * cost_of_delay_per_month`
- Total function: never returns `None`.
- Worked example: `traditional_update_cost(80_000.0, 4.0, 50_000.0) == 280_000.0`

### `traditional_lifetime_cost(n_updates: f64, submission_cost: f64, review_months: f64, cost_of_delay_per_month: f64) -> f64`

- Formula: `n_updates * traditional_update_cost(submission_cost, review_months, cost_of_delay_per_month)`
- Total function: never returns `None`.
- Worked example: `traditional_lifetime_cost(12.0, 80_000.0, 4.0, 50_000.0) == 3_360_000.0`

### `pccp_lifetime_cost(pccp_authoring_cost: f64, n_updates: f64, protocol_execution_cost: f64) -> f64`

- Formula: `pccp_authoring_cost + n_updates * protocol_execution_cost`
- Total function: never returns `None`.
- Worked example: `pccp_lifetime_cost(250_000.0, 12.0, 30_000.0) == 610_000.0`

### `pccp_saving(traditional_lifetime_cost: f64, pccp_lifetime_cost: f64) -> f64`

- Formula: `traditional_lifetime_cost - pccp_lifetime_cost`
- Total function: never returns `None`. Negative if the PCCP route costs more.
- Worked example: `pccp_saving(3_360_000.0, 610_000.0) == 2_750_000.0`

### `benefit_months_gained(n_updates: f64, review_months_avoided_per_update: f64) -> f64`

- Formula: `n_updates * review_months_avoided_per_update`
- Total function: never returns `None`.
- Worked example: `benefit_months_gained(12.0, 4.0) == 48.0`

## Invariants

- `traditional_lifetime_cost` is defined in terms of `traditional_update_cost`
  (per-update cost times `n_updates`); `pccp_lifetime_cost` amortizes a single
  fixed `pccp_authoring_cost` across `n_updates` protocol executions instead.
- `pccp_saving` is meant to be computed from the outputs of
  `traditional_lifetime_cost` and `pccp_lifetime_cost` on the same `n_updates`
  and product life, as in the worked example (£3.36M − £610k = £2.75M).
