# Spec: Clinical AI Evaluation

- **Module**: [`src/clinical_ai_evaluation.rs`](../src/clinical_ai_evaluation.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/clinical-ai-evaluation.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `sensitivity(true_positives: f64, false_negatives: f64) -> Option<f64>`

- Formula: `true_positives / (true_positives + false_negatives)`
- Returns `None` iff `true_positives + false_negatives == 0.0`
- Worked example: `sensitivity(90.0, 10.0) == Some(0.90)`

### `specificity(true_negatives: f64, false_positives: f64) -> Option<f64>`

- Formula: `true_negatives / (true_negatives + false_positives)`
- Returns `None` iff `true_negatives + false_positives == 0.0`
- Worked example: `specificity(93.0, 7.0) == Some(0.93)`

### `ppv_from_counts(true_positives: f64, false_positives: f64) -> Option<f64>`

- Formula: `true_positives / (true_positives + false_positives)`
- Returns `None` iff `true_positives + false_positives == 0.0`
- Worked example: `ppv_from_counts(90.0, 693.0) == Some(90.0 / 783.0)` (≈0.115)

### `npv_from_counts(true_negatives: f64, false_negatives: f64) -> Option<f64>`

- Formula: `true_negatives / (true_negatives + false_negatives)`
- Returns `None` iff `true_negatives + false_negatives == 0.0`
- Worked example: `npv_from_counts(9_207.0, 10.0) > Some(0.998)`

### `positive_rate(sensitivity: f64, specificity: f64, prevalence: f64) -> f64`

- Formula: `sensitivity * prevalence + (1.0 - specificity) * (1.0 - prevalence)`
- Total function: never returns `None`.
- Worked example: `positive_rate(0.90, 0.93, 0.01) == 0.0783`

### `ppv_from_rates(sensitivity: f64, specificity: f64, prevalence: f64) -> Option<f64>`

- Formula: `sensitivity * prevalence / positive_rate(sensitivity, specificity, prevalence)`
- Returns `None` iff `positive_rate(sensitivity, specificity, prevalence) == 0.0`
- Worked example: `ppv_from_rates(0.90, 0.93, 0.20) == Some(0.18 / 0.236)` (≈0.76); `ppv_from_rates(0.90, 0.93, 0.01) ≈ Some(0.115)`

### `npv_from_rates(sensitivity: f64, specificity: f64, prevalence: f64) -> Option<f64>`

- Formula: `specificity * (1.0 - prevalence) / (specificity * (1.0 - prevalence) + (1.0 - sensitivity) * prevalence)`
- Returns `None` iff the denominator `specificity * (1.0 - prevalence) + (1.0 - sensitivity) * prevalence == 0.0`
- Worked example: `npv_from_rates(0.90, 0.93, 0.01) > Some(0.998)`

### `number_needed_to_screen(prevalence: f64, sensitivity: f64) -> Option<f64>`

- Formula: `1.0 / (prevalence * sensitivity)`
- Returns `None` iff `prevalence * sensitivity == 0.0`
- Worked example: `number_needed_to_screen(0.01, 0.90) == Some(1.0 / 0.009)` (≈111.1)

### `cost_per_true_case_from_counts(program_cost: f64, true_positives: f64) -> Option<f64>`

- Formula: `program_cost / true_positives`
- Returns `None` iff `true_positives == 0.0`
- Worked example: `cost_per_true_case_from_counts(783.0 * 350.0, 90.0) ≈ Some(3_045.0)`

### `cost_per_true_case(sensitivity: f64, specificity: f64, prevalence: f64, workup_cost_per_positive: f64) -> Option<f64>`

- Formula: `positive_rate(sensitivity, specificity, prevalence) * workup_cost_per_positive / (sensitivity * prevalence)`
- Returns `None` iff `sensitivity * prevalence == 0.0`
- Worked example: `cost_per_true_case(0.90, 0.93, 0.01, 350.0) ≈ Some(3_045.0)`

### `auroc(positive_scores: &[f64], negative_scores: &[f64]) -> Option<f64>`

- Formula: exact pairwise computation of `P(a random positive scores above a random negative)`, with ties (within a `1e-9` tolerance) counting as 0.5; normalized by `positive_scores.len() * negative_scores.len()`.
- Returns `None` iff `positive_scores` is empty or `negative_scores` is empty.
- Worked example: `auroc(&[0.9, 0.8], &[0.2, 0.1]) == Some(1.0)`; `auroc(&[0.5, 0.5], &[0.5, 0.5]) == Some(0.5)`

## Invariants

- `ppv_from_rates` and `npv_from_rates` are Bayes-rule recomputations of
  `ppv_from_counts`/`npv_from_counts` at an arbitrary deployment
  `prevalence`, rather than at the prevalence of a specific trial's confusion
  matrix — recompute at local prevalence rather than reusing a trial-derived
  PPV/NPV.
- `cost_per_true_case` and `cost_per_true_case_from_counts` express the same
  economic bottom line at two granularities (population-rate view vs.
  program-ledger-count view); the worked example shows both landing on
  ≈£3,045 for equivalent inputs.
- `positive_rate` is the shared denominator inside both `ppv_from_rates` and
  `cost_per_true_case`.
- `auroc` is threshold-independent (a model's AUROC does not depend on any
  operating point), which is exactly why the module's rustdoc says it is
  "deployment-decision-insufficient" — two models with the same AUROC can
  have very different economics at their actual operating points
  (`sensitivity`/`specificity` pair).
