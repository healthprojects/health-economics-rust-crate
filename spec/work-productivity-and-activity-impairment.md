# Spec: Work Productivity and Activity Impairment (WPAI)

- **Module**: [`src/work_productivity_and_activity_impairment.rs`](../src/work_productivity_and_activity_impairment.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/work-productivity-and-activity-impairment.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `absenteeism_percent(hours_missed: f64, hours_worked: f64) -> Option<f64>`

- Formula: `hours_missed / (hours_missed + hours_worked) * 100`
- Returns `None` iff `hours_missed + hours_worked == 0.0`
- Worked example: `absenteeism_percent(4.0, 36.0) == Some(10.0)`

### `overall_work_impairment_percent(absenteeism_percent: f64, presenteeism_percent: f64) -> f64`

- Formula: `absenteeism_percent + (1 - absenteeism_percent / 100) * presenteeism_percent`
- Total function: never returns `None`. `presenteeism_percent` is elicited
  directly via questionnaire (0–10 self-rated impairment × 10); this
  function does not derive it.
- Worked example: `overall_work_impairment_percent(10.0, 30.0) == 37.0`

### `productivity_cost(overall_work_impairment_percent: f64, period_earnings: f64) -> f64`

- Formula: `overall_work_impairment_percent / 100 * period_earnings`
- Total function: never returns `None`.
- Worked example: `productivity_cost(37.0, 800.0) == 296.0`

## Invariants

- `overall_work_impairment_percent` can never exceed 100%: applying
  presenteeism only to the remaining (non-absent) share of work time — rather
  than simply adding the two percentages — guarantees the total is bounded
  by 100, which the module's tests verify directly (e.g.
  `overall_work_impairment_percent(80.0, 100.0) <= 100.0`).
- The three functions compose end-to-end in the worked example:
  `absenteeism_percent(4.0, 36.0)` feeds `overall_work_impairment_percent`
  (with a directly-elicited presenteeism value), which feeds
  `productivity_cost`.
