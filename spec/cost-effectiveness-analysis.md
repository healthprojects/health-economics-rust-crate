# Spec: Cost-Effectiveness Analysis (CEA)

- **Module**: [`src/cost_effectiveness_analysis.rs`](../src/cost_effectiveness_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/cost-effectiveness-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `InterventionOption`

- `name: String` — option name (e.g. "Pharmacy screening events").
- `cost: f64` — total cost from the declared perspective over the declared horizon.
- `effect: f64` — outcome in natural units (e.g. cases found).

### `icer(cost_a: f64, effect_a: f64, cost_b: f64, effect_b: f64) -> Option<f64>`

- Formula: `(cost_a - cost_b) / (effect_a - effect_b)`
- Returns `None` iff `effect_a - effect_b == 0.0`
- Worked example: `icer(400_000.0, 520.0, 150_000.0, 300.0).unwrap() ≈ 1_136.0`; `icer(400_000.0, 300.0, 150_000.0, 300.0).is_none()`

### `average_cost_effectiveness_ratio(cost: f64, effect: f64) -> Option<f64>`

- Formula: `cost / effect`
- Returns `None` iff `effect == 0.0`
- Worked example: `average_cost_effectiveness_ratio(900_000.0, 610.0).unwrap() ≈ 1_475.0`; `average_cost_effectiveness_ratio(100.0, 0.0).is_none()`

### `incremental_icers(options_sorted_by_effect: &[InterventionOption]) -> Vec<Option<f64>>`

- Formula: for each adjacent pair `(pair[0], pair[1])` in `options_sorted_by_effect.windows(2)`, `icer(pair[1].cost, pair[1].effect, pair[0].cost, pair[0].effect)`
- Returns one entry per adjacent pair; a slot is `None` iff that pair's effects are equal. The vector is empty for fewer than two options.
- Worked example: for the three-option worked example (pulse checks £150k/300, pharmacy £400k/520, wearable £900k/610), `incremental_icers(&options)[0].unwrap() ≈ 1_136.0` and `[1].unwrap() ≈ 5_556.0`

## Invariants

- The wearable option's average ratio (`average_cost_effectiveness_ratio`)
  is lower than its incremental ratio versus pharmacy (`icer`) in the worked
  example — the module's rustdoc states the average is "flattering" and the
  incremental figure is "the honest number for an expansion decision," but
  this ordering is a property of the worked example's numbers, not a general
  inequality the functions enforce.
- `incremental_icers` assumes its input is already sorted by increasing
  effect with dominated options eliminated (see
  `dominance_and_efficiency_frontier`); it does not sort or filter its input.
