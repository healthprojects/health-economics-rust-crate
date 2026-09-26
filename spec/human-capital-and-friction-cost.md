# Spec: Human Capital Approach vs Friction Cost Method

- **Module**: [`src/human_capital_and_friction_cost.rs`](../src/human_capital_and_friction_cost.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/human-capital-and-friction-cost.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `human_capital_cost(daily_wage: f64, days_lost: f64) -> f64`

- Formula: `daily_wage × days_lost`
- Total function: never returns `None`. Values every day of absence at the
  full wage rate for the entire duration, with no cap.
- Worked example: `human_capital_cost(150.0, 180.0) == 27_000.0`

### `friction_cost(daily_wage: f64, days_lost: f64, friction_period_days: f64) -> f64`

- Formula: `daily_wage × days_lost.min(friction_period_days)`
- Total function: never returns `None`. Caps the counted days at the
  friction period regardless of how long the actual absence runs.
- Worked example: `friction_cost(150.0, 180.0, 85.0) == 12_750.0`

## Invariants

- `friction_cost(w, d, f) <= human_capital_cost(w, d)` whenever `w >= 0.0`,
  because `d.min(f) <= d`; the module's rustdoc states FCM "produces a
  systematically lower, more conservative estimate" than HCA, and its own
  worked example asserts `fcm < hca / 2.0` for the 180-day/85-day case.
- When `days_lost <= friction_period_days`, `friction_cost` and
  `human_capital_cost` are equal (the absence is fully within the friction
  period, so nothing is capped) — demonstrated by the module's own test
  `short_absence_within_friction_period_matches_hca`.
