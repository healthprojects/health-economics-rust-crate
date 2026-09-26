# Spec: Germany's DiGA Fast-Track

- **Module**: [`src/diga_fast_track.rs`](../src/diga_fast_track.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/diga-fast-track.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `revenue(prescriptions: f64, activation_rate: f64, price_per_prescription: f64) -> f64`

- Formula: `prescriptions * activation_rate * price_per_prescription`
- Total function: never returns `None`.
- Worked example: `revenue(20_000.0, 0.81, 450.0) ≈ 7_290_000.0`

### `activated_prescriptions(prescriptions: f64, activation_rate: f64) -> f64`

- Formula: `prescriptions * activation_rate`
- Total function: never returns `None`.
- Worked example: `activated_prescriptions(20_000.0, 0.81) ≈ 16_200.0`

### `expected_value(probability_evidence_succeeds: f64, steady_state_revenue: f64, evidence_cost: f64) -> f64`

- Formula: `probability_evidence_succeeds * steady_state_revenue - evidence_cost`
- Total function: never returns `None`.
- Worked example: `expected_value(0.5, 18_468_000.0, 2_000_000.0) == 7_234_000.0`; `expected_value(0.0, 18_468_000.0, 2_000_000.0) == -2_000_000.0`

### `provisional_year_finances_evidence(year_one_revenue: f64, evidence_cost: f64) -> bool`

- Formula: `year_one_revenue >= evidence_cost`
- Returns a `bool`, not an `Option`: `true` iff year-one revenue at least covers the pivotal RCT cost.
- Worked example: `provisional_year_finances_evidence(revenue(20_000.0, 0.81, 450.0), 2_000_000.0) == true`; `provisional_year_finances_evidence(1_500_000.0, 2_000_000.0) == false`

## Invariants

- `revenue` feeds both `provisional_year_finances_evidence` (year-one
  revenue vs. evidence cost) and `expected_value` (steady-state revenue vs.
  evidence cost) — both downstream functions take a `revenue(...)` output as
  one of their arguments, per the module's worked example.
- `expected_value` is linear in `probability_evidence_succeeds`: at
  probability 0 it reduces exactly to `-evidence_cost` (Outcome B: evidence
  fails, revenue stops), per the module's worked example.
