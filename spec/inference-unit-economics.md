# Spec: Inference Unit Economics

- **Module**: [`src/inference_unit_economics.rs`](../src/inference_unit_economics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/inference-unit-economics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `LlmCall`

One LLM call, measured in tokens.

- `input_tokens: f64` — prompt-side tokens: record context, RAG context,
  instructions.
- `output_tokens: f64` — completion-side tokens; typically priced ~4× input.

### `cost_per_call(call: &LlmCall, input_rate_per_million: f64, output_rate_per_million: f64) -> f64`

- Formula: `call.input_tokens × input_rate_per_million / 1_000_000.0 + call.output_tokens × output_rate_per_million / 1_000_000.0`
- Total function: never returns `None`.
- Worked example: for `LlmCall { input_tokens: 12_000.0, output_tokens: 1_200.0 }`,
  `cost_per_call(&draft, 3.0, 15.0) == 0.054`

### `cost_per_unit(calls: &[LlmCall], input_rate_per_million: f64, output_rate_per_million: f64) -> f64`

- Formula: `Σ cost_per_call(call, input_rate_per_million, output_rate_per_million)` over `calls`
- Total function: never returns `None`. An empty slice costs `0.0`.
- Worked example: `cost_per_unit(&[draft, verify], 3.0, 15.0) == 0.0765`
  (draft `$0.054` + verify `$0.0225`)

### `annual_cost(cost_per_unit: f64, units_per_year: f64) -> f64`

- Formula: `cost_per_unit × units_per_year`
- Total function: never returns `None`.
- Worked example: `annual_cost(0.0765, 100_000.0) == 7_650.0`

### `cost_share_of_value(cost_per_unit: f64, value_per_unit: f64) -> Option<f64>`

- Formula: `cost_per_unit / value_per_unit`
- Returns `None` iff `value_per_unit == 0.0`
- Worked example: `cost_share_of_value(0.0765, 31.25) ≈ Some(0.0025)`
  (module doctest asserts `(share - 0.0025).abs() < 0.0005`)

### `projected_cost(cost_0: f64, annual_decline_ratio: f64, years: f64) -> f64`

- Formula: `cost_0 × annual_decline_ratio.powf(years)` (geometric decay,
  `cost_t = cost_0 × d^t`)
- Total function: never returns `None`.
- Worked example: `projected_cost(1_000.0, 0.5, 1.0) == 500.0`;
  `projected_cost(1_000.0, 0.5, 2.0) == 250.0`
