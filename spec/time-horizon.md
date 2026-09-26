# Spec: Time Horizon

- **Module**: [`src/time_horizon.rs`](../src/time_horizon.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/time-horizon.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `net_present_value(net_flows: &[f64], discount_rate: f64) -> f64`

- Formula: `Σ_t net_flows[t] / (1 + discount_rate)^t`, where `t` is the index
  into `net_flows` (year 0 first, undiscounted)
- Total function: never returns `None`; an empty slice returns `0.0`.
- Worked example: for `flows = [-2_000_000.0, 400_000.0 × 10 repeated]`,
  `net_present_value(&flows, 0.0) == 2_000_000.0`

### `net_benefit_at_horizon(implementation_cost: f64, annual_benefit: f64, annual_running_cost: f64, horizon_years: f64) -> f64`

- Formula: `-implementation_cost + horizon_years * (annual_benefit - annual_running_cost)`
- Total function: never returns `None`.
- Worked example: `net_benefit_at_horizon(2_000_000.0, 600_000.0, 200_000.0, 5.0) == 0.0`

### `break_even_horizon_years(implementation_cost: f64, annual_benefit: f64, annual_running_cost: f64) -> Option<f64>`

- Formula: `implementation_cost / (annual_benefit - annual_running_cost)`
- Returns `None` iff `annual_benefit - annual_running_cost == 0.0` (the case
  never breaks even; a negative annual net benefit yields a negative,
  meaningless horizon rather than `None`)
- Worked example: `break_even_horizon_years(2_000_000.0, 600_000.0, 200_000.0) == Some(5.0)`

### `net_benefit_by_horizons(implementation_cost: f64, annual_benefit: f64, annual_running_cost: f64, horizons_years: &[f64]) -> Vec<(f64, f64)>`

- Formula: for each `h` in `horizons_years`, `(h, net_benefit_at_horizon(implementation_cost, annual_benefit, annual_running_cost, h))`, in the input order
- Total function: never returns `None`.
- Worked example: `net_benefit_by_horizons(2_000_000.0, 600_000.0, 200_000.0, &[1.0, 3.0, 5.0, 10.0])`
  gives net benefits `[-1_600_000.0, -800_000.0, 0.0, 2_000_000.0]`

## Invariants

- `net_present_value` at `discount_rate = 0.0` reproduces the undiscounted
  figure from `net_benefit_at_horizon`/`net_benefit_by_horizons` for the same
  cash-flow pattern (year-0 outlay, then constant annual net flow).
- Discounting strictly shrinks a positive multi-year benefit stream:
  `net_present_value(flows, r) < net_present_value(flows, 0.0)` for `r > 0`
  and a stream with positive later-year flows — the module's rustdoc calls
  out "year-30 benefits at face value are fiction" as the pitfall this
  guards against.
