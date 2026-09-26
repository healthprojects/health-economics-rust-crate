# Spec: Activation and Uptake

- **Module**: [`src/activation_and_uptake.rs`](../src/activation_and_uptake.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/activation-and-uptake.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `activation_rate_percent(users_completing_key_action: f64, sign_ups: f64) -> Option<f64>`

- Formula: `users_completing_key_action / sign_ups * 100`
- Returns `None` iff `sign_ups == 0.0`
- Worked example: `activation_rate_percent(5_400.0, 12_000.0) == Some(45.0)`

### `uptake_rate_percent(adopters: f64, eligible_population: f64) -> Option<f64>`

- Formula: `adopters / eligible_population * 100`
- Returns `None` iff `eligible_population == 0.0`
- Worked example: `uptake_rate_percent(12_000.0, 80_000.0) == Some(15.0)`

### `dtx_fill_rate_percent(activated_prescription_codes: f64, issued_prescriptions: f64) -> Option<f64>`

- Formula: `activated_prescription_codes / issued_prescriptions * 100`
- Returns `None` iff `issued_prescriptions == 0.0`
- Worked example: `dtx_fill_rate_percent(81.0, 100.0) == Some(81.0)`

### `value_per_completer(qalys_per_completer: f64, willingness_to_pay_per_qaly: f64, avoided_costs_per_completer: f64) -> f64`

- Formula: `qalys_per_completer * willingness_to_pay_per_qaly + avoided_costs_per_completer`
- Total function: never returns `None`.
- Worked example: `value_per_completer(0.03, 20_000.0, 180.0) == 780.0`

### `funnel_population_value(eligible_population: f64, uptake_fraction: f64, activation_fraction: f64, completion_fraction: f64, value_per_completer: f64) -> f64`

- Formula: `eligible_population * uptake_fraction * activation_fraction * completion_fraction * value_per_completer`
- Total function: never returns `None`.
- Worked example: `funnel_population_value(80_000.0, 0.15, 0.45, 1_600.0 / 5_400.0, 780.0) ≈ 1_248_000.0`

### `population_value(completers: f64, value_per_completer: f64) -> f64`

- Formula: `completers * value_per_completer`
- Total function: never returns `None`.
- Worked example: `population_value(1_600.0, 780.0) == 1_248_000.0`

### `per_eligible_person_value(population_value: f64, eligible_population: f64) -> Option<f64>`

- Formula: `population_value / eligible_population`
- Returns `None` iff `eligible_population == 0.0`
- Worked example: `per_eligible_person_value(1_248_000.0, 80_000.0) == Some(15.6)`

## Invariants

- Funnel stage fractions (`uptake_fraction`, `activation_fraction`,
  `completion_fraction`) are expressed as 0.0–1.0 fractions when passed into
  `funnel_population_value`, not as the 0–100 percentages that
  `activation_rate_percent`, `uptake_rate_percent`, and
  `dtx_fill_rate_percent` return — callers must convert between the two.
- `funnel_population_value` is linear in each stage factor: doubling any one
  factor (e.g. `uptake_fraction`) doubles the resulting population value,
  holding the others fixed.
- `population_value(completers, value_per_completer)` is the short form of
  `funnel_population_value` when the completer count is already known
  (`completers = eligible_population * uptake_fraction * activation_fraction
  * completion_fraction`).
