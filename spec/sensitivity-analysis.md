# Spec: Sensitivity Analysis

- **Module**: [`src/sensitivity_analysis.rs`](../src/sensitivity_analysis.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/sensitivity-analysis.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `CodingAssistantCase`

Base-case model for the worked example (an AI coding assistant for a
developer organization), held in one struct so a DSA can copy the base case
and vary one field at a time.

- `developers: f64` — number of developers licensed.
- `hours_saved_per_day: f64` — hours saved per developer per day (the most
  contested estimate; worked example's plausible range is 0.1–1.0 h/day).
- `loaded_cost_per_hour: f64` — loaded developer cost per hour, £/hour.
- `working_days_per_year: f64` — working days per year (base case 220).
- `license_per_dev_per_month: f64` — license cost per developer per month.

### `CodingAssistantCase::annual_benefit(&self) -> f64`

- Formula: `developers * hours_saved_per_day * working_days_per_year * loaded_cost_per_hour`
- Total function: never returns `None`.
- Worked example: base case (200, 0.5, 60.0, 220.0, 39.0) gives
  `annual_benefit() == 1_320_000.0`

### `CodingAssistantCase::annual_cost(&self) -> f64`

- Formula: `developers * license_per_dev_per_month * 12.0`
- Total function: never returns `None`.
- Worked example: base case gives `annual_cost() == 93_600.0`

### `CodingAssistantCase::net_benefit(&self) -> f64`

- Formula: `annual_benefit() - annual_cost()`
- Total function: never returns `None` (negative when the license costs more
  than the time it saves).
- Worked example: base case gives `net_benefit() == 1_226_400.0`

### `CodingAssistantCase::threshold_hours_saved_per_day(&self) -> Option<f64>`

- Formula: `annual_cost() / (developers * working_days_per_year * loaded_cost_per_hour)`
  (solves `net_benefit = 0` for `hours_saved_per_day`)
- Returns `None` iff the benefit-per-hour-saved denominator
  (`developers * working_days_per_year * loaded_cost_per_hour`) is `0.0`
- Worked example: base case gives
  `threshold_hours_saved_per_day().unwrap() * 60.0 ≈ 2.1` minutes/day (tolerance 0.05)

### `OneWayResult`

Results of a one-way DSA on a single parameter.

- `result_at_low: f64` — model result at the parameter's low value.
- `result_at_high: f64` — model result at the parameter's high value.

### `OneWayResult::swing(&self) -> f64`

- Formula: `(result_at_high - result_at_low).abs()`
- Total function: never returns `None`.
- Worked example: `OneWayResult { result_at_low: 170_400.0, result_at_high: 2_546_400.0 }.swing() == 2_376_000.0`

### `rank_by_swing(parameter_swings: &mut [(&str, f64)])`

- Sorts `(name, swing)` pairs in place, descending by swing (the
  tornado-diagram ordering, dominant parameter first). Non-comparable swings
  (NaN) are treated as equal rather than panicking.
- Returns `()`: mutates the slice in place rather than returning a value.
- Worked example: given
  `[("license price", 48_000.0), ("time saved", 2_376_000.0), ("working days", 240_000.0), ("loaded cost", 880_000.0)]`,
  after `rank_by_swing`, `swings[0].0 == "time saved"` and `swings[3].0 == "license price"`

## Invariants

- `rank_by_swing` never panics on any `f64` input, including `NaN`.
