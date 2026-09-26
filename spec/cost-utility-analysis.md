# Spec: Cost-Utility Analysis (CUA)

- **Module**: [`src/cost_utility_analysis.rs`](../src/cost_utility_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-utility-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `HealthState`

- `duration_years: f64` — time spent in the state, in years (e.g. 0.5 for six months).
- `utility: f64` — preference-weighted utility of the state, 0 = dead, 1 = full health (typically from EQ-5D).

### `total_qalys(states: &[HealthState]) -> f64`

- Formula: `Σ duration_i * utility_i` over `states`
- Total function: never returns `None`.
- Worked example: `total_qalys(&[HealthState { duration_years: 0.5, utility: 0.76 }]) == 0.38`

### `delta_qalys(new_states: &[HealthState], old_states: &[HealthState]) -> f64`

- Formula: `total_qalys(new_states) - total_qalys(old_states)`
- Total function: never returns `None`.
- Worked example: for `new = [HealthState { duration_years: 0.5, utility: 0.76 }]`, `old = [HealthState { duration_years: 0.5, utility: 0.68 }]`, `delta_qalys(&new, &old) == 0.04`

### `displaced_care_saving(displacement_fraction: f64, comparator_cost: f64) -> f64`

- Formula: `displacement_fraction * comparator_cost`
- Total function: never returns `None`.
- Worked example: `displaced_care_saving(0.40, 1_700.0) == 680.0`

### `icur(delta_cost: f64, delta_qalys: f64) -> Option<f64>`

- Formula: `delta_cost / delta_qalys`
- Returns `None` iff `delta_qalys == 0.0`
- Worked example: `icur(80.0, 0.04) == Some(2_000.0)`; `icur(100.0, 0.0) == None`

### `is_dominant(delta_cost: f64, delta_qalys: f64) -> bool`

- Formula: `delta_cost < 0.0 && delta_qalys > 0.0`
- Returns a `bool`, not an `Option`: `true` iff the intervention saves money and gains health.
- Worked example: `is_dominant(-430.0, 0.04) == true`; `is_dominant(80.0, 0.04) == false`

## Invariants

- `icur` is ambiguous on its own when `delta_cost` and `delta_qalys` have
  matching signs that both indicate dominance or the reverse (a negative
  ICUR can mean "dominant" or "dominated"); the module's rustdoc directs
  callers to check `is_dominant` first before interpreting a negative or
  small ICUR.
- `delta_qalys` composes with `total_qalys`: it is defined as the difference
  of two `total_qalys` calls, so callers building `new_states`/`old_states`
  pathways can rely on that identity.
