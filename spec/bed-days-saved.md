# Spec: Bed Days Saved

- **Module**: [`src/bed_days_saved.rs`](../src/bed_days_saved.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/bed-days-saved.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `bed_days_saved_from_earlier_discharge(patients_affected: f64, length_of_stay_reduction_days: f64) -> f64`

- Formula: `patients_affected * length_of_stay_reduction_days`
- Total function: never returns `None`.
- Worked example: `bed_days_saved_from_earlier_discharge(600.0, 2.0) == 1_200.0`

### `bed_days_saved_from_avoided_admissions(admissions_avoided: f64, average_length_of_stay_days: f64) -> f64`

- Formula: `admissions_avoided * average_length_of_stay_days`
- Total function: never returns `None`. Never claim both this and `bed_days_saved_from_earlier_discharge` for the same freed bed (double counting).
- Worked example: `bed_days_saved_from_avoided_admissions(400.0, 3.0) == 1_200.0`

### `FreedCapacityUse`

What happens to the freed bed capacity — determines what kind of value the
bed days carry. An enum with variants:

- `RefilledWithElective { average_elective_stay_days: f64, income_per_spell: f64 }` — beds refilled with elective activity; value is non-cash-releasing funded activity income.
- `WardClosedOrFlexedDown { cost_released_per_bed_day: f64 }` — a ward closes or flexes down; staffing and running cost genuinely released (cash) per bed day.
- `AbsorbedAsSlack { marginal_hotel_cost_per_bed_day: f64 }` — capacity absorbed as slack; worth only the marginal (hotel) cost, typically £50–£150/day.

### `freed_capacity_value(bed_days_saved: f64, use_of_capacity: &FreedCapacityUse) -> Option<f64>`

- Formula:
  - `RefilledWithElective`: `bed_days_saved / average_elective_stay_days * income_per_spell`
  - `WardClosedOrFlexedDown`: `bed_days_saved * cost_released_per_bed_day`
  - `AbsorbedAsSlack`: `bed_days_saved * marginal_hotel_cost_per_bed_day`
- Returns `None` iff `use_of_capacity` is `RefilledWithElective` with `average_elective_stay_days == 0.0`; the other two variants always yield a value.
- Worked example: `freed_capacity_value(1_200.0, &FreedCapacityUse::RefilledWithElective { average_elective_stay_days: 3.0, income_per_spell: 6_000.0 }) == Some(2_400_000.0)`; `freed_capacity_value(1_200.0, &FreedCapacityUse::AbsorbedAsSlack { marginal_hotel_cost_per_bed_day: 100.0 }) == Some(120_000.0)`

### `additional_elective_spells(bed_days_saved: f64, average_elective_stay_days: f64) -> Option<f64>`

- Formula: `bed_days_saved / average_elective_stay_days`
- Returns `None` iff `average_elective_stay_days == 0.0`
- Worked example: `additional_elective_spells(1_200.0, 3.0) == Some(400.0)`

### `naive_bed_day_value(bed_days_saved: f64, average_cost_per_bed_day: f64) -> f64`

- Formula: `bed_days_saved * average_cost_per_bed_day`
- Total function: never returns `None`. Provided to compute and contrast the naive (usually wrong) claim.
- Worked example: `naive_bed_day_value(1_200.0, 400.0) == 480_000.0`

## Invariants

- `freed_capacity_value`'s `RefilledWithElective` arm uses
  `additional_elective_spells` internally (bed days ÷ average elective stay)
  before multiplying by `income_per_spell`; both functions return `None` on
  the same zero-denominator condition.
- `naive_bed_day_value` (average-cost valuation) is only honest when
  `use_of_capacity` is `WardClosedOrFlexedDown` with the same per-bed-day
  figure — for `RefilledWithElective` or `AbsorbedAsSlack`, the naive value
  overstates the true `freed_capacity_value`, as shown in the worked example
  (naive £480,000 vs. slack-absorption £120,000).
- Never claim `bed_days_saved_from_earlier_discharge` and
  `bed_days_saved_from_avoided_admissions` for the same freed bed (double
  counting), per the module's pitfalls.
