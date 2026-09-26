# Spec: Time Trade-Off (TTO) Utility Elicitation

- **Module**: [`src/time_trade_off_utility.rs`](../src/time_trade_off_utility.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/time-trade-off-utility.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `time_trade_off_utility(time_in_full_health: f64, time_in_impaired_state: f64) -> Option<f64>`

- Formula: `time_in_full_health / time_in_impaired_state`
- Returns `None` iff `time_in_impaired_state == 0.0`.
- Worked example: `time_trade_off_utility(7.0, 10.0) == Some(0.7)`

### `worse_than_death_utility(time_traded_for_death: f64, total_duration: f64) -> Option<f64>`

- Formula: `-time_traded_for_death / (total_duration - time_traded_for_death)`
- Returns `None` iff `total_duration - time_traded_for_death == 0.0`.
- Worked example: `worse_than_death_utility(2.0, 10.0) == Some(-0.25)`

## Invariants

- `time_trade_off_utility` is only valid for states considered better than
  death; `worse_than_death_utility` is the extended formulation for states
  some respondents consider worse than death. Applying the standard formula
  to a worse-than-death state produces a wrong (positive) result — the
  module's rustdoc states this as a named pitfall, not just an edge case.
