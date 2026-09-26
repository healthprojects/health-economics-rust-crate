# Spec: Health App Unit Economics

- **Module**: [`src/health_app_unit_economics.rs`](../src/health_app_unit_economics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/health-app-unit-economics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `cac(sales_and_marketing_spend: f64, new_paying_customers: f64) -> Option<f64>`

- Formula: `sales_and_marketing_spend / new_paying_customers`
- Returns `None` iff `new_paying_customers == 0.0`
- Worked example: `cac(38_000.0, 1_000.0) == Some(38.0)`

### `arpu(revenue: f64, active_users: f64) -> Option<f64>`

- Formula: `revenue / active_users`
- Returns `None` iff `active_users == 0.0`
- Worked example: `arpu(6_990.0, 1_000.0) == Some(6.99)`

### `ltv(arpu: f64, churn_rate: f64) -> Option<f64>`

- Formula: `arpu / churn_rate` (equivalent to `arpu × average_lifetime`,
  since `average_lifetime = 1 / churn_rate` under constant churn)
- Returns `None` iff `churn_rate == 0.0` (infinite lifetime)
- Worked example: `ltv(6.99, 0.18) ≈ Some(38.8)` (module doctest asserts
  `(v - 38.8).abs() < 0.05`)

### `ltv_cac_ratio(ltv: f64, cac: f64) -> Option<f64>`

- Formula: `ltv / cac`
- Returns `None` iff `cac == 0.0`
- Worked example: `ltv_cac_ratio(38.83, 38.0) ≈ Some(1.0)`

### `is_viable_ltv_cac(ltv: f64, cac: f64) -> bool`

- Formula: `ltv_cac_ratio(ltv, cac) >= Some(3.0)` (the standard 3:1
  viability bar)
- Returns `false` when `cac == 0.0` (no meaningful ratio), rather than
  `None`/panicking.
- Worked example: `is_viable_ltv_cac(38.83, 38.0) == false`;
  `is_viable_ltv_cac(114.0, 38.0) == true`

### `effective_cac_per_retained_user(cac: f64, retention_at_t: f64) -> Option<f64>`

- Formula: `cac / retention_at_t`
- Returns `None` iff `retention_at_t == 0.0`
- Worked example: `effective_cac_per_retained_user(5.0, 0.04) == Some(125.0)`

### `pmpm_revenue(pmpm_rate: f64, enrolled_members: f64, months: f64) -> f64`

- Formula: `pmpm_rate × enrolled_members × months`
- Total function: never returns `None`.
- Worked example: `pmpm_revenue(1.20, 40_000.0, 1.0) == 48_000.0`

### `pmpm_margin(pmpm_rate: f64, cost_to_serve_pmpm: f64) -> f64`

- Formula: `pmpm_rate − cost_to_serve_pmpm`
- Total function: never returns `None`. Negative if serving costs exceed the
  rate.
- Worked example: `pmpm_margin(1.20, 0.30) == 0.90`

### `pmpm_margin_fraction(pmpm_rate: f64, cost_to_serve_pmpm: f64) -> Option<f64>`

- Formula: `pmpm_margin(pmpm_rate, cost_to_serve_pmpm) / pmpm_rate`
- Returns `None` iff `pmpm_rate == 0.0`
- Worked example: `pmpm_margin_fraction(1.20, 0.30) == Some(0.75)`

### `health_value_per_acquired_user(retention_weighted_qalys: f64, threshold_per_qaly: f64) -> f64`

- Formula: `retention_weighted_qalys × threshold_per_qaly`
- Total function: never returns `None`.
- Worked example: `health_value_per_acquired_user(0.01, 20_000.0) == 200.0`
