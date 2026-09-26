# Spec: Hard Cash-Releasing Savings (Deficit Defense)

- **Module**: [`src/hard_cash_releasing_savings_deficit_defense.rs`](../src/hard_cash_releasing_savings_deficit_defense.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/hard-cash-releasing-savings-deficit-defense.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `premium_shift_saving(premium_shifts_avoided: f64, premium_rate_per_shift: f64, substantive_rate_per_shift: f64) -> f64`

- Formula: `premium_shifts_avoided × (premium_rate_per_shift − substantive_rate_per_shift)`
- Total function: never returns `None`. Only the premium over the
  substantive rate releases cash.
- Worked example: `premium_shift_saving(780.0, 180.0, 0.0) == 140_400.0`

### `overtime_saving(overtime_hours_avoided: f64, overtime_premium_per_hour: f64) -> f64`

- Formula: `overtime_hours_avoided × overtime_premium_per_hour`
- Total function: never returns `None`.
- Worked example: `overtime_saving(34_500.0, 8.0) == 276_000.0`

### `cancelled_contract_saving(contracts_cancelled: f64, contract_value: f64) -> f64`

- Formula: `contracts_cancelled × contract_value`
- Total function: never returns `None`. Annual saving; recurs only while the
  cancellation holds.
- Worked example: `cancelled_contract_saving(1.0, 50_000.0) == 50_000.0`

### `hard_saving(premium_shifts_avoided: f64, premium_rate_per_shift: f64, substantive_rate_per_shift: f64, overtime_hours_avoided: f64, overtime_premium_per_hour: f64, contracts_cancelled: f64, contract_value: f64) -> f64`

- Formula: `premium_shift_saving(...) + overtime_saving(...) + cancelled_contract_saving(...)`
  (sum of the three preceding functions)
- Total function: never returns `None`.
- Worked example: `hard_saving(780.0, 180.0, 0.0, 34_500.0, 8.0, 0.0, 0.0) == 416_400.0`

### `annual_workforce_overtime_saving(staff: f64, overtime_hours_avoided_per_week: f64, overtime_premium_per_hour: f64, weeks_per_year: f64) -> f64`

- Formula: `staff × overtime_hours_avoided_per_week × overtime_premium_per_hour × weeks_per_year`
- Total function: never returns `None`. Convenience form of `overtime_saving`
  built from weekly per-person figures.
- Worked example: `annual_workforce_overtime_saving(300.0, 2.5, 8.0, 46.0) == 276_000.0`

### `annual_bank_agency_saving(shifts_avoided_per_week: f64, premium_per_shift: f64, weeks_per_year: f64) -> f64`

- Formula: `shifts_avoided_per_week × premium_per_shift × weeks_per_year`
- Total function: never returns `None`. Convenience form of
  `premium_shift_saving`.
- Worked example: `annual_bank_agency_saving(15.0, 180.0, 52.0) == 140_400.0`

### `net_of_licence(hard_saving_total: f64, licence_cost: f64) -> f64`

- Formula: `hard_saving_total − licence_cost`
- Total function: never returns `None`. Negative means the licence costs
  more than the cash it releases.
- Worked example: `net_of_licence(416_400.0, 150_000.0) == 266_400.0`

## Invariants

- `hard_saving(...)` is exactly the sum of `premium_shift_saving`,
  `overtime_saving`, and `cancelled_contract_saving` on the corresponding
  arguments; `annual_workforce_overtime_saving` and
  `annual_bank_agency_saving` are convenience forms that expand to the same
  underlying `overtime_saving`/`premium_shift_saving` arithmetic from
  weekly/annual figures.
