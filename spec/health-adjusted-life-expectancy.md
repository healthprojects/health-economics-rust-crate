# Spec: Health-Adjusted Life Expectancy (HALE)

- **Module**: [`src/health_adjusted_life_expectancy.rs`](../src/health_adjusted_life_expectancy.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/health-adjusted-life-expectancy.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `ConditionBurden`

A condition's contribution to population health loss at some age.

- `prevalence: f64` — prevalence of the condition: fraction of the
  population affected (0–1).
- `disability_weight: f64` — disability weight for the condition: 0 = full
  health, 1 = death-like (from Global Burden of Disease data).

### `proportion_in_full_health(conditions: &[ConditionBurden]) -> f64`

- Formula: `1 − Σ (prevalence × disability_weight)` over `conditions`.
  Assumes independent, additive burdens; with high prevalences or many
  comorbid conditions the sum can exceed 1 and the result go negative.
- Total function: never returns `None`. Returns `1.0` for an empty slice.
- Worked example: `proportion_in_full_health(&[ConditionBurden { prevalence: 0.10, disability_weight: 0.32 }, ConditionBurden { prevalence: 0.20, disability_weight: 0.10 }]) == 0.948`

### `sullivan_hale(person_years: &[f64], full_health_proportion: &[f64], survivors_at_x: f64) -> Option<f64>`

- Formula: `Σ (person_years[i] × full_health_proportion[i]) / survivors_at_x`
  (Sullivan method); with every proportion at 1.0 this reduces to ordinary
  life expectancy `Σ L_a / l_x`.
- Returns `None` iff `survivors_at_x == 0.0` or `person_years.len() != full_health_proportion.len()`
- Worked example: `sullivan_hale(&[950.0, 800.0], &[0.9, 0.7], 100.0) == Some(14.15)`;
  `sullivan_hale(&[950.0, 800.0], &[1.0, 1.0], 100.0) == Some(17.5)`

### `hale_gap(life_expectancy: f64, hale: f64) -> f64`

- Formula: `life_expectancy − hale`
- Total function: never returns `None`. Positive whenever any ill health
  exists.
- Worked example: `hale_gap(73.3, 61.9) == 11.4`

### `hale_contribution_per_person(healthy_years_saved: f64, cohort_size: f64) -> Option<f64>`

- Formula: `healthy_years_saved / cohort_size`
- Returns `None` iff `cohort_size == 0.0`
- Worked example: `hale_contribution_per_person(15_000.0, 500_000.0) == Some(0.03)`

### `years_to_days(years: f64) -> f64`

- Formula: `years × 365.25`
- Total function: never returns `None`.
- Worked example: `years_to_days(0.03) ≈ 11.0` (module doctest asserts
  `(years_to_days(0.03) - 11.0).abs() < 0.5`)
