# Spec: GDS Service Metrics

- **Module**: [`src/gds_service_metrics.rs`](../src/gds_service_metrics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/gds-service-metrics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `cost_per_transaction(total_service_cost: f64, completed_transactions: f64) -> Option<f64>`

- Formula: `total_service_cost / completed_transactions` (GDS KPI 1)
- Returns `None` iff `completed_transactions == 0.0`
- Worked example: `cost_per_transaction(500_000.0, 2_000_000.0) == Some(0.25)`

### `completion_rate_percent(completed: f64, started: f64) -> Option<f64>`

- Formula: `completed / started × 100` (GDS KPI 2)
- Returns `None` iff `started == 0.0`
- Worked example: `completion_rate_percent(84.0, 100.0) == Some(84.0)`

### `digital_take_up_percent(digital_transactions: f64, all_channel_transactions: f64) -> Option<f64>`

- Formula: `digital_transactions / all_channel_transactions × 100` (GDS KPI 3)
- Returns `None` iff `all_channel_transactions == 0.0`
- Worked example: `digital_take_up_percent(1_100_000.0, 2_000_000.0) == Some(55.0)`

### `user_satisfaction_percent(satisfied_or_very_satisfied: f64, respondents: f64) -> Option<f64>`

- Formula: `satisfied_or_very_satisfied / respondents × 100` (GDS KPI 4)
- Returns `None` iff `respondents == 0.0`
- Worked example: `user_satisfaction_percent(80.0, 100.0) == Some(80.0)`

### `channel_shift_saving(volume: f64, take_up_shift: f64, cost_old_channel: f64, cost_digital: f64) -> f64`

- Formula: `volume × take_up_shift × (cost_old_channel − cost_digital)`
- Total function: never returns `None`.
- Worked example: `channel_shift_saving(2_000_000.0, 0.25, 3.20, 0.25) == 1_475_000.0`

### `failure_demand_cost(volume: f64, digital_share: f64, completion_rate: f64, fallback_channel_cost: f64) -> f64`

- Formula: `volume × digital_share × (1.0 − completion_rate) × fallback_channel_cost`
- Total function: never returns `None`.
- Worked example: `failure_demand_cost(2_000_000.0, 0.30, 0.84, 3.20) == 307_200.0`

## Invariants

- The four GDS KPI functions (`cost_per_transaction`,
  `completion_rate_percent`, `digital_take_up_percent`,
  `user_satisfaction_percent`) are, per the module's rustdoc, "one economic
  model, not four dashboards": `channel_shift_saving` and
  `failure_demand_cost` compose the take-up and completion-rate KPIs
  together — savings only materialize when completion rate and take-up
  improve together, since failure demand from incomplete digital journeys
  falls back to the expensive channel and erodes the channel-shift saving.
