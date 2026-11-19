# Roadmap

The crate's calculation backlog is specification-driven. All 91 contracts in
[`spec/`](spec/README.md) are currently implemented. New metric proposals start
as a spec and follow the workflow in [`AGENTS.md`](AGENTS.md).

## Current priorities

- Keep line, function, and region coverage at 100% and retain zero-warning test, Clippy, and rustdoc
  gates.
- Keep formulas, rustdoc, unit tests, integration tests, `llms.txt`, and
  `llms.json` synchronized.
- Improve contributor onboarding, release discipline, dependency auditing,
  link checking, and documentation accessibility.
- Add end-to-end examples when they clarify a real decision pathway rather
  than duplicate an existing formula example.

## Candidate work

- Publish generated rustdoc through GitHub Pages.
- Add scheduled dependency, license, vulnerability, spelling, and link checks.
- Review scheduled mutation reports and add deterministic fuzz/property sweeps
  for the small number of iterative algorithms.
- Evaluate optional serialization only if users present a concrete use case;
  adding a dependency requires maintainer approval.

## Explicit non-goals

- Async APIs for pure calculations.
- Replacing specified `Option` results with a different error model.
- Feature-gating individual metric modules without a demonstrated compile-time
  or binary-size need.
- Adding dependencies, bindings, analytics, or account-level automation merely
  for future-proofing.

The generated audit under `docs/audits/` is advisory input, not a promise that
every generic recommendation is applicable to this crate.
