# Repository instructions

Read `AGENTS.md` and every file under `spec/` before proposing code. The specs
are authoritative for signatures, formulas, and `None`/`Err` behavior. A
contract change must update its spec, implementation, rustdoc, and tests in the
same pull request.

Do not add dependencies, validation beyond specified denominator/shape guards,
or alternate error models without maintainer agreement. Preserve the direct
`rusty_money::Money` signatures in exact-currency modules. New public items need
complete rustdoc and `#[must_use]` where required.

Generated tests and examples must state the contract or invariant they verify,
use tolerance comparisons for floats, and remain deterministic. Before
suggesting completion, run `bin/ci` and `bin/coverage`; both must pass, with
coverage at 100% for lines, functions, and regions.
