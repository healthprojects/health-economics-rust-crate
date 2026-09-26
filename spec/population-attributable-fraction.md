# Spec: Population Attributable Fraction (PAF)

- **Module**: [`src/population_attributable_fraction.rs`](../src/population_attributable_fraction.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/population-attributable-fraction.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `paf_from_relative_risk(prevalence_exposed: f64, relative_risk: f64) -> Option<f64>`

- Formula: `prevalence_exposed * (relative_risk - 1.0) / (1.0 + prevalence_exposed * (relative_risk - 1.0))`
- Returns `None` iff the denominator `1 + prevalence_exposed × (relative_risk − 1)` is `0.0`.
- Worked example: `paf_from_relative_risk(0.3, 2.5) == Some(0.310_344_827_586_206_9)` (0.45 / 1.45)

### `cases_attributable(total_cases: f64, paf: f64) -> f64`

- Formula: `total_cases * paf`
- Total function: never returns `None`.
- Worked example: `cases_attributable(1_000.0, 0.310_344_827_586_206_9) ≈ 310.34`

## Invariants

- PAF is `0.0` when `prevalence_exposed == 0.0` (nobody exposed, nothing
  attributable) or when `relative_risk == 1.0` (no association) — both are
  covered by unit tests, not just the general formula.
- PAFs for different risk factors affecting the same outcome are not
  additive — this is stated as a pitfall in the module's rustdoc, not
  enforced by the function (there is no multi-factor function here).
