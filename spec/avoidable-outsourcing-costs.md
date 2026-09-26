# Spec: Avoidable Outsourcing Costs

- **Module**: [`src/avoidable_outsourcing_costs.rs`](../src/avoidable_outsourcing_costs.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/avoidable-outsourcing-costs.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `avoidable_outsourcing_saving(cases_moved_in_house: f64, outsourced_unit_price: f64, internal_marginal_cost_per_case: f64) -> f64`

- Formula: `cases_moved_in_house * (outsourced_unit_price - internal_marginal_cost_per_case)`
- Total function: never returns `None`. Negative if internal marginal cost exceeds the external price.
- Worked example: `avoidable_outsourcing_saving(500.0, 900.0, 350.0) == 275_000.0`

### `outsourcing_spend(cases_outsourced: f64, outsourced_unit_price: f64) -> f64`

- Formula: `cases_outsourced * outsourced_unit_price`
- Total function: never returns `None`.
- Worked example: `outsourcing_spend(800.0, 900.0) == 720_000.0`

### `outsourcing_premium_ratio(outsourced_unit_price: f64, internal_scheme_price: f64) -> Option<f64>`

- Formula: `outsourced_unit_price / internal_scheme_price`
- Returns `None` iff `internal_scheme_price == 0.0`
- Worked example: `outsourcing_premium_ratio(900.0, 750.0) == Some(1.2)`

### `net_benefit(avoidable_outsourcing_saving: f64, software_annual_cost: f64) -> f64`

- Formula: `avoidable_outsourcing_saving - software_annual_cost`
- Total function: never returns `None`. Negative if the software costs more than it saves.
- Worked example: `net_benefit(275_000.0, 90_000.0) == 185_000.0`

## Invariants

- `avoidable_outsourcing_saving` uses *marginal* internal cost per case, not
  average cost — comparing against average cost understates the saving
  because the fixed estate cost runs either way.
- `net_benefit` is meant to be computed from the output of
  `avoidable_outsourcing_saving` minus a software cost, as in the worked
  example (£275,000 saving − £90,000 software cost = £185,000 net).
- The saving claimed by `avoidable_outsourcing_saving` is only realizable if
  the released internal capacity (theatre sessions, beds, staff) actually
  exists to absorb the repatriated activity — the binding constraint governs,
  per the module's "Why it matters" and pitfalls sections (not encoded as a
  runtime check by the function itself).
