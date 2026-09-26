# Spec: Flow Metrics

- **Module**: [`src/flow_metrics.rs`](../src/flow_metrics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/flow-metrics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `cycle_time(finished: f64, started: f64) -> f64`

- Formula: `finished − started`
- Total function: never returns `None`. Measures the in-progress clock only
  (the pre-work queue belongs to `lead_time`).
- Worked example: `cycle_time(10.0, 6.0) == 4.0`

### `lead_time(delivered: f64, requested: f64) -> f64`

- Formula: `delivered − requested`
- Total function: never returns `None`. Includes the pre-work queue, unlike
  `cycle_time`.
- Worked example: `lead_time(10.0, 2.0) == 8.0`

### `throughput(items_completed: f64, period: f64) -> Option<f64>`

- Formula: `items_completed / period`
- Returns `None` iff `period == 0.0`
- Worked example: `throughput(10.0, 1.0) == Some(10.0)`

### `flow_efficiency_percent(active_time: f64, wait_time: f64) -> Option<f64>`

- Formula: `active_time / (active_time + wait_time) × 100`
- Returns `None` iff `active_time + wait_time == 0.0`
- Worked example: `flow_efficiency_percent(1.0, 9.0) == Some(10.0)`

### `littles_law_cycle_time(wip: f64, throughput: f64) -> Option<f64>`

- Formula: `wip / throughput` (Little's Law solved for cycle time)
- Returns `None` iff `throughput == 0.0`
- Worked example: `littles_law_cycle_time(40.0, 10.0) == Some(4.0)`; also
  `littles_law_cycle_time(15.0, 10.0) == Some(1.5)`

### `littles_law_wip(throughput: f64, average_cycle_time: f64) -> f64`

- Formula: `throughput × average_cycle_time` (Little's Law solved for WIP)
- Total function: never returns `None`.
- Worked example: `littles_law_wip(40.0, 6.0) == 240.0`

### `delay_cost_eliminated(throughput: f64, queue_time_saved_per_item: f64, cost_of_delay_per_item_period: f64) -> f64`

- Formula: `throughput × queue_time_saved_per_item × cost_of_delay_per_item_period`
- Total function: never returns `None`.
- Worked example: `delay_cost_eliminated(10.0, 2.5, 3_000.0) == 75_000.0`
