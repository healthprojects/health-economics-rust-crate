# Spec: National Tariff and Unit Costs

- **Module**: [`src/national_tariff_and_unit_costs.rs`](../src/national_tariff_and_unit_costs.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/national-tariff-and-unit-costs.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `ncc_unit_cost(total_cost: f64, activity_volume: f64) -> Option<f64>`

- Formula: `total_cost / activity_volume`
- Returns `None` iff `activity_volume == 0.0`
- Worked example: `ncc_unit_cost(16_000_000.0, 100_000.0) == Some(160.0)`

### `tariff_price(national_average_unit_cost: f64, market_forces_factor: f64) -> f64`

- Formula: `national_average_unit_cost × market_forces_factor`
- Total function: never returns `None`.
- Worked example: `tariff_price(160.0, 1.15) == 184.0`

### `blended_payment(fixed_element: f64, variable_price_per_unit: f64, activity_units: f64) -> f64`

- Formula: `fixed_element + variable_price_per_unit × activity_units`
- Total function: never returns `None`.
- Worked example: `blended_payment(1_000_000.0, 160.0, 500.0) == 1_080_000.0`

### `staff_capacity_value(hours_freed_per_day: f64, working_days_per_year: f64, unit_cost_per_hour: f64) -> f64`

- Formula: `hours_freed_per_day × working_days_per_year × unit_cost_per_hour`
- Total function: never returns `None`.
- Worked example: `staff_capacity_value(1.0, 250.0, 31.0) == 7_750.0`

### `redeployed_activity_value(extra_units_per_day: f64, working_days_per_year: f64, scheme_price_per_unit: f64) -> f64`

- Formula: `extra_units_per_day × working_days_per_year × scheme_price_per_unit`
- Total function: never returns `None`.
- Worked example: `redeployed_activity_value(2.0, 250.0, 160.0) == 80_000.0`

### `valuation_ratio(higher_claim: f64, lower_claim: f64) -> Option<f64>`

- Formula: `higher_claim / lower_claim`
- Returns `None` iff `lower_claim == 0.0`
- Worked example: `valuation_ratio(80_000.0, 7_750.0)` is approximately
  `Some(10.0)` (module doctest asserts `(ratio - 10.0).abs() < 0.5`)
