# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.0.3] - 2026-11-19

### Added

- An enforced 100% line, function, and region coverage gate and downloadable
  HTML coverage report in CI.
- A zero-missed mutation-testing gate for the branch-heavy Markov, PSA, and
  efficiency-frontier kernels, plus structural checks for all 91 specs.
- Cross-platform, MSRV, nightly, WASM, dependency, license, advisory, spelling,
  link, cognitive-complexity, and documentation-site automation.
- A searchable mdBook and Pages site, runnable benchmarks, synthetic datasets,
  a Rust notebook, four case studies, contributor templates, and expanded
  tutorial, workshop, course, and publication material.
- Project governance, security, review, release, FAQ, glossary, architecture,
  and architecture-decision documentation.

### Changed

- The local coverage command now uses `cargo-llvm-cov`, matching the coverage
  engine used for the measured threshold.
- Test, benchmark, coverage, CI, package, and MSRV entry points now work around
  incompatible injected test runners consistently.
- The health-economics agent skill now routes every implemented metric module.

### Fixed

- Corrected the README installation version and BSD-3-Clause SPDX label.
- Added missing boundary, shape, interpolation, and deterministic sampler tests
  found by mutation analysis without changing public calculation contracts.

## [1.0.2] - 2026-11-17

### Changed

- Updated release metadata and documentation for the 1.0.2 release.

[Unreleased]: https://github.com/healthprojects/health-economics-rust-crate/compare/v1.0.3...HEAD
[1.0.3]: https://github.com/healthprojects/health-economics-rust-crate/releases/tag/v1.0.3
[1.0.2]: https://github.com/healthprojects/health-economics-rust-crate/releases/tag/v1.0.2
