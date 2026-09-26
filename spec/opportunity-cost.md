# Spec: Opportunity Cost

- **Module**: [`src/opportunity_cost.rs`](../src/opportunity_cost.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/opportunity-cost.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `NHS_MARGINAL_COST_PER_QALY_GBP: f64`

Constant, `13_000.0`: the Claxton et al. (2015) estimate of what the NHS
pays for one QALY at the margin (GBP per QALY).

### `opportunity_cost(forgone_alternative_values: &[f64]) -> Option<f64>`

- Formula: the maximum of the supplied values (values may be in any
  consistent unit; negative values allowed).
- Returns `None` iff `forgone_alternative_values` is empty.
- Worked example: `opportunity_cost(&[300_000.0]) == Some(300_000.0)`;
  `opportunity_cost(&[120_000.0, 300_000.0, 90_000.0]) == Some(300_000.0)`

### `net_gain(value_chosen: f64, value_best_alternative: f64) -> f64`

- Formula: `value_chosen − value_best_alternative`
- Total function: never returns `None`.
- Worked example: `net_gain(400_000.0, 300_000.0) == 100_000.0`

### `bed_day_savings_value(bed_days_freed: f64, marginal_cost_per_bed_day: f64) -> f64`

- Formula: `bed_days_freed × marginal_cost_per_bed_day`
- Total function: never returns `None`.
- Worked example: `bed_day_savings_value(2_000.0, 150.0) == 300_000.0`

### `qalys_displaced(spend: f64, marginal_cost_per_qaly: f64) -> Option<f64>`

- Formula: `spend / marginal_cost_per_qaly`
- Returns `None` iff `marginal_cost_per_qaly == 0.0`
- Worked example: `qalys_displaced(13_000.0, NHS_MARGINAL_COST_PER_QALY_GBP) == Some(1.0)`
