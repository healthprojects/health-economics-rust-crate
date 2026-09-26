# Spec: Remote Patient Monitoring Economics

- **Module**: [`src/remote_patient_monitoring_economics.rs`](../src/remote_patient_monitoring_economics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/remote-patient-monitoring-economics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `CPT_99453_SETUP: f64`

Constant, `19.73`: CPT 99453, RPM setup and patient education, one-time
(2025 national average, USD).

### `CPT_99454_DEVICE_SUPPLY: f64`

Constant, `43.03`: CPT 99454, device supply + data transmission per 30 days
(2025 national average, USD); gated by the 16-day rule (>= 16 days of
readings in the 30-day period).

### `CPT_99457_FIRST_20_MIN: f64`

Constant, `47.87`: CPT 99457, first 20 minutes/month of care management
(2025 national average, USD); gated by the 20-minute rule (>= 20 logged
minutes of clinical management per month).

### `CPT_99458_ADDITIONAL_20_MIN: f64`

Constant, `38.49`: CPT 99458, each additional 20 minutes of management per
month (2025 national average, USD), billable on top of CPT 99457.

### `revenue_per_member_per_month(device_compliant_fraction: f64, device_supply_rate: f64, management_logged_fraction: f64, management_rate: f64) -> f64`

- Formula: `device_compliant_fraction × device_supply_rate + management_logged_fraction × management_rate`
- Total function: never returns `None`.
- Worked example: `revenue_per_member_per_month(0.70, CPT_99454_DEVICE_SUPPLY, 0.60, CPT_99457_FIRST_20_MIN)`
  is approximately `58.84` (module doctest asserts `(pmpm - 58.84).abs() < 0.005`)

### `monthly_revenue(enrolled: f64, revenue_pmpm: f64) -> f64`

- Formula: `enrolled × revenue_pmpm`
- Total function: never returns `None`.
- Worked example: `monthly_revenue(400.0, 58.84)` is approximately
  `23_536.0` (module doctest asserts `(monthly - 23_536.0).abs() < 1.0`)

### `annual_revenue(monthly_revenue: f64) -> f64`

- Formula: `monthly_revenue × 12.0`
- Total function: never returns `None`.
- Worked example: `annual_revenue(23_537.20)` is approximately `282_446.40`
  (module doctest asserts `(annual - 282_446.40).abs() < 0.01`)

### `annual_margin(annual_revenue: f64, annual_service_cost: f64) -> f64`

- Formula: `annual_revenue − annual_service_cost`
- Total function: never returns `None`; negative if the service runs at a
  loss.
- Worked example: `annual_margin(282_446.40, 180_000.0)` is approximately
  `102_446.40` (module doctest asserts `(margin - 102_446.40).abs() < 0.01`)

### `compliance_lever_annual_gain(enrolled: f64, compliance_fraction_increase: f64, device_supply_rate: f64) -> f64`

- Formula: `enrolled × compliance_fraction_increase × device_supply_rate × 12.0`
- Total function: never returns `None`.
- Worked example: `compliance_lever_annual_gain(400.0, 0.15, CPT_99454_DEVICE_SUPPLY)`
  is approximately `30_981.60` (module doctest asserts
  `(gain - 30_981.60).abs() < 0.01`)

### `virtual_ward_gross_annual_value(beds: f64, occupancy_fraction: f64, net_saving_per_day: f64) -> f64`

- Formula: `beds × occupancy_fraction × 365.0 × net_saving_per_day`
- Total function: never returns `None`.
- Worked example: `virtual_ward_gross_annual_value(50.0, 0.80, 150.0) == 2_190_000.0`

### `nhs_style_net_value(admissions_avoided: f64, marginal_admission_cost: f64, bed_days_substituted: f64, inpatient_day_cost: f64, virtual_ward_day_cost: f64, service_cost: f64) -> f64`

- Formula: `admissions_avoided × marginal_admission_cost + bed_days_substituted × (inpatient_day_cost − virtual_ward_day_cost) − service_cost`
- Total function: never returns `None`; negative if the service costs more
  than it offsets.
- Worked example: `nhs_style_net_value(0.0, 0.0, 14_600.0, 400.0, 250.0, 0.0) == 2_190_000.0`

## Invariants

- `nhs_style_net_value` with `admissions_avoided = 0.0`,
  `marginal_admission_cost = 0.0`, and `service_cost = 0.0` reduces
  algebraically to `bed_days_substituted × (inpatient_day_cost − virtual_ward_day_cost)`,
  which equals `virtual_ward_gross_annual_value(beds, occupancy_fraction, net_saving_per_day)`
  when `bed_days_substituted = beds × occupancy_fraction × 365.0` and
  `net_saving_per_day = inpatient_day_cost − virtual_ward_day_cost` — the
  module's own test (`nhs_style_net_value_matches_virtual_ward_form`)
  verifies this cross-check numerically.
- `revenue_per_member_per_month` only bills the compliant/logged fractions
  of a panel; enrollment alone earns nothing (module's rustdoc: "Enrollment
  ≠ revenue").
