# Spec: AI Quality Metrics

- **Module**: [`src/ai_quality_metrics.rs`](../src/ai_quality_metrics.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/ai-quality-metrics.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `hallucination_rate(outputs_with_unsupported_content: f64, total_outputs: f64) -> Option<f64>`

- Formula: `outputs_with_unsupported_content / total_outputs` (fraction, 0.0–1.0)
- Returns `None` iff `total_outputs == 0.0`
- Worked example: `hallucination_rate(3.6, 100.0) == Some(0.036)`; `hallucination_rate(1.6, 100.0) == Some(0.016)`

### `faithfulness(supported_claims: f64, total_claims: f64) -> Option<f64>`

- Formula: `supported_claims / total_claims` (fraction, 0.0–1.0)
- Returns `None` iff `total_claims == 0.0`
- Worked example: `faithfulness(97.0, 100.0) == Some(0.97)`

### `ErrorType`

One error type in the economic weighting of hallucinations; contributes to
`expected_harm_cost_per_output`.

- `rate: f64` — fraction of outputs containing this error type (0.0–1.0).
- `probability_undetected: f64` — probability the human-review layer fails to catch the error (0.0–1.0).
- `probability_acted_upon: f64` — probability an undetected error is acted upon (0.0–1.0).
- `cost_per_acted_upon_error: f64` — cost per acted-upon error (currency units).

### `expected_harm_cost_per_output(error_types: &[ErrorType]) -> f64`

- Formula: `Σ over error_types of (rate * probability_undetected * probability_acted_upon * cost_per_acted_upon_error)`
- Total function: never returns `None`; returns `0.0` for an empty slice.
- Worked example: for a single `ErrorType { rate: 0.02, probability_undetected: 0.15, probability_acted_upon: 1.0, cost_per_acted_upon_error: 250.0 }`, `expected_harm_cost_per_output(&[error]) * 200_000.0 ≈ 150_000.0`

### `errors_reaching_submission(annual_volume: f64, material_error_rate: f64, human_catch_rate: f64) -> f64`

- Formula: `annual_volume * material_error_rate * (1.0 - human_catch_rate)`
- Total function: never returns `None`.
- Worked example: `errors_reaching_submission(200_000.0, 0.02, 0.85) == 600.0`

### `expected_error_cost(uncaught_errors: f64, cost_per_uncaught_error: f64) -> f64`

- Formula: `uncaught_errors * cost_per_uncaught_error`
- Total function: never returns `None`.
- Worked example: `expected_error_cost(600.0, 250.0) == 150_000.0`

### `review_cost(minutes_per_item: f64, annual_volume: f64, cost_per_reviewer_minute: f64) -> f64`

- Formula: `minutes_per_item * annual_volume * cost_per_reviewer_minute`
- Total function: never returns `None`.
- Worked example: `review_cost(2.0, 200_000.0, 0.50) == 200_000.0`

## Invariants

- `errors_reaching_submission` and `expected_harm_cost_per_output` express
  the same underlying model at two granularities: `errors_reaching_submission`
  followed by `expected_error_cost` (a single error type, `P(acted upon) = 1`)
  equals `expected_harm_cost_per_output` on the equivalent single-`ErrorType`
  slice times `annual_volume` — both worked examples land on £150,000/year
  for the same inputs.
- `expected_harm_cost_per_output` sums independently across `error_types`;
  there is no interaction term between error types.
