# Spec: Engagement Metrics

- **Module**: [`src/engagement_metrics.rs`](../src/engagement_metrics.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/engagement-metrics.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `stickiness_percent(daily_active_users: f64, monthly_active_users: f64) -> Option<f64>`

- Formula: `daily_active_users / monthly_active_users × 100`
- Returns `None` iff `monthly_active_users == 0.0`
- Worked example: `stickiness_percent(4_000.0, 20_000.0) == Some(20.0)`

### `sessions_per_user(sessions: f64, users: f64) -> Option<f64>`

- Formula: `sessions / users`
- Returns `None` iff `users == 0.0`
- Worked example: `sessions_per_user(80_000.0, 20_000.0) == Some(4.0)`

### `average_session_duration(total_time: f64, sessions: f64) -> Option<f64>`

- Formula: `total_time / sessions`
- Returns `None` iff `sessions == 0.0`
- Worked example: `average_session_duration(240_000.0, 80_000.0) == Some(3.0)`

### `feature_engagement(users_performing_key_action: f64, active_users: f64) -> Option<f64>`

- Formula: `users_performing_key_action / active_users`
- Returns `None` iff `active_users == 0.0`
- Worked example: `feature_engagement(7_000.0, 20_000.0) == Some(0.35)`

### `effective_dose_share(effective_dose_users: f64, registered_users: f64) -> Option<f64>`

- Formula: `effective_dose_users / registered_users`
- Returns `None` iff `registered_users == 0.0`
- Worked example: `effective_dose_share(7_000.0, 50_000.0) == Some(0.14)`

### `population_effect(trial_effect: f64, effective_dose_share: f64) -> f64`

- Formula: `trial_effect × effective_dose_share`
- Total function: never returns `None`. Gives zero credit for sub-threshold
  users.
- Worked example: `population_effect(6.0, 0.14) == 0.84`

### `overstatement_factor(registered_users: f64, effective_dose_users: f64) -> Option<f64>`

- Formula: `registered_users / effective_dose_users`
- Returns `None` iff `effective_dose_users == 0.0`
- Worked example: `overstatement_factor(50_000.0, 7_000.0) ≈ Some(7.14)` (the
  module's doctest asserts `(factor - 7.14).abs() < 0.01`)

## Invariants

- `overstatement_factor` and `effective_dose_share` are reciprocals scaled
  by `registered_users`; both are guarded on the same kind of zero
  denominator but over different variables (`effective_dose_users` vs
  `registered_users`) — callers must not conflate the two guard conditions.
