# Spec: Number Needed to Treat (NNT)

- **Module**: [`src/number_needed_to_treat.rs`](../src/number_needed_to_treat.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/number-needed-to-treat.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `absolute_risk_reduction(control_event_rate: f64, treatment_event_rate: f64) -> f64`

- Formula: `control_event_rate − treatment_event_rate`
- Total function: never returns `None`; negative when the treatment
  increases the event rate.
- Worked example: `absolute_risk_reduction(0.032, 0.024) == 0.008`

### `relative_risk_reduction(control_event_rate: f64, treatment_event_rate: f64) -> Option<f64>`

- Formula: `ARR / control_event_rate`
- Returns `None` iff `control_event_rate == 0.0`
- Worked example: `relative_risk_reduction(0.032, 0.024) == Some(0.25)`

### `number_needed_to_treat(absolute_risk_reduction: f64) -> Option<f64>`

- Formula: `1 / ARR`
- Returns `None` iff `absolute_risk_reduction == 0.0`
- Worked example: `number_needed_to_treat(0.008) == Some(125.0)`

### `number_needed_to_harm(harm_rate_treatment: f64, harm_rate_control: f64) -> Option<f64>`

- Formula: `1 / (harm_rate_treatment − harm_rate_control)`
- Returns `None` iff `harm_rate_treatment == harm_rate_control` (excess harm
  is zero)
- Worked example: `number_needed_to_harm(0.05, 0.03) == Some(50.0)`

### `cost_per_event_prevented(nnt: f64, cost_per_treatment_course: f64) -> f64`

- Formula: `nnt × cost_per_treatment_course`
- Total function: never returns `None`.
- Worked example: `cost_per_event_prevented(125.0, 40.0) == 5_000.0`

### `prevention_payoff_ratio(cost_of_event: f64, cost_per_event_prevented: f64) -> Option<f64>`

- Formula: `cost_of_event / cost_per_event_prevented`
- Returns `None` iff `cost_per_event_prevented == 0.0`
- Worked example: `prevention_payoff_ratio(12_000.0, 5_000.0) == Some(2.4)`
