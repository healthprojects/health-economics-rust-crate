# Spec: AI Return on Investment

- **Module**: [`src/ai_return_on_investment.rs`](../src/ai_return_on_investment.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/ai-return-on-investment.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `AiCostStack`

The full cost stack of an AI initiative (currency units per period); the
true ROI denominator.

- `licences_and_inference: f64` — licences and inference spend (currency).
- `integration: f64` — cost of connecting the AI into existing systems (currency).
- `data_readiness: f64` — cleaning, access, pipelines (currency).
- `evaluation: f64` — test sets, pilots, measurement (currency).
- `workflow_redesign: f64` — cost of changing how people work around the AI (currency).
- `governance_and_assurance: f64` — safety cases, review boards, audit (currency).

### `AiCostStack::total(&self) -> f64`

- Formula: `licences_and_inference + integration + data_readiness + evaluation + workflow_redesign + governance_and_assurance`
- Total function: never returns `None`.
- Worked example: for `AiCostStack { licences_and_inference: 30_000.0, integration: 40_000.0, data_readiness: 20_000.0, evaluation: 10_000.0, workflow_redesign: 15_000.0, governance_and_assurance: 5_000.0 }`, `stack.total() == 120_000.0`

### `ai_roi(attributable_benefit: f64, total_ai_cost: f64) -> Option<f64>`

- Formula: `(attributable_benefit - total_ai_cost) / total_ai_cost`
- Returns `None` iff `total_ai_cost == 0.0`
- Worked example: `ai_roi(320_000.0, 120_000.0) ≈ Some(1.67)`; `ai_roi(0.0, 120_000.0) == Some(-1.0)`

### `attributable_benefit(baseline_spend_stopped: f64, new_costs_introduced: f64) -> f64`

- Formula: `baseline_spend_stopped - new_costs_introduced`
- Total function: never returns `None`. Negative if new costs exceed the stopped spend.
- Worked example: `attributable_benefit(380_000.0, 60_000.0) == 320_000.0`

## Invariants

- `ai_roi` is meant to be called with `attributable_benefit` (or
  `AiCostStack::total()`-based cost) as computed by
  `attributable_benefit`/`AiCostStack::total`; the worked example composes
  `attributable_benefit(380_000.0, 60_000.0)` into `ai_roi(_, 120_000.0)`.
- `ai_roi` returns `-1.0` (total loss) whenever `attributable_benefit` is
  `0.0`, for any nonzero `total_ai_cost`.
