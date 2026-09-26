# Spec: Willingness-to-Pay Thresholds

- **Module**: [`src/willingness_to_pay_thresholds.rs`](../src/willingness_to_pay_thresholds.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/willingness-to-pay-thresholds.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `icer(incremental_cost: f64, incremental_effect: f64) -> Option<f64>`

- Formula: `incremental_cost / incremental_effect`
- Returns `None` iff `incremental_effect == 0.0`
- Worked example: `icer(800.0, 0.05) == Some(16_000.0)`

### `net_monetary_benefit(threshold: f64, incremental_effect: f64, incremental_cost: f64) -> f64`

- Formula: `threshold * incremental_effect - incremental_cost`
- Total function: never returns `None`.
- Worked example: `net_monetary_benefit(20_000.0, 0.05, 800.0) == 200.0`

### `adopt_by_icer(incremental_cost: f64, incremental_effect: f64, threshold: f64) -> Option<bool>`

- Formula: `icer(incremental_cost, incremental_effect) < threshold`
- Returns `None` iff `incremental_effect == 0.0` (no ICER exists; use
  `adopt_by_nmb` instead, which is defined even at `incremental_effect == 0.0`)
- Worked example: `adopt_by_icer(800.0, 0.05, 20_000.0) == Some(true)`

### `adopt_by_nmb(threshold: f64, incremental_effect: f64, incremental_cost: f64) -> bool`

- Formula: `net_monetary_benefit(threshold, incremental_effect, incremental_cost) > 0.0`
- Total function: never returns `None` (equivalent to `adopt_by_icer` for
  positive `incremental_effect`, but defined even when it is zero, since it
  never divides).
- Worked example: `adopt_by_nmb(20_000.0, 0.05, 800.0) == true`

### `max_defensible_price(threshold: f64, qalys_gained: f64, cost_offsets: f64) -> f64`

- Formula: `threshold * qalys_gained + cost_offsets`
- Total function: never returns `None`.
- Worked example: `max_defensible_price(20_000.0, 0.05, 0.0) == 1_000.0`

## Invariants

- `adopt_by_icer` and `adopt_by_nmb` are equivalent decision rules whenever
  `incremental_effect != 0.0`: `adopt_by_icer(c, e, t) == Some(adopt_by_nmb(t, e, c))`.
  `adopt_by_nmb` is total and remains defined at `incremental_effect == 0.0`,
  where `adopt_by_icer` returns `None`.
- `max_defensible_price` is monotonically increasing in `threshold`: the
  module's worked example prices the same 0.05-QALY product at three
  thresholds (England £20,000 → £1,000; a $4,000 GDP-per-capita threshold →
  $200; US $150,000 → $7,500), illustrating that the threshold *is* the
  pricing model.
