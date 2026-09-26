# Spec: Multi-Criteria Decision Analysis (MCDA)

- **Module**: [`src/multi_criteria_decision_analysis.rs`](../src/multi_criteria_decision_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/multi-criteria-decision-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `WeightedCriterion`

- `weight: f64` — the criterion's relative importance, elicited from
  stakeholders; weights across all criteria in a scoring exercise should sum
  to 1 (see `weights_sum_to_one`).
- `score: f64` — the option's normalized performance on this criterion,
  typically in the range 0–1 (0 = worst, 1 = best).

### `mcda_score(criteria: &[WeightedCriterion]) -> f64`

- Formula: `Σ_i (weight_i × score_i)`
- Total function: never returns `None`; returns `0.0` for an empty slice.
- Worked example: for
  `criteria = [(0.4, 0.8), (0.3, 0.5), (0.2, 0.9), (0.1, 0.6)]`,
  `mcda_score(&criteria) == 0.71`

### `weights_sum_to_one(criteria: &[WeightedCriterion]) -> bool`

- Formula: `|Σ_i weight_i − 1.0| < 1e-9`
- Returns a `bool`, not an `Option`; total function.
- Worked example: for
  `criteria = [(0.4, 0.8), (0.3, 0.5), (0.2, 0.9), (0.1, 0.6)]`,
  `weights_sum_to_one(&criteria) == true`
