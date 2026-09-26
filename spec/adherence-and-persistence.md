# Spec: Adherence and Persistence

- **Module**: [`src/adherence_and_persistence.rs`](../src/adherence_and_persistence.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/adherence-and-persistence.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `mpr_percent(total_days_supply_dispensed: f64, days_in_period: f64) -> Option<f64>`

- Formula: `total_days_supply_dispensed / days_in_period * 100` (uncapped, can exceed 100%)
- Returns `None` iff `days_in_period == 0.0`
- Worked example: `mpr_percent(400.0, 365.0) > Some(100.0)` (exact value ≈ 109.6)

### `pdc_percent(days_covered: f64, days_in_period: f64) -> Option<f64>`

- Formula: `min(days_covered / days_in_period * 100, 100.0)` (capped at 100%)
- Returns `None` iff `days_in_period == 0.0`
- Worked example: `pdc_percent(400.0, 365.0) == Some(100.0)`; `pdc_percent(292.0, 365.0) == Some(80.0)`

### `digital_adherence_percent(actual_usage_events: f64, prescribed_usage_events: f64) -> Option<f64>`

- Formula: `actual_usage_events / prescribed_usage_events * 100`
- Returns `None` iff `prescribed_usage_events == 0.0`
- Worked example: `digital_adherence_percent(4.0, 6.0) == Some(400.0 / 6.0)` (≈66.7%)

### `persistence_days(initiation_day: f64, discontinuation_day: f64) -> f64`

- Formula: `discontinuation_day - initiation_day`
- Total function: never returns `None`. Negative if arguments are reversed; callers should pass `discontinuation_day >= initiation_day`.
- Worked example: `persistence_days(0.0, 42.0) == 42.0`

### `percent_persistent(still_persistent: f64, cohort_size: f64) -> Option<f64>`

- Formula: `still_persistent / cohort_size * 100`
- Returns `None` iff `cohort_size == 0.0`
- Worked example: `percent_persistent(380.0, 1_000.0) == Some(38.0)`

### `is_adherent(adherence_percent: f64) -> bool`

- Formula: `adherence_percent >= 80.0` (inclusive at exactly 80.0)
- Total function: never returns `None`.
- Worked example: `is_adherent(80.0) == true`; `is_adherent(79.9) == false`

### `payer_spend(prescriptions: f64, price_per_prescription: f64) -> f64`

- Formula: `prescriptions * price_per_prescription`
- Total function: never returns `None`.
- Worked example: `payer_spend(1_000.0, 250.0) == 250_000.0`

### `qalys_realized(prescriptions: f64, fraction_reaching_minimum_effective_dose: f64, qalys_per_effectively_dosed_patient: f64) -> f64`

- Formula: `prescriptions * fraction_reaching_minimum_effective_dose * qalys_per_effectively_dosed_patient`
- Total function: never returns `None`.
- Worked example: `qalys_realized(1_000.0, 0.38, 0.025) == 9.5`

### `cost_per_qaly(total_spend: f64, qalys: f64) -> Option<f64>`

- Formula: `total_spend / qalys`
- Returns `None` iff `qalys == 0.0`
- Worked example: `cost_per_qaly(250_000.0, 9.5) ≈ Some(26_300.0)`

## Invariants

- MPR (`mpr_percent`) and PDC (`pdc_percent`) share the same numerator/denominator
  shape but differ in one respect: MPR is uncapped and can exceed 100%
  (overestimates via early refills), while PDC is capped at 100% — on the
  same inputs `pdc_percent <= mpr_percent`.
- `is_adherent` is meant to be applied to a PDC (or MPR) percentage output by
  `pdc_percent`/`mpr_percent`; the ≥80% threshold is the conventional bar
  that feeds US Medicare Star Ratings.
- `qalys_realized` implements value gating: only the
  `fraction_reaching_minimum_effective_dose` of `prescriptions` contributes
  QALYs — below the minimum effective dose the dose-response function is
  treated as ≈0, so cost (`payer_spend`) is incurred without benefit.
