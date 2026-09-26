# Spec: EQ-5D

- **Module**: [`src/eq_5d.rs`](../src/eq_5d.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/eq-5d.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Eq5dProfile`

An EQ-5D-5L health-state profile: one level (1–5) in each of five
dimensions.

- `mobility: u8` — mobility level, 1–5 (1 = no problems walking about … 5 =
  unable to).
- `self_care: u8` — self-care level, 1–5 (washing and dressing).
- `usual_activities: u8` — usual activities level, 1–5 (work, study,
  housework, family, leisure).
- `pain_discomfort: u8` — pain/discomfort level, 1–5 (1 = none … 5 =
  extreme).
- `anxiety_depression: u8` — anxiety/depression level, 1–5 (1 = not
  anxious/depressed … 5 = extremely).

### `Eq5dProfile::new(mobility: u8, self_care: u8, usual_activities: u8, pain_discomfort: u8, anxiety_depression: u8) -> Option<Self>`

- Validates every level is within 1–5.
- Returns `None` iff any of the five levels is `0` or greater than `5`. This
  constructor never panics.
- Worked example: `Eq5dProfile::new(2, 1, 2, 2, 1).unwrap().code() == "21221"`;
  `Eq5dProfile::new(0, 1, 1, 1, 1).is_none()` and
  `Eq5dProfile::new(1, 1, 6, 1, 1).is_none()`

### `Eq5dProfile::code(&self) -> String`

- Formula: concatenation of the five levels in standard dimension order
  (mobility, self-care, usual activities, pain/discomfort,
  anxiety/depression).
- Total function: never returns `None`.
- Worked example: `Eq5dProfile::new(2, 1, 2, 2, 1).unwrap().code() == "21221"`

### `Eq5dProfile::is_full_health(&self) -> bool`

- Returns `true` iff all five levels equal `1` (the profile `"11111"`).
- Worked example: `Eq5dProfile::new(1, 1, 1, 1, 1).unwrap().is_full_health() == true`;
  `Eq5dProfile::new(2, 1, 2, 2, 1).unwrap().is_full_health() == false`

### `qalys(duration_years: f64, utility: f64) -> f64`

- Formula: `duration_years × utility`
- Total function: never returns `None`. Anchors: utility 1 = full health,
  0 = dead; states worse than death carry negative utility and produce
  negative QALYs.
- Worked example: `qalys(1.0, 1.0) == 1.0`

### `qaly_gain(utility_before: f64, utility_after: f64, duration_years: f64) -> f64`

- Formula: `(utility_after − utility_before) × duration_years`
- Total function: never returns `None`. Negative if utility declined.
- Worked example: `qaly_gain(0.62, 0.71, 1.0) == 0.09` (module doctest
  asserts `(gain - 0.09).abs() < 1e-9`)

### `attributable_qaly_gain(intervention_gain: f64, control_gain: f64) -> f64`

- Formula: `intervention_gain − control_gain`
- Total function: never returns `None`. Negative if controls did better.
- Worked example: `attributable_qaly_gain(0.09, 0.03) == 0.06`

### `monetized_value(qalys: f64, threshold_per_qaly: f64) -> f64`

- Formula: `qalys × threshold_per_qaly`
- Total function: never returns `None`.
- Worked example: `monetized_value(0.06, 20_000.0) == 1_200.0` and
  `monetized_value(0.06, 30_000.0) == 1_800.0`
