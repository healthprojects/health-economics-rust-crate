# Project metrics

The project reports reproducible engineering metrics, not vanity estimates.

| Metric | Source | Update rule |
| --- | --- | --- |
| Published version and downloads | crates.io badges in `README.md` | Live |
| Test success | CI workflow | Every push and pull request |
| Line/function/region coverage | `bin/test-coverage` | Every push and pull request; all must be 100% |
| Mutation score and survivors | mutation workflow artifact | Weekly and on demand; configured gate currently has zero missed mutants |
| Dependency advisories and licenses | `cargo audit`, `cargo deny check` | Weekly and relevant pull requests |
| Documentation spelling and links | `typos`, Lychee | Weekly and documentation pull requests |
| Supported platforms | Linux, macOS, Windows, MSRV, nightly, WASM jobs | Scheduled or every relevant change |
| Benchmark observations | `bin/test-benchmarks` | Before performance-sensitive releases |
| Cognitive complexity | Clippy `cognitive_complexity` lint in `bin/ci` | Every push and pull request |

GitHub’s repository Insights remains the source for contributor, issue, and
pull-request activity; crates.io remains the source for downloads. The project
does not add third-party visitor analytics or transmit user data merely to
duplicate those sources. Release notes should record meaningful API, quality,
documentation, and performance changes rather than lines-of-code counts.

## Local collection

```sh
cargo test --all-targets --all-features
bin/test-coverage
bin/test-benchmarks
cargo tree
cargo audit
cargo deny check
```

The crate has no runtime telemetry, threads, I/O, network access, unsafe code,
or cryptographic operations. Those facts make memory, thread, internationalized
UI, and cryptographic audit surfaces intentionally empty; dependency and numeric
correctness audits remain relevant.
