# Spec: Value of a Statistical Life (VSL)

- **Module**: [`src/value_of_a_statistical_life.rs`](../src/value_of_a_statistical_life.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/value-of-a-statistical-life.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `deaths_averted_from_risk_reduction(population: f64, risk_reduction_per_person: f64) -> f64`

- Formula: `population * risk_reduction_per_person`
- Total function: never returns `None` (the result is a fractional
  "statistical" death count, not a whole number of identified people).
- Worked example: `deaths_averted_from_risk_reduction(800_000.0, 0.000_001) == 0.8`

### `monetized_mortality_benefit(deaths_averted: f64, value_of_prevented_fatality: f64) -> f64`

- Formula: `deaths_averted * value_of_prevented_fatality`
- Total function: never returns `None`.
- Worked example: `monetized_mortality_benefit(0.8, 2_180_000.0) == 1_744_000.0`

## Invariants

- The module's worked example chains the two functions end-to-end:
  `monetized_mortality_benefit(deaths_averted_from_risk_reduction(800_000.0, 0.000_001), 2_180_000.0) == 1_744_000.0`.
- Using a VSL/VPF figure alongside a separate QALY-based net monetary benefit
  calculation in the same case, without reconciling the two frameworks, risks
  double counting the value of the same averted deaths — the module's
  rustdoc states this as a pitfall, not as a property enforced by the code.
