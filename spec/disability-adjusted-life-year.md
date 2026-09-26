# Spec: Disability-Adjusted Life Year (DALY)

- **Module**: [`src/disability_adjusted_life_year.rs`](../src/disability_adjusted_life_year.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/disability-adjusted-life-year.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `WhoChoiceBand`

- Enum with variants `HighlyCostEffective` (cost per DALY averted < 1× GDP per capita), `CostEffective` (between 1× and 3× GDP per capita), and `NotCostEffective` (> 3× GDP per capita).

### `years_of_life_lost(deaths: f64, life_expectancy_at_death: f64) -> f64`

- Formula: `deaths * life_expectancy_at_death`
- Total function: never returns `None`.
- Worked example: `years_of_life_lost(10.0, 20.0) == 200.0`

### `years_lived_with_disability(prevalence: f64, disability_weight: f64) -> f64`

- Formula: `prevalence * disability_weight`
- Total function: never returns `None`.
- Worked example: `years_lived_with_disability(200.0, 0.2) == 40.0`

### `dalys(yll: f64, yld: f64) -> f64`

- Formula: `yll + yld`
- Total function: never returns `None`.
- Worked example: `dalys(200.0, 40.0) == 240.0`

### `cost_per_daly_averted(annual_cost: f64, dalys_averted: f64) -> Option<f64>`

- Formula: `annual_cost / dalys_averted`
- Returns `None` iff `dalys_averted == 0.0`
- Worked example: `cost_per_daly_averted(600_000.0, 240.0) == Some(2_500.0)`; `cost_per_daly_averted(600_000.0, 0.0) == None`

### `who_choice_band(cost_per_daly_averted: f64, gdp_per_capita: f64) -> WhoChoiceBand`

- Formula: `HighlyCostEffective` if `cost_per_daly_averted < gdp_per_capita`; else `CostEffective` if `cost_per_daly_averted <= 3.0 * gdp_per_capita`; else `NotCostEffective`
- Total function: never returns `None`.
- Worked example: `who_choice_band(2_500.0, 8_000.0) == WhoChoiceBand::HighlyCostEffective`; `who_choice_band(10_000.0, 8_000.0) == WhoChoiceBand::CostEffective`; `who_choice_band(30_000.0, 8_000.0) == WhoChoiceBand::NotCostEffective`

## Invariants

- `dalys` composes with its two components: `dalys(years_of_life_lost(deaths,
  life_expectancy), years_lived_with_disability(prevalence, weight))` is the
  standard pattern for computing DALYs averted by an intervention, per the
  module's worked example.
- `who_choice_band`'s bands partition the non-negative reals at exactly 1×
  and 3× `gdp_per_capita`, with the boundary at 3× included in
  `CostEffective` (`<=`) and the boundary at 1× excluded from
  `HighlyCostEffective` (`<`).
