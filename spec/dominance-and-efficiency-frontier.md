# Spec: Dominance and the Efficiency Frontier

- **Module**: [`src/dominance_and_efficiency_frontier.rs`](../src/dominance_and_efficiency_frontier.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/dominance-and-efficiency-frontier.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Alternative`

- `name: String` — human-readable label for the option.
- `cost: f64` — cost per period (e.g. £/year).
- `effect: f64` — effect in natural units (e.g. appointments recovered/year).

### `Alternative::new(name: &str, cost: f64, effect: f64) -> Self`

- Formula: constructs `Alternative { name: name.to_string(), cost, effect }`
- Total function: never returns `None`.
- Worked example: `Alternative::new("SMS reminders", 20_000.0, 2_000.0).cost == 20_000.0`

### `strictly_dominates(a: &Alternative, b: &Alternative) -> bool`

- Formula: `a.cost <= b.cost && a.effect >= b.effect && (a.cost < b.cost || a.effect > b.effect)`
- Returns a `bool`, not an `Option`: `true` iff `a` strictly dominates `b` (weak inequality on both axes, plus at least one strict inequality — identical options do not dominate each other).
- Worked example: for `sms_ai = Alternative::new("SMS + AI triage", 90_000.0, 3_500.0)` and `phone = Alternative::new("Phone calls", 120_000.0, 2_200.0)`, `strictly_dominates(&sms_ai, &phone) == true` and `strictly_dominates(&phone, &sms_ai) == false`

### `icer(next: &Alternative, prev: &Alternative) -> Option<f64>`

- Formula: `(next.cost - prev.cost) / (next.effect - prev.effect)`
- Returns `None` iff `next.effect - prev.effect == 0.0`
- Worked example: `icer(&Alternative::new("SMS reminders", 20_000.0, 2_000.0), &Alternative::new("Do nothing", 0.0, 0.0)) == Some(10.0)`

### `efficiency_frontier(options: &[Alternative]) -> Vec<Alternative>`

- Sorts `options` by increasing effect (ties broken by increasing cost), removes strictly dominated options, then repeatedly removes extended-dominated options (an interior point whose ICER over the previous frontier point exceeds the ICER of the next point over it) until frontier ICERs increase monotonically.
- Returns the frontier, ordered by increasing effect (and cost); does not mutate `options`.
- Worked example: for `options = [Alternative::new("Do nothing", 0.0, 0.0), Alternative::new("SMS reminders", 20_000.0, 2_000.0), Alternative::new("Phone calls", 120_000.0, 2_200.0), Alternative::new("SMS + AI triage", 90_000.0, 3_500.0)]`, `efficiency_frontier(&options)` yields names `["Do nothing", "SMS reminders", "SMS + AI triage"]` (Phone calls is strictly dominated)

### `frontier_icers(frontier: &[Alternative]) -> Vec<Option<f64>>`

- Formula: `frontier.windows(2).map(|w| icer(&w[1], &w[0])).collect()`
- Returns a vector of length `frontier.len() - 1` (empty for fewer than two options); an entry is `None` iff that adjacent pair's effects are equal — a proper frontier will not contain such a pair.
- Worked example: for the frontier `["Do nothing", "SMS reminders", "SMS + AI triage"]`, `frontier_icers(&frontier)` gives `[Some(10.0), Some(46.67...)]`

## Invariants

- On a valid frontier as returned by `efficiency_frontier`, the sequence
  from `frontier_icers` is strictly increasing — this is the defining
  property of "extended dominance" removal: any interior point with a
  decreasing ICER relative to its neighbors is removed by
  `efficiency_frontier` before the frontier is returned.
- `strictly_dominates` and `efficiency_frontier`'s first pass agree: an
  option is dropped from the frontier's strict-dominance pass iff some other
  option in the input `strictly_dominates` it.
