# Spec: WSJF and CD3

- **Module**: [`src/wsjf_and_cd3.rs`](../src/wsjf_and_cd3.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/wsjf-and-cd3.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Feature`

A backlog item with a real-currency cost of delay and a duration on the
constrained capacity.

- `cost_of_delay_per_week: f64` — value lost per week this item is not
  delivered (e.g. £/week).
- `duration_weeks: f64` — calendar time the item occupies the constraint
  (weeks), not effort.

### `Feature::cd3(&self) -> Option<f64>`

- Formula: `cost_of_delay_per_week / duration_weeks`
- Returns `None` iff `duration_weeks == 0.0`
- Worked example: `Feature { cost_of_delay_per_week: 12_000.0, duration_weeks: 2.0 }.cd3() == Some(6_000.0)`

### `cd3(cost_of_delay_per_week: f64, duration_weeks: f64) -> Option<f64>`

- Formula: `cost_of_delay_per_week / duration_weeks`
- Returns `None` iff `duration_weeks == 0.0`
- Worked example: `cd3(30_000.0, 10.0) == Some(3_000.0)`

### `wsjf(user_business_value: f64, time_criticality: f64, risk_reduction_opportunity_enablement: f64, job_size: f64) -> Option<f64>`

- Formula: `(user_business_value + time_criticality + risk_reduction_opportunity_enablement) / job_size`
- Returns `None` iff `job_size == 0.0`
- Worked example: `wsjf(8.0, 5.0, 3.0, 2.0) == Some(8.0)`

### `total_delay_cost(features: &[Feature]) -> f64`

- Formula: `Σ_i cost_of_delay_per_week_i * start_time_i`, where `start_time_i`
  is the cumulative sum of `duration_weeks` of all items before item `i` in
  the given order (each item accrues delay cost for the weeks it waits
  before starting)
- Total function: never returns `None`.
- Worked example: for `[a, b, c]` with `a = {30_000.0, 10.0}`,
  `b = {12_000.0, 2.0}`, `c = {5_000.0, 1.0}`,
  `total_delay_cost(&[b, c, a]) == 100_000.0` and `total_delay_cost(&[a, b, c]) == 180_000.0`

### `sequence_by_cd3(features: &[Feature]) -> Vec<Feature>`

- Sorts a copy of `features` by descending `cd3()`, treating a `None` CD3
  (zero duration) as `+∞` so zero-duration items sort first. Under a shared,
  fixed capacity this is the mathematically optimal sequence for minimizing
  `total_delay_cost`.
- Total function: never returns `None`; does not mutate the input slice.
- Worked example: for `a`, `b`, `c` as above (CD3 scores 3,000 / 6,000 /
  5,000), `sequence_by_cd3(&[a, b, c]) == vec![b, c, a]`

### `sequencing_savings(features: &[Feature]) -> f64`

- Formula: `total_delay_cost(features) - total_delay_cost(&sequence_by_cd3(features))`
- Total function: never returns `None`; non-negative when CD3 order is
  optimal, zero when the given order already is CD3 order.
- Worked example: `sequencing_savings(&[a, b, c]) == 80_000.0`

## Invariants

- `sequencing_savings(features) >= 0.0` for any `features`, since
  `sequence_by_cd3` is the delay-cost-minimizing order under a single shared,
  fixed-capacity pipeline.
- `Feature::cd3(&self)` and the free function `cd3(...)` compute the same
  formula; the method is defined in terms of the free function.
