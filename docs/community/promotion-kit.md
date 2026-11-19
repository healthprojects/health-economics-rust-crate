# Promotion kit

Use these as editable drafts. Replace the bracketed release number and link;
never invent adoption claims or testimonials.

## Short release announcement

> `health-economics` [VERSION] is available: 91 spec-backed Rust modules for
> QALYs, ICER/NMB, budget impact, healthcare operations, digital health, AI
> evaluation, and engineering economics. Pure calculation APIs, runnable
> examples, zero-warning docs, and 100% line/function/region coverage.
> https://crates.io/crates/health-economics

This fits a short social post. For LinkedIn, add one paragraph describing the
specific decision problem solved by the release. For Reddit, lead with the
technical design and ask for review rather than cross-posting promotional copy.

## Awesome Rust submission draft

Under a suitable science or healthcare heading:

> [health-economics](https://github.com/healthprojects/health-economics-rust-crate)
> — Spec-driven, dependency-light calculation functions for health-economic,
> healthcare-operations, digital-health, and clinical-AI metrics.

Submitting this requires a separate pull request to another project and should
follow that project’s current contribution rules.

## Press and speaker facts

- Package: `health-economics`
- License: user’s choice of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or
  GPL-3.0-only
- Scope: 91 metric modules, each backed by a contract under `spec/`
- Design: pure synchronous calculations; `f64` for ratios and
  `rusty_money::Money` for the three exact-currency topics
- Quality gates: format, pedantic Clippy, tests, doctests, rustdoc, dependency
  policy, mutation campaigns, and 100% coverage

The logo is [`assets/logo.svg`](../../assets/logo.svg). Do not publish user
stories or organization logos without written permission.

## Community participation

Beginner work should be labelled `good first issue` only when it has a bounded
scope, accepted direction, and test expectations. Add `hacktoberfest` only in a
year when maintainers can review contributions promptly. Discussion templates
support questions, ideas, and show-and-tell without manufacturing empty issues.
