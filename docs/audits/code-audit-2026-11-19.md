# Code and API audit — 2026-11-19

## Scope

All 91 specifications and implementation modules, public API documentation,
unit/doctest/integration suites, dependency metadata, and automation.

## Automated evidence

- `cargo test`: unit, integration, example-target, and doctest suites pass.
- `cargo clippy --all-targets --all-features`: pedantic and cognitive-complexity
  lint policies pass.
- `cargo doc --no-deps`: missing-docs and rustdoc warnings are denied.
- `cargo llvm-cov`: 100% regions, functions, and lines.
- `cargo mutants`: 273 branch-heavy kernel mutants exercised; 259 caught by
  tests, 12 detected by timeout, 2 unviable, and 0 missed. Three documented
  behaviorally equivalent Gamma comparison mutations are excluded.
- `bin/check-spec-coverage`: all 91 specs have an implementation, crate export,
  README entry, machine-readable indexes, and agent-skill route.
- `cargo audit`: dependency advisory scan.
- `cargo deny check`: dependency source, ban, and license policy.
- `typos`: documentation and source spelling scan with domain abbreviations
  explicitly allowlisted.

## Manual findings

- There is no `unsafe`, network, filesystem, concurrency, or cryptographic API.
- Public denominator guards and shape checks are specified rather than inferred.
- Three exact-money modules retain decimal arithmetic and currency identity.
- Iterative code is limited to deterministic folds, sorting, simulation, and a
  seeded non-cryptographic Monte Carlo generator documented as such.
- The crate's flat module architecture is appropriate for independent pure
  formulas; cross-topic composition remains in tests, examples, and tutorials.
- The generated improvement audit contains generic suggestions that are not all
  applicable. In particular, async, builder, macro, feature-group, and richer
  error APIs would add complexity or break specified contracts without a user
  need.

## Residual risks

- Callers can supply nonsensical negative counts or out-of-range probabilities;
  this is a deliberate scope boundary documented in `AGENTS.md`.
- Model validity depends on the provenance and units of caller inputs.
- External links and dependency advisories change over time, so scheduled CI
  rechecks them weekly.
