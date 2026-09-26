# Spec: Probabilistic Sensitivity Analysis (PSA)

- **Module**: [`src/probabilistic_sensitivity_analysis.rs`](../src/probabilistic_sensitivity_analysis.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/probabilistic-sensitivity-analysis.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `Lcg`

A seeded, deterministic 64-bit linear congruential generator (with output
mixing) local to this module. Not cryptographically secure; deterministic by
design so the same seed always reproduces the same draw sequence.

### `Lcg::new(seed: u64) -> Self`

- Formula: one LCG step applied to the seed (Knuth's MMIX constants,
  multiplier `6_364_136_223_846_793_005`, increment
  `1_442_695_040_888_963_407`), so consecutive seeds start from
  well-separated states.
- Total function: never returns `None`.
- Worked example: `Lcg::new(42)` and a second `Lcg::new(42)` produce
  identical `next_uniform()` draws.

### `Lcg::next_uniform(&mut self) -> f64`

- Formula: takes the top 53 bits of one LCG step (via `next_u64`) and scales
  by `2^-53`.
- Returns a plain `f64` in `[0, 1)`; total function (no `None`).
- Worked example: `rng.next_uniform()` for `Lcg::new(1)` lies in `[0.0, 1.0)`.

### `Lcg::uniform(&mut self, low: f64, high: f64) -> f64`

- Formula: `low + (high - low) × next_uniform()`
- Total function: never returns `None`.
- Worked example: `rng.uniform(3.0, 6.0)` for `Lcg::new(2)` lies in
  `[3.0, 6.0)`.

### `Lcg::normal(&mut self, mean: f64, sd: f64) -> f64`

- Formula: Box–Muller transform —
  `z = sqrt(-2 × ln(u1)) × cos(2π × u2)` for two independent uniforms `u1 ∈ (0,1]`
  (as `1 - next_uniform()`) and `u2 ∈ [0,1)`, then `mean + sd × z`.
- Total function: never returns `None`.
- Worked example: sampling `rng.normal(350_000.0, 150_000.0)` 10,000 times
  from `Lcg::new(3)` gives a sample mean within `10_000.0` of `350_000.0`.

### `Lcg::gamma(&mut self, shape: f64, scale: f64) -> f64`

- Formula: Marsaglia–Tsang method for `shape >= 1.0` (rejection sampling on a
  squeezed normal variate); for `shape < 1.0` uses the boost identity
  `Gamma(shape) = Gamma(shape + 1) × U^(1/shape)`.
- Total function: never returns `None`; result is `> 0` for valid
  parameters.
- Worked example: sampling `rng.gamma(16.0, 50_000.0)` 10,000 times from
  `Lcg::new(4)` gives a sample mean within `20_000.0` of `800_000.0`
  (`16 × 50_000`).

### `Lcg::gamma_mean_sd(&mut self, mean: f64, sd: f64) -> Option<f64>`

- Formula: converts to shape/scale via `shape = (mean/sd)^2`,
  `scale = sd^2/mean`, then samples `gamma(shape, scale)`.
- Returns `None` iff `mean <= 0.0` or `sd <= 0.0`
- Worked example: `rng.gamma_mean_sd(800_000.0, 200_000.0)` for
  `Lcg::new(5)` returns `Some(x)` with `x > 0.0`;
  `rng.gamma_mean_sd(0.0, 200_000.0)` returns `None`.

### `net_monetary_benefit(threshold: f64, effect: f64, cost: f64) -> f64`

- Formula: `threshold × effect − cost`
- Total function: never returns `None`.
- Worked example: `net_monetary_benefit(30_000.0, 2.0, 24_000.0) == 36_000.0`

### `ceac(option_nmb_draws: &[&[f64]]) -> Option<Vec<f64>>`

- Formula: for each joint draw index `i`, finds the option with the highest
  NMB (ties go to the earliest-listed option) and tallies a win; returns
  `wins_j / n` per option (fractions sum to 1).
- Returns `None` iff there are no options, no draws (the first option's
  slice has length 0), or the options' draw counts differ.
- Worked example: for `a = [1.0, 5.0, 3.0, 2.0]`, `b = [2.0, 1.0, 4.0, 1.0]`,
  `ceac(&[&a, &b]) == Some(vec![0.5, 0.5])`

### `mean(draws: &[f64]) -> Option<f64>`

- Formula: arithmetic mean, `Σ draws / draws.len()`
- Returns `None` iff `draws` is empty.
- Worked example: `mean(&[700_000.0, 850_000.0]) == Some(775_000.0)`

### `probability_positive(draws: &[f64]) -> Option<f64>`

- Formula: `count(draws where x > 0.0) / draws.len()`
- Returns `None` iff `draws` is empty. Draws exactly equal to zero do not
  count as positive.
- Worked example: `probability_positive(&[100.0, -50.0, 200.0, 1.0]) == Some(0.75)`

### `percentile(draws: &[f64], p: f64) -> Option<f64>`

- Formula: sorts a copy of `draws` (via `f64::total_cmp`), then linearly
  interpolates between the order statistics bracketing rank
  `(p / 100).clamp(0, 1) × (len - 1)`.
- Returns `None` iff `draws` is empty. `p` outside `[0, 100]` is clamped, not
  an error.
- Worked example: for `draws = [10.0, 20.0, 30.0, 40.0, 50.0]`,
  `percentile(&draws, 50.0) == Some(30.0)` and
  `percentile(&draws, 25.0) == Some(20.0)`

### `MigrationCase`

- `cost_mean: f64` — migration cost's Gamma mean (£; worked example
  `800_000.0`).
- `cost_sd: f64` — migration cost's Gamma standard deviation (£; worked
  example `200_000.0`).
- `benefit_mean: f64` — annual benefit's Normal mean (£/year; worked example
  `350_000.0`).
- `benefit_sd: f64` — annual benefit's Normal standard deviation (£/year;
  worked example `150_000.0`).
- `duration_low: f64` — benefit duration's Uniform lower bound (years;
  worked example `3.0`).
- `duration_high: f64` — benefit duration's Uniform upper bound (years;
  worked example `6.0`).

### `simulate_migration_net_benefits(case: &MigrationCase, n: usize, seed: u64) -> Option<Vec<f64>>`

- Formula: for each of `n` draws, samples `cost ~ Gamma(cost_mean, cost_sd)`,
  `annual ~ Normal(benefit_mean, benefit_sd)`,
  `duration ~ Uniform(duration_low, duration_high)`, and computes
  `duration × annual − cost` (discounting omitted).
- Returns `None` iff `case.cost_mean` or `case.cost_sd` is not strictly
  positive (propagated from `Lcg::gamma_mean_sd`).
- Worked example: for the worked-example `MigrationCase`,
  `simulate_migration_net_benefits(&case, 10_000, 42)` gives a mean net
  benefit within `30_000.0` of `775_000.0` and calling it twice with the
  same `case`, `n`, and `seed` produces identical `Vec<f64>` results
  (determinism).

## Invariants

- Determinism: for fixed `case`, `n`, and `seed`,
  `simulate_migration_net_benefits` always returns the identical sequence of
  draws (the module's rustdoc: "every simulation is exactly reproducible
  from its seed").
- `Lcg` is not cryptographically secure; its guarantee is reproducibility
  and adequate statistical quality for Monte Carlo at PSA scale, not
  unpredictability.
