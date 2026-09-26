# Spec: Carbon Footprint per QALY

- **Module**: [`src/carbon_footprint_per_qaly.rs`](../src/carbon_footprint_per_qaly.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/carbon-footprint-per-qaly.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `carbon_per_qaly(total_emissions_tonnes_co2e: f64, total_qalys: f64) -> Option<f64>`

- Formula: `total_emissions_tonnes_co2e / total_qalys`
- Returns `None` iff `total_qalys == 0.0`
- Worked example: `carbon_per_qaly(-40.0, 25.0) == Some(-1.6)`

### `monetized_carbon_impact(emissions_tonnes_co2e: f64, carbon_value_per_tonne: f64) -> f64`

- Formula: `emissions_tonnes_co2e * carbon_value_per_tonne`
- Total function: never returns `None`.
- Worked example: `monetized_carbon_impact(-40.0, 269.0) == -10_760.0`

### `carbon_adjusted_net_monetary_benefit(net_monetary_benefit: f64, emissions_tonnes_co2e: f64, carbon_value_per_tonne: f64) -> f64`

- Formula: `net_monetary_benefit - (emissions_tonnes_co2e * carbon_value_per_tonne)`
- Total function: never returns `None`.
- Worked example: `carbon_adjusted_net_monetary_benefit(500_000.0, -40.0, 269.0) == 510_760.0`

## Invariants

- `carbon_adjusted_net_monetary_benefit` internally computes the same product
  as `monetized_carbon_impact(emissions_tonnes_co2e, carbon_value_per_tonne)`
  and subtracts it from `net_monetary_benefit`; the two functions must agree
  when composed (`net_monetary_benefit - monetized_carbon_impact(e, v) ==
  carbon_adjusted_net_monetary_benefit(net_monetary_benefit, e, v)`).
- A negative `total_emissions_tonnes_co2e` or `emissions_tonnes_co2e` (net
  emissions avoided) is a valid, expected input across all three functions,
  and produces a negative `carbon_per_qaly` or `monetized_carbon_impact` — by
  convention this represents a benefit, not an error.
- `carbon_value_per_tonne` is not a fixed constant: the module's rustdoc
  notes the UK Green Book's non-traded carbon value is updated annually
  (illustrative 2023 figure ≈£269/tonne), so callers must supply a current,
  dated value rather than hardcoding it.
