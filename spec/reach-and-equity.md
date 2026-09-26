# Spec: Reach and Equity (RE-AIM)

- **Module**: [`src/reach_and_equity.rs`](../src/reach_and_equity.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/reach-and-equity.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Stratum`

- `reach: f64` — fraction of the stratum's eligible population that
  participates (0–1).
- `effectiveness: f64` — real-world effect per participant (e.g. QALYs per
  participant).

### `Stratum::impact(&self) -> f64`

- Formula: `self.reach × self.effectiveness`
- Total function: never returns `None`.
- Worked example: `Stratum { reach: 0.22, effectiveness: 0.02 }.impact() == 0.0044`

### `reach(participants: f64, eligible_population: f64) -> Option<f64>`

- Formula: `participants / eligible_population`
- Returns `None` iff `eligible_population == 0.0`
- Worked example: `reach(12_000.0, 100_000.0) == Some(0.12)`

### `population_impact(reach: f64, effectiveness: f64) -> f64`

- Formula: `reach × effectiveness`
- Total function: never returns `None`.
- Worked example: `population_impact(0.12, 0.02) == 0.0024` (approximately;
  doctest checks within `1e-9`)

### `equity_gap(impact_top_group: f64, impact_bottom_group: f64) -> f64`

- Formula: `impact_top_group − impact_bottom_group`
- Total function: never returns `None`.
- Worked example: `equity_gap(0.0044, 0.0010) == 0.0034` (approximately)

### `impact_ratio(impact_top_group: f64, impact_bottom_group: f64) -> Option<f64>`

- Formula: `impact_top_group / impact_bottom_group`
- Returns `None` iff `impact_bottom_group == 0.0`
- Worked example: `impact_ratio(0.0044, 0.0010) == Some(4.4)`

### `equity_weighted_qalys(qalys: f64, equity_weight: f64) -> f64`

- Formula: `qalys × equity_weight`
- Total function: never returns `None`.
- Worked example: `equity_weighted_qalys(10.0, 1.5) == 15.0`

## Invariants

- `population_impact(reach, effectiveness)` and `Stratum::impact` compute the
  identical formula; `Stratum::impact` is the per-stratum method form used
  when reach and effectiveness are packaged together.
- `reach` must be computed against the eligible population, not registered
  users — the module's rustdoc states an unstratified/wrong-denominator
  reach is where inequity hides.
