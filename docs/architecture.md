# Architecture

The crate is a flat library of independent topic modules. Each module has four
representations of the same contract:

```text
spec/<topic>.md
      |
      +--> src/<topic>.rs (implementation, rustdoc, unit tests)
      +--> tests/         (cross-module and edge-case tests)
      +--> llms.txt / llms.json / health-economics-skill
```

Most functions are pure `f64` calculations. Undefined ratios use `Option`.
Three exact-currency modules use `rusty_money::Money` and `Result`. There is no
I/O layer, persistence, global state, runtime, or service boundary.

`src/lib.rs` is the public module index. Examples and tutorials compose modules
into decision pathways; modules do not depend on one another unless composition
is intrinsic to the topic. This keeps formulas independently testable and makes
cross-module equivalence tests explicit.

See the decisions in [`adr/`](adr/) and the contributor workflow in
[`../AGENTS.md`](../AGENTS.md).
