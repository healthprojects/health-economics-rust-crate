# Spec: Analysis Perspective

- **Module**: [`src/analysis_perspective.rs`](../src/analysis_perspective.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/analysis-perspective.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Perspective`

Whose costs and benefits count in the analysis. An enum with variants:

- `Payer` — e.g. NHS commissioner, insurer: only costs the payer reimburses.
- `Provider` — e.g. a hospital trust: internal delivery costs, staffing, estates.
- `Societal` — everything: patient time, travel, informal care by family, and productivity losses to employers.

### `ImpactItem`

One row of an impact inventory: a signed amount plus flags for which
perspectives it counts from.

- `amount: f64` — signed value of the item (currency units; positive benefit, negative cost).
- `counts_for_payer: bool` — whether the item counts from the payer perspective.
- `counts_for_provider: bool` — whether the item counts from the provider perspective.
- `counts_for_societal: bool` — whether the item counts from the societal perspective.

### `ImpactItem::included_in(&self, perspective: Perspective) -> bool`

- Formula: returns `self.counts_for_payer`, `self.counts_for_provider`, or `self.counts_for_societal`, selected by matching `perspective`.
- Total function: never returns `None`.
- Worked example: for `ImpactItem { amount: 300_000.0, counts_for_payer: false, counts_for_provider: false, counts_for_societal: true }`, `patient_time.included_in(Perspective::Payer) == false` and `patient_time.included_in(Perspective::Societal) == true`

### `net_value_from_perspective(items: &[ImpactItem], perspective: Perspective) -> f64`

- Formula: `sum over items where item.included_in(perspective) of item.amount`
- Total function: never returns `None`; returns `0.0` if no items count from that perspective.
- Worked example: for the three-item worked-example inventory (payer savings +£420,000 counted for payer and societal; patient time +£300,000 counted for societal only; harm -£600,000 counted for societal only), `net_value_from_perspective(&items, Perspective::Payer) == 420_000.0` and `net_value_from_perspective(&items, Perspective::Societal) == 120_000.0`

### `payer_savings(visits_diverted: f64, cost_per_visit: f64) -> f64`

- Formula: `visits_diverted * cost_per_visit`
- Total function: never returns `None`.
- Worked example: `payer_savings(10_000.0, 42.0) == 420_000.0`

### `patient_time_value(visits: f64, hours_per_visit: f64, value_per_hour: f64) -> f64`

- Formula: `visits * hours_per_visit * value_per_hour`
- Total function: never returns `None`.
- Worked example: `patient_time_value(10_000.0, 2.0, 15.0) == 300_000.0`

### `false_reassurance_harm(visits: f64, false_reassurance_rate: f64, extra_treatment_cost_per_case: f64) -> f64`

- Formula: `visits * false_reassurance_rate * extra_treatment_cost_per_case`
- Total function: never returns `None`. Returns a positive harm cost; negate it before using as an `ImpactItem.amount`.
- Worked example: `false_reassurance_harm(10_000.0, 0.02, 3_000.0) == 600_000.0`

## Invariants

- `net_value_from_perspective` composed across all three `Perspective`
  variants on the same `items` inventory can give three different totals for
  the same underlying activity ("same app, three different answers"), per
  the module's worked example (£420,000 payer vs £120,000 societal vs £0
  provider).
- `false_reassurance_harm` returns a positive cost figure; callers must negate
  it (`-false_reassurance_harm(...)`) when constructing the corresponding
  `ImpactItem.amount`, since `ImpactItem.amount` is signed (positive =
  benefit, negative = cost).
- Marking an `ImpactItem` as counting for only one perspective (rather than
  multiple) is how double counting is avoided when perspectives are compared
  side by side.
