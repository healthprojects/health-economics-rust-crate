# Spec: Technical Debt

- **Module**: [`src/technical_debt.rs`](../src/technical_debt.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/technical-debt.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `SqaleGrade`

SonarQube-style maintainability grade derived from the technical debt ratio.
Bands are inclusive at the upper bound: `A` (TDR ≤ 5%), `B` (5% < TDR ≤ 10%),
`C` (10% < TDR ≤ 20%), `D` (20% < TDR ≤ 50%), `E` (TDR > 50%).

### `sqale_principal(remediation_hours: f64, cost_per_hour: f64) -> f64`

- Formula: `remediation_hours * cost_per_hour`
- Total function: never returns `None`.
- Worked example: `sqale_principal(3_800.0, 75.0) == 285_000.0`

### `technical_debt_ratio_percent(remediation_cost: f64, redevelopment_cost: f64) -> Option<f64>`

- Formula: `remediation_cost / redevelopment_cost * 100`
- Returns `None` iff `redevelopment_cost == 0.0`
- Worked example: `technical_debt_ratio_percent(285_000.0, 2_375_000.0) == Some(12.0)`

### `sqale_grade(tdr_percent: f64) -> SqaleGrade`

- Formula: bands A ≤ 5%, B ≤ 10%, C ≤ 20%, D ≤ 50%, E > 50% (inclusive at
  each upper bound)
- Total function: never returns `None`.
- Worked example: `sqale_grade(12.0) == SqaleGrade::C`

### `annual_interest(dev_hours_absorbed_per_year: f64, velocity_drag_fraction: f64, cost_per_hour: f64, extra_failures_per_year: f64, cost_per_failure: f64) -> f64`

- Formula: `dev_hours_absorbed_per_year * velocity_drag_fraction * cost_per_hour + extra_failures_per_year * cost_per_failure`
- Total function: never returns `None`.
- Worked example: `annual_interest(6_000.0, 0.40, 75.0, 12.0, 8_000.0) == 276_000.0`

### `interest_avoided_per_year(annual_interest: f64, interest_reduction_fraction: f64) -> f64`

- Formula: `annual_interest * interest_reduction_fraction`
- Total function: never returns `None`.
- Worked example: `interest_avoided_per_year(276_000.0, 0.60) == 165_600.0`

### `payback_period_years(remediation_cost: f64, interest_avoided_per_year: f64) -> Option<f64>`

- Formula: `remediation_cost / interest_avoided_per_year`
- Returns `None` iff `interest_avoided_per_year == 0.0`
- Worked example: `payback_period_years(85_500.0, 165_600.0).unwrap() * 12.0 ≈ 6.0` months (tolerance 0.5)

### `pv_of_interest_avoided(interest_avoided_per_year: f64, discount_rate: f64, horizon_years: u32) -> f64`

- Formula: `Σ_{t=1}^{horizon_years} interest_avoided_per_year / (1 + discount_rate)^t`
  (nothing accrues in year 0; with `discount_rate = 0` this equals
  `horizon_years * interest_avoided_per_year`)
- Total function: never returns `None`.
- Worked example: `pv_of_interest_avoided(165_600.0, 0.0, 3) == 496_800.0`

### `paydown_net_value(pv_interest_avoided: f64, remediation_cost: f64) -> f64`

- Formula: `pv_interest_avoided - remediation_cost`
- Total function: never returns `None` (negative means a bad trade).
- Worked example: `paydown_net_value(pv_of_interest_avoided(400_000.0, 0.0, 5), 500_000.0) > 0.0`
  (a good trade); `paydown_net_value(pv_of_interest_avoided(40_000.0, 0.0, 5), 500_000.0) < 0.0`
  (a bad trade)

## Invariants

- `technical_debt_ratio_percent`'s output feeds directly into `sqale_grade`:
  the worked example chains `technical_debt_ratio_percent(...).unwrap()` into
  `sqale_grade(...)`.
- Principal (`sqale_principal`) alone justifies nothing per the module's
  rustdoc; the paydown case is `paydown_net_value(pv_of_interest_avoided(...), remediation_cost)`,
  which requires quantifying `annual_interest` and `interest_avoided_per_year`
  first — principal-only reporting is called out as a pitfall.
