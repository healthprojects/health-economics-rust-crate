# Getting started

## Add the crate

```toml
[dependencies]
health-economics = "1"
```

Choose a module by the decision you need to make, not merely by the input data
you happen to have. The README groups all modules by theme and the glossary
expands common abbreviations.

## Calculate a defined metric

```rust
use health_economics::incremental_cost_effectiveness_ratio::{
    adopt_at_threshold, classify_quadrant, icer,
};

let delta_cost = 500_000.0;
let delta_qalys = 50.0;
let quadrant = classify_quadrant(delta_cost, delta_qalys);
let ratio = icer(delta_cost, delta_qalys).expect("effect is non-zero");

assert!(matches!(
    quadrant,
    health_economics::incremental_cost_effectiveness_ratio::CostEffectivenessQuadrant::TradeOff
));
assert!(adopt_at_threshold(ratio, 20_000.0));
```

Every ratio whose denominator may be zero returns `Option<f64>`. Handle `None`
as undefined, not as zero. Keep units consistent and label the analysis
perspective, comparator, and time horizon.

## Explore and verify

```sh
cargo doc --open
cargo run --example qaly_to_decision
cargo test
```

The exact calculation contract lives under `spec/`; narrative guidance and
sources live in module rustdoc. For exact-cents allocation or rollup, use the
three `rusty_money::Money` modules instead of `f64`.

## Contribute

Read `AGENTS.md` and `CONTRIBUTING.md`, make the spec and implementation move
together, then run `bin/ci` and `bin/coverage`.
