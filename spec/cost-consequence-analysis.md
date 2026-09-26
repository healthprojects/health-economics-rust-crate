# Spec: Cost-Consequence Analysis (CCA)

- **Module**: [`src/cost_consequence_analysis.rs`](../src/cost_consequence_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-consequence-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `ConsequenceRow`

- `name: String` — what this row measures (e.g. "Nurse hours on assessments").
- `intervention: f64` — value under the intervention.
- `comparator: f64` — value under the comparator.

### `ConsequenceRow::difference(&self) -> f64`

- Formula: `self.intervention - self.comparator`
- Total function: never returns `None`.
- Worked example: for `intervention = 6_200.0, comparator = 11_800.0`, `difference() == -5_600.0`

### `CostConsequenceTable`

- `cost: ConsequenceRow` — annual (or per-period) cost row.
- `consequences: Vec<ConsequenceRow>` — quantitative outcome rows, each in natural units.

### `CostConsequenceTable::incremental_cost(&self) -> f64`

- Formula: `self.cost.difference()`
- Total function: never returns `None`.
- Worked example: for `cost = ConsequenceRow { intervention: 180_000.0, comparator: 95_000.0, .. }`, `incremental_cost() == 85_000.0`

### `cost_per_unit_gained(incremental_cost: f64, units_gained: f64) -> Option<f64>`

- Formula: `incremental_cost / units_gained`
- Returns `None` iff `units_gained == 0.0`
- Worked example: `cost_per_unit_gained(85_000.0, 5_600.0).unwrap() ≈ 15.0`; `cost_per_unit_gained(85_000.0, 0.0).is_none()`

### `value_of_avoided_events(events_avoided: f64, value_per_event: f64) -> f64`

- Formula: `events_avoided * value_per_event`
- Total function: never returns `None`.
- Worked example: `value_of_avoided_events(82.0, 1_200.0) == 98_400.0`

## Invariants

- There is deliberately no aggregation formula across rows: each
  `ConsequenceRow` keeps its own natural units, and `cost_per_unit_gained` /
  `value_of_avoided_events` are informal per-row reading aids, not a
  composite score for the table as a whole.
