# Dependency graph

The production dependency graph is intentionally small and is reproducible with
`cargo tree --depth 3`.

```mermaid
flowchart TD
    H[health-economics] --> D[rust_decimal 1.43]
    H --> M[rusty-money 0.5]
    M --> D
    M --> A[arrayvec 0.7]
    D --> A
    D --> N[num-traits 0.2]
    N -. build only .-> C[autocfg 1.5]
```

`rusty-money` supplies public exact-currency values and delegates decimal
storage to `rust_decimal`; the crate also names `rust_decimal` directly to
construct and round exact amounts. All other metric modules use `std`. Test,
documentation, coverage, audit, and mutation tools are installed tools rather
than Cargo dependencies.

The lockfile, `cargo deny check`, and `cargo audit` are the authoritative inputs
for version, license, source, duplication, and advisory review. Do not infer
security or maintenance quality from graph size alone.
