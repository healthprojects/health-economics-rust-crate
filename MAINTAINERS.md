# Maintainer guide

## Decision making

Maintainers seek consensus on public contracts. The `spec/` directory records
the accepted calculation behavior; changing it is an API decision, not a
refactor. The repository owner has final say when consensus is not possible.

Commit access is granted by the repository owner after sustained, constructive
contributions and can be removed for security or conduct reasons. Formula,
dependency, license, and compatibility decisions are recorded in the spec,
an architecture decision record, or both.

## Triage

- Confirm bugs against the relevant spec.
- Label reports by area and impact; reserve `good first issue` for a scoped task
  with an accepted solution and test expectations.
- Close support questions after documenting reusable answers in the FAQ or
  rustdoc.
- Treat formula changes, dependencies, licenses, and public API compatibility as
  maintainer-review-required.

Apply one primary type label: `bug`, `enhancement`, `documentation`, or
`question`. Add `help wanted` only when the desired outcome and acceptance
criteria are settled; add `good first issue` only when the task is bounded and
requires no unresolved API decision. Use `duplicate`, `invalid`, and `wontfix`
with a closing explanation and link to the canonical discussion. Labels describe
work; they do not replace a written problem statement.

Create a milestone only for an intended release with a named owner and target
date. Empty milestones and project cards without an issue are avoided because
they create apparent work with no actionable scope.

## Pull requests

Use [`REVIEW.md`](REVIEW.md), require green CI, and ensure contributor-facing
documentation remains accurate. Follow [`RELEASING.md`](RELEASING.md) for
releases and [`SECURITY.md`](SECURITY.md) for private vulnerability reports.

## Repository continuity

GitHub is the canonical repository and crates.io is the release archive. Before
a release, maintainers should verify that a fresh clone builds, preserve the
signed or annotated release tag, and retain the packaged `.crate` artifact and
generated documentation as workflow artifacts. Repository administrators
should keep at least two-factor authentication and a current local mirror. A
recovery uses the canonical Git history, then verifies tags against crates.io
versions before restoring branch protections and repository secrets.
