# Spec: QALY Shortfall and Severity Modifiers

- **Module**: [`src/qaly_shortfall_and_severity_modifiers.rs`](../src/qaly_shortfall_and_severity_modifiers.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/qaly-shortfall-and-severity-modifiers.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `absolute_shortfall(general_population_qalys: f64, qalys_with_condition: f64) -> f64`

- Formula: `general_population_qalys − qalys_with_condition`
- Total function: never returns `None`.
- Worked example: `absolute_shortfall(14.2, 2.1) == 12.1` (approximately;
  doctest checks within `1e-9`)

### `proportional_shortfall(absolute_shortfall: f64, general_population_qalys: f64) -> Option<f64>`

- Formula: `absolute_shortfall / general_population_qalys`
- Returns `None` iff `general_population_qalys == 0.0`
- Worked example: `proportional_shortfall(12.1, 14.2)` is approximately
  `Some(0.852)` (module doctest asserts `(p - 0.852).abs() < 5e-4`)

### `severity_weight(absolute_shortfall: f64, proportional_shortfall: f64) -> f64`

- Formula: `1.7` if `absolute_shortfall >= 18.0 || proportional_shortfall >= 0.95`;
  else `1.2` if `absolute_shortfall >= 12.0 || proportional_shortfall >= 0.85`;
  else `1.0`. Whichever measure gives the higher weight applies.
- Total function: never returns `None`.
- Worked example: `severity_weight(12.1, 0.852) == 1.2`;
  `severity_weight(18.0, 0.50) == 1.7`; `severity_weight(5.0, 0.95) == 1.7`;
  `severity_weight(5.0, 0.40) == 1.0`

### `effective_icer(icer: f64, severity_weight: f64) -> Option<f64>`

- Formula: `icer / severity_weight`
- Returns `None` iff `severity_weight == 0.0` (valid NICE weights are never
  zero)
- Worked example: `effective_icer(26_000.0, 1.2)` is approximately
  `Some(21_700.0)` (module doctest asserts `(e - 21_700.0).abs() < 100.0`)

### `effective_threshold(threshold: f64, severity_weight: f64) -> f64`

- Formula: `threshold × severity_weight`
- Total function: never returns `None`.
- Worked example: `effective_threshold(20_000.0, 1.2) == 24_000.0`;
  `effective_threshold(30_000.0, 1.7) == 51_000.0`

## Invariants

- `effective_icer(icer, w)` and `effective_threshold(threshold, w)` are
  algebraically equivalent views of the same weighting: weighting the QALY
  gain by `w` is equivalent to dividing the ICER by `w`, which is equivalent
  to multiplying the threshold by `w` (the module's rustdoc states this
  equivalence explicitly).
- Both shortfall measures are computed against the *current standard of
  care*, not untreated natural history, and against the age/sex-matched
  general population's remaining discounted QALYs — the module's rustdoc
  states this is a required, not optional, baseline choice.
