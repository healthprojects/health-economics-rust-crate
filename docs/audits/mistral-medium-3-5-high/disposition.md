# Backlog disposition — 2026-11-19

The generated audit is broad advisory input. This disposition records every
category so useful work is implemented without silently changing the crate’s
specified API, adding unapproved dependencies, inventing community claims, or
changing external accounts.

See [`task-status.md`](task-status.md) for an item-by-item trace of every named
recommendation.

## Implemented repository work

- **Promotion and onboarding:** crates.io/download/CI/MSRV/license/coverage
  badges, an accessible SVG logo, corrected installation guidance, security
  policy, roadmap, issue/PR/discussion templates, GitHub Pages workflow,
  release-post/press/speaker drafts, and explicit license choice wording.
- **Learning:** mdBook navigation and searchable rustdoc deployment, getting
  started guide, FAQ, glossary, architecture and three ADRs, comparison and
  migration guides, data-flow diagram, cheat sheet, four case studies, six
  runnable examples, synthetic datasets, and benchmark results. A Rust Jupyter
  notebook, workshop, eight-session course, blog series, talk, video, narration,
  and podcast outlines provide versionable publishing source.
- **Engineering:** Rust 2024 (already in use), changelog, `justfile`, pre-commit
  configuration, nightly check, proven WASM compile check, release guide, and
  dependency-free benchmark harness.
- **Testing:** unit tests, doctests, eight cross-module integration suites,
  deterministic invariant/property sweeps, extreme-input stress cases, exact
  worked-example regression values, a zero-survivor cargo-mutants gate over 273
  branch-heavy kernel mutations, structural coverage of all 91 specs, and a CI
  gate requiring 100% line, function, and region coverage.
- **Audit and compliance:** pedantic Clippy, denied rustdoc warnings, code/API,
  architecture, dependency, accessibility, internationalization, performance,
  coverage, memory/thread/crypto-surface, and security-checklist reviews;
  weekly `cargo audit`, `cargo deny`, spelling, and link validation.
- **Metrics:** reproducible project-metric sources, live crates.io metrics,
  GitHub Insights as the activity source, benchmark procedure, mutation
  artifacts, and platform/MSRV/nightly/WASM checks.
- **AI:** every metric is routed from `health-economics-skill/SKILL.md`; GitHub
  coding-agent instructions require spec-first changes, deterministic tests,
  and all quality gates. The existing migration guide, examples, and rustdoc
  are the grounded sources for generated help rather than a second API schema.
- **Infrastructure:** MSRV policy and matrix, issue/PR templates, reviewed code
  of conduct and license, governance/triage/maintainer/release guidance,
  dependency updates, and repository-continuity guidance.

## Satisfied by the existing design

- Snapshot, golden-master, contract, and documentation validation are supplied
  more directly by executable spec worked examples, unit tests, integration
  tests, doctests, and `missing_docs = "deny"`.
- Performance regression observations use the dependency-free benchmark
  harness. A universal timing failure threshold would be flaky on shared CI.
- Module dependency analysis is the architecture/data-flow documentation plus
  `cargo tree`; most metric modules are intentionally independent.
- `#[non_exhaustive]` is not appropriate for closed contract types and would
  restrict downstream construction or matching. Const generics, builders,
  macros, custom derives, and async do not improve pure scalar/slice formulas.
- Replacing specified `Option` results with custom errors would violate the
  source-of-truth contracts. The Money APIs already return `MoneyError` where
  construction can fail.
- Per-module feature flags, `no_std`, serialization, FFI, and Python bindings
  have no demonstrated consumer requirement. Several add dependencies or
  widen the public support surface, so the maintainer must approve a concrete
  design before implementation.
- Third-party analytics, AI reviewers/scanners/translators, and artificial
  testimonials are not enabled: they introduce data, permissions, cost, or
  accuracy concerns while existing CI and source documentation cover the
  repository-local need.
- Binary audio, podcast, video, PDF, mobile, and playground deliverables require
  a publishing platform or new toolchain. The repository contains an executable
  notebook and versionable tutorial, workshop, course, blog, talk, video,
  narration, podcast, example, and case-study source material instead.

## Owner or external action still required

- A funding configuration needs the owner’s intended sponsorship platform and
  account; none is inferred from package metadata.
- Submitting to Awesome Rust and publishing social posts require changes to
  third-party repositories/accounts; drafts are in the promotion kit.
- GitHub currently has the standard `good first issue` label and no open
  issues. No fake beginner issue, milestone, or project card was created.
  Hacktoberfest labels should be added only for a year with review capacity.
- Discussion templates are present, but repository Discussions are currently
  disabled. Pages, Discussions, repository topics, analytics settings,
  milestones, and project boards are owner-level settings rather than source
  changes.
- Literal `proptest` and `cargo-fuzz` harnesses require new dependencies, which
  `AGENTS.md` forbids adding without prior approval. Existing deterministic
  property/stress coverage remains dependency-free.

These items are explicit decisions or external publication steps, not hidden
code debt. Revisit them when the owner supplies the missing account details,
dependency approval, audience, and release milestone.
