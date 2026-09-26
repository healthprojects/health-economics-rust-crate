# Spec: SPACE and DevEx

- **Module**: [`src/space_and_devex.rs`](../src/space_and_devex.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/space-and-devex.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `SpaceDimension`

The five SPACE dimensions: `Satisfaction`, `Performance`, `Activity`,
`Communication`, `Efficiency`. A compliant measurement design draws metrics
from at least three of these (see `space_rule_satisfied`).

### `MetricSource`

How a metric is captured: `Perceptual` (self-report/survey) or `System`
(telemetry). The SPACE rule requires at least one of each.

### `SpaceMetric`

One metric in a SPACE measurement design.

- `dimension: SpaceDimension` — which SPACE dimension the metric belongs to.
- `source: MetricSource` — whether it is perceptual or system.

### `space_rule_satisfied(metrics: &[SpaceMetric]) -> bool`

- Rule: the metric set covers at least 3 distinct dimensions AND includes at
  least 1 `Perceptual` metric AND at least 1 `System` metric.
- Returns `bool`: `true` if the design satisfies the rule, `false` for
  telemetry-only dashboards or designs spanning fewer than three dimensions.
- Worked example: a design of `[Efficiency/System, Satisfaction/Perceptual, Performance/System]`
  gives `space_rule_satisfied(&metrics) == true`; a telemetry-only design of
  `[Activity/System, Efficiency/System, Performance/System]` gives `false`

### `time_reclaimed_minutes_per_day(builds_per_day: f64, minutes_saved_per_build: f64, usable_fraction: f64) -> f64`

- Formula: `builds_per_day * minutes_saved_per_build * usable_fraction`
- Total function: never returns `None`.
- Worked example: `time_reclaimed_minutes_per_day(6.0, 19.0, 0.4) == 45.6`

### `capacity_value_per_year(developers: f64, hours_reclaimed_per_day: f64, working_days_per_year: f64, loaded_cost_per_hour: f64) -> f64`

- Formula: `developers * hours_reclaimed_per_day * working_days_per_year * loaded_cost_per_hour`
- Total function: never returns `None`.
- Worked example: `capacity_value_per_year(300.0, 0.75, 220.0, 60.0) == 2_970_000.0`

### `vendor_index_minutes_per_week(index_points_gained: f64, minutes_per_point: f64) -> f64`

- Formula: `index_points_gained * minutes_per_point`
- Total function: never returns `None`.
- Worked example: `vendor_index_minutes_per_week(1.0, 13.0) == 13.0`

## Invariants

- `capacity_value_per_year` is a **non-cash-releasing** capacity valuation:
  it values reclaimed developer time at the loaded rate, not as bankable
  cash — the module's rustdoc requires this to be labeled accordingly in any
  business case.
- `vendor_index_minutes_per_week`'s `minutes_per_point` benchmark (e.g. DX's
  DXI ≈ 13 min/dev/week per index point) is a vendor claim to validate
  locally, not a constant of nature, and index points must never be compared
  across vendors' instruments.
