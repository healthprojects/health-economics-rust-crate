# Security audit checklist

- [ ] No `unsafe` code was introduced.
- [ ] Public functions cannot panic for inputs permitted by their specs.
- [ ] Slice and matrix shapes are checked before indexing.
- [ ] Float ordering uses `total_cmp` where NaN could otherwise panic.
- [ ] Exact money never passes through binary floating point.
- [ ] Dependencies are locked, minimal, audited, and license-compatible.
- [ ] CI actions are least-privilege and do not expose secrets to untrusted code.
- [ ] Generated examples and documentation do not contain real patient data,
  credentials, tokens, or confidential commercial inputs.
- [ ] Resource use remains bounded for caller-controlled collection sizes, or
  the limitation is documented.
- [ ] Private vulnerability reporting instructions remain valid.

For this pure calculation crate, injection, authentication, transport, and
storage controls are outside the codebase because it performs no I/O.
