# AGENTS.md

Instructions for any AI coding agent working in this repository. See
[`CLAUDE.md`](CLAUDE.md) for Claude Code-specific supplements.

## What this crate is

`health-economics` is a Rust crate of pure calculation functions for
health-economics metrics — QALYs, ICER/NMB, cost-effectiveness/cost-benefit
analysis, budget impact, healthcare operations, digital health products, AI
evaluation, and their software-engineering analogues (cost of delay, DORA,
technical debt as principal-and-interest, and so on). One module per metric
topic in `src/`, each mirroring a topic in the upstream [Health Economics
Metrics](https://github.com/healthprojects/health-economics-metrics) project
(see the `Topic doc:` line at the end of each module's rustdoc, and each
metric's `Upstream topic` line under [`spec/`](spec/README.md)).

Almost every function is `f64` in, `f64`/`Option<f64>` out. Three modules —
[`exact_cents_cost_allocation`](src/exact_cents_cost_allocation.rs),
[`currency_safe_cost_rollup`](src/currency_safe_cost_rollup.rs), and
[`cross_currency_icer_comparison`](src/cross_currency_icer_comparison.rs) —
instead use the [`rusty-money`](https://docs.rs/rusty-money) crate's `Money`
type directly (never wrapped in a local alias — use
`rusty_money::Money<'static, rusty_money::iso::Currency>` by name in
signatures) for exact-decimal currency arithmetic, because summing or
allocating money in binary floating point is a real correctness bug class
(see those modules' rustdoc). `rusty-money` and its `rust_decimal` dependency
are this crate's only dependencies — do not add another one without asking
first.

## Build, test, lint

```sh
cargo test                                 # unit tests + doctests
cargo clippy --all-targets --all-features  # must be zero warnings — see below
cargo doc --no-deps                        # must be zero warnings
```

Run all three before considering any change to `src/` complete. `Cargo.toml`
sets `[lints.rust] missing_docs = "deny"` and `[lints.clippy] pedantic =
"deny"` — a clippy-pedantic or missing-docs violation fails the build, not
just a lint pass. If a pedantic lint is a false positive for a specific line,
scope an `#[allow(...)]` to that item with a one-line comment explaining why,
rather than weakening the crate-wide lint level.

## Spec-driven workflow

[`spec/`](spec/README.md) holds the calculation contract (formula, function
signature, `None`/`Err` conditions) for every metric — the single source of
truth for "what the function computes," independent of the prose explaining
why it matters. A change to a formula, signature, or `None`/`Err` condition
must update, together, in the same change:

1. The spec file under `spec/`.
2. The function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`, and
   `# Errors` for the three `Money`-based modules).
4. Its unit test(s), and the module doctest if the worked example changed.

A narrative-only edit (why a metric matters, its pitfalls, its data sources)
touches only the module's rustdoc and doesn't require touching `spec/`.

## Adding a new metric module

1. Add `spec/<topic-name>.md` (kebab-case), following the format of an
   existing spec file: `Module`, `Status`, `Upstream topic`, then a
   `## Contract` section per function with formula, `None`/`Err` condition,
   and at least one worked example.
2. Add `src/<topic_name>.rs` (snake_case), following the shape every
   existing module uses:
   - Module-level rustdoc (`//!`) with `# <Title>`, `## Formula`, `## Why
     it matters`, `## Example` (as a runnable doctest), `## Software
     engineering connection`, `## Pitfalls`, `## Sources`, and a trailing
     `Topic doc:` line pointing at the upstream path.
   - Each `pub fn` takes and returns `f64`, uses `Option<f64>` if any
     argument is a denominator that can be zero, carries `#[must_use]`
     (skip it on a function returning `Result`, which is already inherently
     must-use), and has rustdoc with `# Arguments`, `# Returns`, and a
     doctest under `# Examples` that reproduces the module's worked
     example. If the metric is genuinely about money rather than a plain
     ratio, use `rusty_money::Money` directly (see the three existing
     `Money`-based modules for the pattern) instead of `f64`.
   - A `#[cfg(test)] mod tests` block with one test per worked-example
     value (comment the test with the doc line it reproduces) and one
     `zero_denominator_returns_none` test. Compare floats with
     `assert!((x - y).abs() < TOL)` (a `const TOL: f64 = 1e-9;` at the top
     of the test module), never `assert_eq!` — `Money` values compare
     exactly with `assert_eq!` since they're decimal-backed, not float.
3. Add `pub mod <topic_name>;` to `src/lib.rs`, and an entry in its `##
   Module index by theme` doc section under the correct theme heading.
4. Update `README.md`'s module list, `llms.txt`, `llms.json`, and
   `health-economics-skill/SKILL.md` to include the new module.

## Code conventions

- No panics in any public function. Prefer a total order (`f64::total_cmp`)
  over `partial_cmp().unwrap()` when sorting floats, so `NaN` input can't
  panic — see `probabilistic_sensitivity_analysis::percentile`.
- No input validation beyond the zero-denominator check. This crate does not
  verify that a numerator is non-negative or `<=` its denominator; it trusts
  the caller's counts. Do not add that validation without discussing it
  first — it's a deliberate scope boundary, not a gap.
- Comments in function bodies are rare on purpose: most functions are a
  one-line formula that the doctest above it already demonstrates. Add a
  body comment only for genuinely non-obvious logic (e.g. the interpolation
  arithmetic in a percentile function, or a Monte Carlo sampler), not to
  restate the formula.
- Every `#[allow(clippy::...)]` needs a comment explaining why the lint
  doesn't apply, not just that it was silenced — see the `cast_precision_loss`
  allows in `probabilistic_sensitivity_analysis.rs` for the pattern.
- Money math never goes through a local wrapper type — use
  `rusty_money::Money`/`rusty_money::iso`/`rusty_money::MoneyError` directly
  in public signatures, so the type a caller needs to construct is exactly
  the type the function signature names.

## Things not to do without asking first

- Add a dependency beyond `rusty-money`/`rust_decimal` (already added
  deliberately for the three `Money`-based modules; anything further needs
  its own justification).
- Change the `license` field in `Cargo.toml` or add a new license option.
- Change the repository URL (canonical: the `origin` git remote —
  currently `https://github.com/healthprojects/health-economics-rust-crate/`;
  keep `Cargo.toml`, `CITATION.cff`, and `README.md` in agreement with it).
- Scaffold or modify a `*.github.io` / SvelteKit / Lily Design System site —
  that's a separate repository, out of scope here.
