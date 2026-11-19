# Video tutorial outline

## Health economics in Rust (20 minutes)

1. State the decision: whether a waiting-list intervention is worth funding.
2. Install `health-economics = "1"` and show how specs map to modules.
3. Calculate QALYs recovered from a shorter wait.
4. Calculate ICER and net monetary benefit and explain their agreement.
5. Vary uncertain inputs, then show PSA and EVPI.
6. Distinguish economic value from cash-releasing budget impact.
7. Run `cargo test`, `cargo doc --open`, and one repository example.

Use the worked values in `examples/qaly_to_decision.rs`; captions should spell
out acronyms and narrators should describe diagrams rather than relying on
colour alone. Recordings are a publishing task, so this repository keeps the
versionable script rather than binary media.
