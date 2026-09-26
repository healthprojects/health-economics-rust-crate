# Spec: AI Developer Productivity

- **Module**: [`src/ai_developer_productivity.rs`](../src/ai_developer_productivity.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/ai-developer-productivity.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `acceptance_rate(accepted_suggestions: f64, shown_suggestions: f64) -> Option<f64>`

- Formula: `accepted_suggestions / shown_suggestions` (fraction, 0.0–1.0)
- Returns `None` iff `shown_suggestions == 0.0`
- Worked example: `acceptance_rate(30.0, 100.0) == Some(0.30)`

### `retention_rate(code_surviving_to_merge: f64, accepted_ai_code: f64) -> Option<f64>`

- Formula: `code_surviving_to_merge / accepted_ai_code` (fraction, 0.0–1.0)
- Returns `None` iff `accepted_ai_code == 0.0`
- Worked example: `retention_rate(88.0, 100.0) == Some(0.88)`

### `speedup(t_control: f64, t_ai: f64) -> Option<f64>`

- Formula: `(t_control - t_ai) / t_control`
- Returns `None` iff `t_control == 0.0`
- Worked example: `speedup(161.0, 71.0) ≈ Some(0.558)` (Peng et al. 2023); `speedup(100.0, 119.0) == Some(-0.19)` (METR 2025)

### `throughput_delta(merged_prs_before: f64, merged_prs_after: f64) -> Option<f64>`

- Formula: `(merged_prs_after - merged_prs_before) / merged_prs_before`
- Returns `None` iff `merged_prs_before == 0.0`
- Worked example: `throughput_delta(100.0, 106.0) == Some(0.06)`

### `annual_capacity_value(developers: f64, hours_saved_per_dev_per_day: f64, working_days_per_year: f64, loaded_hourly_rate: f64, utilization_factor: f64) -> f64`

- Formula: `developers * hours_saved_per_dev_per_day * working_days_per_year * loaded_hourly_rate * utilization_factor`
- Total function: never returns `None`.
- Worked example: `annual_capacity_value(500.0, 0.25, 220.0, 60.0, 0.6) ≈ 990_000.0`

### `annual_tool_cost(developers: f64, monthly_price_per_dev: f64) -> f64`

- Formula: `developers * monthly_price_per_dev * 12.0`
- Total function: never returns `None`.
- Worked example: `annual_tool_cost(500.0, 39.0) ≈ 234_000.0`

### `net_capacity_ratio(annual_capacity_value: f64, annual_tool_cost: f64) -> Option<f64>`

- Formula: `annual_capacity_value / annual_tool_cost`
- Returns `None` iff `annual_tool_cost == 0.0`
- Worked example: `net_capacity_ratio(990_000.0, 234_000.0) ≈ Some(4.0)`

### `perception_gap_ratio(self_reported_saving: f64, measured_saving: f64) -> Option<f64>`

- Formula: `self_reported_saving / measured_saving`
- Returns `None` iff `measured_saving == 0.0`
- Worked example: `perception_gap_ratio(45.0, 15.0) == Some(3.0)`

## Invariants

- `speedup` is signed: a positive result means the AI arm is faster; a
  negative result (as in the METR 2025 worked example) means it is slower.
  Its inputs must come from a controlled comparison, never self-report.
- `net_capacity_ratio` is meant to be computed from `annual_capacity_value`
  (which should use *measured*, not self-reported, time savings) divided by
  `annual_tool_cost`; the worked example composes these two functions'
  outputs directly.
- `perception_gap_ratio` quantifies the divergence between self-reported and
  measured savings that `annual_capacity_value` and `speedup` are meant to
  guard against by using measured figures only.
