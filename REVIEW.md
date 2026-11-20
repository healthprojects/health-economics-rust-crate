# Code review guide

Review changes against the specification before reviewing style.

## Required checks

- A formula, signature, or `None`/`Err` change updates the spec, implementation,
  rustdoc, worked examples, and tests together.
- Public functions do not panic and denominator checks match the exact spec.
- Float sorting uses a total order and float assertions use tolerances.
- Money calculations remain decimal-backed and expose `rusty_money` types
  directly.
- New public items have complete documentation and runnable examples.
- No dependency, license option, or repository URL changes are introduced
  without explicit maintainer agreement.
- `bin/ci` and `bin/test-coverage` pass.

## Review posture

Prefer small, evidence-backed changes. Ask for the source behind a new formula,
threshold, or worked value. Treat generated documentation and indexes as part of
the public interface: check them for drift rather than reviewing only `src/`.
