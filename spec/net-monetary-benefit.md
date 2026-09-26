# Spec: Net Monetary Benefit (NMB)

- **Module**: [`src/net_monetary_benefit.rs`](../src/net_monetary_benefit.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/net-monetary-benefit.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `EvaluatedOption`

- `name: &'static str` — name of the option (e.g. `"App + coaching"`).
- `delta_cost: f64` — incremental cost ΔC versus the common baseline
  (currency).
- `delta_effect: f64` — incremental effect ΔE versus the common baseline
  (e.g. QALYs).

### `net_monetary_benefit(delta_effect: f64, delta_cost: f64, lambda: f64) -> f64`

- Formula: `NMB = (ΔE × λ) − ΔC`
- Total function: never returns `None`.
- Worked example: `net_monetary_benefit(30.0, 400_000.0, 20_000.0) == 200_000.0`

### `net_health_benefit(delta_effect: f64, delta_cost: f64, lambda: f64) -> Option<f64>`

- Formula: `NHB = ΔE − (ΔC / λ)`
- Returns `None` iff `lambda == 0.0`
- Worked example: `net_health_benefit(30.0, 400_000.0, 20_000.0) == Some(10.0)`

### `adopt(nmb: f64) -> bool`

- Formula: `nmb > 0.0` (decision rule: adopt if NMB is strictly positive)
- Returns a `bool`, not an `Option`; total function.
- Worked example: `adopt(200_000.0) == true`, `adopt(-60_000.0) == false`

### `best_option_index(options: &[EvaluatedOption], lambda: f64) -> Option<usize>`

- Formula: computes `net_monetary_benefit` for each option at `lambda` and
  returns the index of the highest; ties keep the earliest option (a later
  option must strictly exceed the incumbent to win).
- Returns `None` iff `options` is empty.
- Worked example: for
  `options = [("App + coaching", 400_000.0, 30.0), ("App only", 150_000.0, 12.0), ("Extra clinics", 700_000.0, 32.0)]`,
  `best_option_index(&options, 20_000.0) == Some(0)`

## Invariants

- `NMB > 0 ⇔ ICER < λ` when `ΔE > 0` — the module's rustdoc states the two
  decision rules agree (NMB is just better behaved: linear, so it can be
  averaged across Monte Carlo draws and ranks any number of options in one
  pass, unlike a pairwise ICER).
