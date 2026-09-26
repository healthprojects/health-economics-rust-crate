# Spec: Life-Years Gained (LYG)

- **Module**: [`src/life_years_gained.rs`](../src/life_years_gained.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/life-years-gained.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `EVLYG_FIXED_UTILITY: f64`

The fixed utility (`0.851`) the US ICER institute applies to every extended
life-year in the evLYG — the average US population utility.

### `life_years_gained_from_deaths_prevented(deaths_prevented: f64, remaining_life_years_each: f64) -> f64`

- Formula: `deaths_prevented × remaining_life_years_each`
- Total function: never returns `None`. Undiscounted; economic models apply
  discounting downstream.
- Worked example: `life_years_gained_from_deaths_prevented(12.0, 8.0) == 96.0`

### `life_years_gained_from_mean_survival(mean_survival_new: f64, mean_survival_comparator: f64) -> f64`

- Formula: `mean_survival_new − mean_survival_comparator`
- Total function: never returns `None`. Negative if the new arm is worse.
- Worked example: `life_years_gained_from_mean_survival(5.0, 3.0) == 2.0`

### `area_between_survival_curves(times: &[f64], survival_new: &[f64], survival_comparator: &[f64]) -> Option<f64>`

- Formula: trapezoidal-rule integral of the gap curve
  `g(t) = survival_new(t) − survival_comparator(t)` over `times`:
  `Σ_i dt_i × (g(t_{i-1}) + g(t_i)) / 2`, restricted to the sampled horizon
  (no extrapolation beyond the last time point). `times` must be increasing.
- Returns `None` iff `times.len() < 2`, or `survival_new.len() != times.len()`,
  or `survival_comparator.len() != times.len()`.
- Worked example: for `times = [0.0, 6.0, 10.0]`, `survival_new = [1.0, 0.4, 0.0]`,
  `survival_comparator = [1.0, 0.0, 0.0]` →
  `area_between_survival_curves(&times, &survival_new, &survival_comparator) == Some(2.0)`

### `qalys_from_life_extension(life_years: f64, patient_utility: f64) -> f64`

- Formula: `life_years × patient_utility`
- Total function: never returns `None`.
- Worked example: `qalys_from_life_extension(96.0, 0.7) == 67.2`

### `evlyg_from_life_extension(life_years: f64, fixed_utility: f64) -> f64`

- Formula: `life_years × fixed_utility`
- Total function: never returns `None`.
- Worked example: `evlyg_from_life_extension(96.0, EVLYG_FIXED_UTILITY) == 81.696`

### `monetary_value(health_units: f64, threshold_per_unit: f64) -> f64`

- Formula: `health_units × threshold_per_unit`
- Total function: never returns `None`. Works for any health unit (QALYs,
  evLYG, raw life-years) as long as the threshold is quoted per that unit.
- Worked example: `monetary_value(67.2, 20_000.0) == 1_344_000.0`

## Invariants

- `area_between_survival_curves` computed on the gap curve is equivalent to
  `life_years_gained_from_mean_survival(mean_survival_new, mean_survival_comparator)`
  when the two mean-survival values equal the areas under the respective
  sampled curves — the module's own worked example verifies
  `area_between_survival_curves(&times, &s_new, &s_comp) == life_years_gained_from_mean_survival(5.0, 3.0)`
  for its sample data (both equal `2.0`).
- `qalys_from_life_extension` and `evlyg_from_life_extension` share the same
  `life_years × utility` shape; the gap between their outputs for the same
  `life_years` is exactly the ethical judgment (patient utility vs fixed
  population utility) the module's rustdoc describes — neither function
  privileges the other, both are meant to be reported side by side.
