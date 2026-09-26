# Spec: Cost-Benefit Analysis (CBA)

- **Module**: [`src/cost_benefit_analysis.rs`](../src/cost_benefit_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-benefit-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `discount_factor(rate: f64, year: f64) -> f64`

- Formula: `1 / (1 + rate)^year`
- Total function: never returns `None`.
- Worked example: `discount_factor(GREEN_BOOK_DISCOUNT_RATE, 0.0) == 1.0`; `discount_factor(GREEN_BOOK_DISCOUNT_RATE, 1.0) == 1.0 / 1.035`

### `annuity_factor(rate: f64, years: u32) -> f64`

- Formula: `Σ_{t=1..=years} 1/(1+rate)^t` (0.0 when `years` is 0)
- Total function: never returns `None`.
- Worked example: `annuity_factor(GREEN_BOOK_DISCOUNT_RATE, 5) ≈ 4.515`

### `present_value(flows_by_year: &[f64], rate: f64) -> f64`

- Formula: `Σ_t flows_by_year[t] / (1 + rate)^t`, where index `t` is the year (index 0 = year 0, undiscounted)
- Total function: never returns `None` (returns 0.0 for an empty slice).
- Worked example: `present_value(&[1_200_000.0, 300_000.0, 300_000.0, 300_000.0, 300_000.0, 300_000.0], GREEN_BOOK_DISCOUNT_RATE) ≈ 2_555_000.0`

### `net_present_value(pv_benefits: f64, pv_costs: f64) -> f64`

- Formula: `pv_benefits - pv_costs`
- Total function: never returns `None`.
- Worked example: `net_present_value(5_102_000.0, 2_555_000.0) == 2_547_000.0`

### `benefit_cost_ratio(pv_benefits: f64, pv_costs: f64) -> Option<f64>`

- Formula: `pv_benefits / pv_costs`
- Returns `None` iff `pv_costs == 0.0`
- Worked example: `benefit_cost_ratio(5_102_000.0, 2_555_000.0).unwrap() ≈ 2.0`; `benefit_cost_ratio(1.0, 0.0).is_none()`

### `optimism_bias_cost_uplift(cost: f64, uplift: f64) -> f64`

- Formula: `cost * (1.0 + uplift)`
- Total function: never returns `None`.
- Worked example: `optimism_bias_cost_uplift(1_200_000.0, 0.40) == 1_680_000.0`

### `optimism_bias_benefit_haircut(benefit: f64, haircut: f64) -> f64`

- Formula: `benefit * (1.0 - haircut)`
- Total function: never returns `None`.
- Worked example: `optimism_bias_benefit_haircut(5_102_000.0, 0.20) == 4_081_600.0`

### `GREEN_BOOK_DISCOUNT_RATE: f64`

- Constant: `0.035` — the Green Book social time preference discount rate, as a fraction.

## Invariants

- `present_value` applied to a stream of `n` equal annual flows starting in
  year 1 is equivalent to `flow * annuity_factor(rate, n)`.
- Adoption rule stated in the module's rustdoc: adopt if `net_present_value >
  0` (equivalently `benefit_cost_ratio > 1`); rank competing projects by NPV,
  not BCR.
