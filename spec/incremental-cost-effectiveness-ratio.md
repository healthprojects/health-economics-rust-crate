# Spec: Incremental Cost-Effectiveness Ratio (ICER)

- **Module**: [`src/incremental_cost_effectiveness_ratio.rs`](../src/incremental_cost_effectiveness_ratio.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/incremental-cost-effectiveness-ratio.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `CostEffectivenessQuadrant`

Where an option lands on the cost-effectiveness plane relative to its
comparator (x-axis ΔE, y-axis ΔC). Variants:

- `Dominant` — cheaper (ΔC ≤ 0) and more effective (ΔE > 0): adopt without
  computing a ratio.
- `TradeOff` — costlier (ΔC > 0) and more effective (ΔE > 0): the only
  quadrant where the ICER is meaningful.
- `Dominated` — costlier (ΔC > 0) and less effective (ΔE < 0): reject
  without computing a ratio.
- `SavingsForLoss` — cheaper (ΔC ≤ 0) and less effective (ΔE < 0): a
  disinvestment trade-off.
- `OnAxis` — ΔE = 0 (or ΔC = 0 with ΔE = 0): the ratio is undefined or
  degenerate.

### `classify_quadrant(delta_cost: f64, delta_effect: f64) -> CostEffectivenessQuadrant`

- Formula/rules (checked in this order): `delta_effect == 0.0` →
  `OnAxis`; else `delta_cost <= 0.0 && delta_effect > 0.0` → `Dominant`;
  else `delta_cost > 0.0 && delta_effect > 0.0` → `TradeOff`; else
  `delta_cost > 0.0` (with `delta_effect < 0.0`) → `Dominated`; else
  (`delta_cost <= 0.0 && delta_effect < 0.0`) → `SavingsForLoss`.
- Total function: never returns `None`; always yields exactly one quadrant.
- Worked example: `classify_quadrant(300_000.0, 25.0) == CostEffectivenessQuadrant::TradeOff`;
  `classify_quadrant(-50_000.0, 10.0) == CostEffectivenessQuadrant::Dominant`;
  `classify_quadrant(50_000.0, -5.0) == CostEffectivenessQuadrant::Dominated`;
  `classify_quadrant(300_000.0, 0.0) == CostEffectivenessQuadrant::OnAxis`

### `icer(delta_cost: f64, delta_effect: f64) -> Option<f64>`

- Formula: `delta_cost / delta_effect`
- Returns `None` iff `delta_effect == 0.0`
- Worked example: `icer(300_000.0, 25.0) == Some(12_000.0)`

### `net_incremental_cost(gross_cost: f64, cost_offsets: f64) -> f64`

- Formula: `gross_cost − cost_offsets`
- Total function: never returns `None`. Negative when offsets exceed the
  gross cost (a candidate for dominance).
- Worked example: `net_incremental_cost(900_000.0, 600_000.0) == 300_000.0`

### `adopt_at_threshold(icer_value: f64, lambda: f64) -> bool`

- Formula: `icer_value < lambda` (strict comparison); only meaningful in the
  `TradeOff` quadrant — dominant and dominated options are decided by
  quadrant alone.
- Total function: never returns `None`.
- Worked example: `adopt_at_threshold(12_000.0, 20_000.0) == true`;
  `adopt_at_threshold(36_000.0, 20_000.0) == false`

## Invariants

- Negative `icer` results are ambiguous by construction — the module's
  rustdoc states that cheaper-and-better (`Dominant`) and
  costlier-and-worse (`Dominated`) both give a negative-sign relationship
  between ΔC and ΔE (in `Dominant`, ΔC ≤ 0 with ΔE > 0; in `Dominated`, ΔC >
  0 with ΔE < 0) — so `classify_quadrant`, not the raw ratio from `icer`,
  must carry the adoption decision outside the `TradeOff` quadrant.
