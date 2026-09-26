# Spec: Cash-Releasing vs Non-Cash-Releasing Savings

- **Module**: [`src/cash_releasing_vs_non_cash_releasing.rs`](../src/cash_releasing_vs_non_cash_releasing.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/cash-releasing-vs-non-cash-releasing.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `SavingCategory`

What actually happens to a saved hour, which determines its category. An
enum with variants:

- `CashReleasing` — a budget line genuinely shrinks (cancelled shift, contract, purchase).
- `NonCashReleasing` — time is redeployed to more output; real value, but not extractable money.
- `NoBenefit` — time dissipates into slack; claiming any value would be fiction.

### `cash_releasing_saving(budget_line_before: f64, budget_line_after: f64) -> f64`

- Formula: `budget_line_before - budget_line_after`
- Total function: never returns `None`. Positive when the line shrank.
- Worked example: `cash_releasing_saving(500_000.0, 419_500.0) == 80_500.0`

### `non_cash_releasing_value(hours_released: f64, unit_cost_per_hour: f64) -> f64`

- Formula: `hours_released * unit_cost_per_hour`
- Total function: never returns `None`. Applied to all saved hours at loaded salary, this produces the "tempting headline" figure the honest split (via `category_value`) is meant to correct.
- Worked example: `non_cash_releasing_value(11_500.0, 25.0) == 287_500.0`

### `annual_hours_saved(staff_count: f64, hours_saved_per_shift: f64, shifts_per_week: f64, weeks_per_year: f64) -> f64`

- Formula: `staff_count * hours_saved_per_shift * shifts_per_week * weeks_per_year`
- Total function: never returns `None`.
- Worked example: `annual_hours_saved(100.0, 0.5, 5.0, 46.0) == 11_500.0`

### `TimeAllocation`

One slice of the honest split of saved time. Fractions across a full split
should sum to 1.0; the `NoBenefit` slice's rate is ignored because
dissipated time is worth exactly £0.

- `category: SavingCategory` — where this slice of the saved time actually goes.
- `fraction: f64` — fraction of total saved hours in this slice (0–1).
- `hourly_rate: f64` — £ per hour used to value this slice (ignored for `NoBenefit`).

### `TimeAllocation::hours(&self, total_hours: f64) -> f64`

- Formula: `total_hours * self.fraction`
- Total function: never returns `None`.
- Worked example: for `TimeAllocation { category: CashReleasing, fraction: 0.20, hourly_rate: 35.0 }`, `slice.hours(11_500.0) == 2_300.0`

### `TimeAllocation::value(&self, total_hours: f64) -> f64`

- Formula: `0.0` if `self.category == SavingCategory::NoBenefit`; otherwise `self.hours(total_hours) * self.hourly_rate`
- Total function: never returns `None`.
- Worked example: for `TimeAllocation { category: CashReleasing, fraction: 0.20, hourly_rate: 35.0 }`, `cash.value(11_500.0) == 80_500.0`; for a `NoBenefit` slice, `slack.value(11_500.0) == 0.0` regardless of `hourly_rate`

### `category_value(total_hours: f64, allocations: &[TimeAllocation], category: SavingCategory) -> f64`

- Formula: `sum over allocations where a.category == category of a.value(total_hours)`
- Total function: never returns `None`; returns `0.0` if no slice matches, and always `0.0` for `SavingCategory::NoBenefit`.
- Worked example: for the worked-example split (20% `CashReleasing` at £35/h, 60% `NonCashReleasing` at £25/h, 20% `NoBenefit`), `category_value(11_500.0, &split, SavingCategory::CashReleasing) == 80_500.0` and `category_value(11_500.0, &split, SavingCategory::NonCashReleasing) == 172_500.0`

## Invariants

- `category_value` is `TimeAllocation::value` summed across matching slices;
  for a single-slice-per-category split (as in the worked example), the two
  give identical results.
- Cash-releasing and non-cash-releasing totals from `category_value` must be
  reported separately, never summed into a single "savings" figure — the
  module's worked example explicitly checks
  `cash + capacity < non_cash_releasing_value(total_hours, rate)` (i.e. the
  honest split total is smaller than the "tempting headline" that applies
  one rate to all hours).
- `non_cash_releasing_value` applied to the full `annual_hours_saved` output
  at a single loaded rate reproduces the "canonical sin" headline figure
  that the category-by-category split (via `TimeAllocation` and
  `category_value`) is meant to replace.
