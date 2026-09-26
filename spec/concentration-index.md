# Spec: Concentration Index

- **Module**: [`src/concentration_index.rs`](../src/concentration_index.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/concentration-index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `concentration_index(health_values: &[f64], socioeconomic_ranks: &[f64]) -> Option<f64>`

- Formula: `(2.0 / mean(health_values)) * covariance(health_values, socioeconomic_ranks)`, where `covariance(X, Y) = mean(X * Y) - mean(X) * mean(Y)` (population covariance).
- Returns `None` iff `health_values.len() != socioeconomic_ranks.len()`, either slice is empty, or `mean(health_values) == 0.0`.
- Worked example: for `health_values = [1.0, 2.0, 3.0, 4.0]` and `socioeconomic_ranks = [0.125, 0.375, 0.625, 0.875]`, `concentration_index(&health_values, &socioeconomic_ranks) == Some(0.25)`.

## Invariants

- Reversing which end of the health scale counts as "good" (i.e. reversing
  `health_values` while holding `socioeconomic_ranks` fixed) exactly flips
  the sign of the result — verified by a dedicated unit test
  (`concentration_index(reversed) == -concentration_index(original)`).
- A perfectly flat health variable (identical value for every person,
  regardless of rank) has covariance zero and therefore `concentration_index
  == 0.0`, as long as the mean is non-zero.
