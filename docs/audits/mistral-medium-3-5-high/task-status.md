# Item-by-item backlog status — 2026-11-19

Status meanings:

- **Implemented** — source-controlled deliverable exists and its applicable
  verifier passes.
- **Existing/equivalent** — the requested outcome was already provided, or a
  more direct repository mechanism provides it.
- **Not applicable** — the generic suggestion conflicts with this pure,
  spec-driven crate or has no relevant runtime surface.
- **Owner action** — needs account settings, publication authority, identity,
  funding details, or explicit dependency/API approval.

## New visitors and promotion

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| README badges | Implemented | crates.io, downloads, stars, contributors, CI, coverage, MSRV, and license badges |
| `SECURITY.md` | Implemented | Private-reporting policy and supported-version guidance |
| Funding configuration | Owner action | Sponsorship platform/account must not be guessed |
| Discussion templates | Implemented | Q&A, ideas, and show-and-tell forms; enabling Discussions is an owner setting |
| `good first issue` label | Existing/equivalent | Label exists; no fake issue was created in an empty issue tracker |
| Project logo | Implemented | Accessible source SVG under `assets/` |
| crates.io usage examples | Implemented | Packaged README, six examples, corrected `health-economics = "1"` install line |
| GitHub Pages | Implemented | rustdoc + mdBook build/deploy workflow; repository Pages activation is owner action |
| Awesome Rust submission | Owner action | Submission draft exists; external PR needs publication authority |
| Roadmap | Implemented | `ROADMAP.md` |
| Contribution statistics | Implemented | Contributors and stars badges; GitHub Insights remains authoritative |
| Social announcements | Implemented | Short, LinkedIn, and Reddit guidance in promotion kit; posting is owner action |
| Hacktoberfest | Owner action | Requires a participating year and maintainer review capacity |
| Press and speaker kit | Implemented | Facts, logo, key messages, release copy, and talk outline |
| Testimonials | Owner action | Must come from consenting real users; none are fabricated |
| Analytics | Not applicable | crates.io and GitHub provide aggregate metrics without adding visitor tracking |

## Documentation and learning

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| Module category badges | Existing/equivalent | Accessible themed headings and complete module index avoid colour-only meaning |
| Interactive tutorial | Implemented | Searchable mdBook with four sequential tutorials and exercises |
| API documentation search | Existing/equivalent | rustdoc’s built-in search is deployed with the Pages site |
| Getting started | Implemented | `docs/getting-started.md` |
| FAQ | Implemented | `FAQ.md` |
| Architecture decisions | Implemented | Three ADRs covering numeric types, money, and strict linting |
| Glossary | Implemented | `GLOSSARY.md` |
| Comparison table | Implemented | `docs/comparisons.md` |
| Benchmark results | Implemented | Dependency-free harness and dated results under `docs/audits/benchmarks/` |
| Data-flow diagrams | Implemented | Mermaid decision and dependency graphs |
| Case studies | Implemented | Hospital, digital-health, AI diagnostic, and prevention scenarios |
| Video tutorial | Implemented | Timed outline plus accessible narration guidance; recording is owner publication |
| Module dependency graph | Implemented | `docs/dependency-graph.md`, reproducible with `cargo tree` |
| Migration guide | Implemented | `docs/migration.md` |
| Audio narration | Implemented | Narration-ready episode scripts and transcript requirements |
| Cheat sheet | Implemented | `docs/cheat-sheet.md`; Markdown is the accessible, versionable source |
| Interactive playground | Owner action | Requires a supported web binding/toolchain and deployment decision |

## Engineering

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| Rust 2024 | Existing/equivalent | Crate already uses edition 2024 and MSRV 1.98 |
| `#[non_exhaustive]` review | Not applicable | Closed contract types should remain constructible/matchable; adding it is restrictive |
| Changelog | Implemented | Keep-a-Changelog format from 1.0.2 |
| Pre-commit hooks | Implemented | Optional local hook runs `bin/ci` |
| `justfile` | Implemented | Test, CI, coverage, spec, MSRV, and package recipes |
| Feature flags | Not applicable | No optional module/dependency use case; per-module flags are an explicit non-goal |
| Nightly compatibility | Implemented | Non-blocking scheduled/PR job |
| Cross-module doc examples | Existing/equivalent | Six executable examples and four case studies compose modules |
| Const generics | Not applicable | Slice APIs already support arbitrary model sizes without new public types |
| `no_std` | Owner action | Exact-money dependencies use `std`; a support split needs an approved design |
| WASM target | Implemented | `wasm32-unknown-unknown` compilation is locally proven and checked in CI |
| Serde | Owner action | No serialization contract exists; dependency/API expansion needs approval |
| Custom errors | Not applicable | Spec requires `Option`; Money construction already returns `MoneyError` |
| Builder pattern | Not applicable | Scalar functions and small literal data carriers have no staged construction |
| FFI/Python bindings | Owner action | New ABI, packaging, dependencies, and support policy need approval |
| Async, macros, custom derives | Not applicable | Pure synchronous formulas perform no I/O and need no code generation |

## Testing and validation

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| Property tests | Existing/equivalent | Deterministic invariant sweeps cover algebraic properties; literal `proptest` needs approval |
| Mutation testing | Implemented | 273 configured mutants: 259 caught, 12 timeout-detected, 2 unviable, 0 missed |
| Fuzz testing | Existing/equivalent | Deterministic extreme/shape/NaN-safe stress cases; literal cargo-fuzz needs approval |
| Snapshot/golden tests | Existing/equivalent | Spec worked examples and seeded sampler sequences are executable golden values |
| Integration tests | Implemented | Eight cross-module suites plus runnable examples |
| Coverage report | Implemented | HTML artifact and CI gate at 100% regions/functions/lines |
| Doctest validation | Implemented | `cargo test --doc` in `bin/ci` |
| Performance regression | Implemented | Dependency-free reproducible benchmark harness; no flaky shared-runner threshold |
| Stress tests | Implemented | Extreme magnitudes, empty inputs, boundaries, malformed shapes, and seeded sampling |
| Platform tests | Implemented | Linux, macOS, Windows, MSRV, nightly, and WASM workflows |
| Vulnerability/license checks | Implemented | `cargo audit` and `cargo deny check` |
| Formal verification | Not applicable | No unsafe or critical systems kernel; executable contracts and mutation tests fit the risk |
| Contract tests | Implemented | `bin/check-spec-coverage` verifies all 91 spec-to-index routes |

## Auditing, metrics, and AI

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| Code/API/architecture/coverage audits | Implemented | Dated audit plus 100% coverage and zero-missed focused mutation campaign |
| Security checklist | Implemented | Panic, unsafe, I/O, dependency, numeric, and disclosure review |
| Static/dependency analysis | Implemented | Clippy pedantic + cognitive complexity, cargo-deny, cargo-audit, dependency graph |
| Review guidelines | Implemented | `REVIEW.md` |
| Spelling/link/docs checks | Implemented | typos, Lychee, denied missing docs, doctests |
| Accessibility/i18n review | Implemented | Accessible SVG/text, captions/transcripts guidance; crate has no localized UI strings |
| Performance review | Implemented | Benchmark harness and audit report |
| Memory/thread/crypto review | Not applicable | No unsafe, concurrency, or cryptographic API surface |
| Download/release/doc/test metrics | Implemented | crates.io badges and `docs/project-metrics.md` with authoritative sources |
| GitHub Insights and issue/PR metrics | Existing/equivalent | Native Insights is authoritative; no duplicate tracker is installed |
| Code complexity | Implemented | Cognitive-complexity Clippy lint is a required CI gate |
| AI agent skill | Implemented | All 91 modules route from the validated skill table |
| AI review/test/example/completion guidance | Implemented | Repository coding-agent instructions enforce specs and quality gates |
| AI documentation/migration help | Existing/equivalent | Grounded skill, rustdoc, machine indexes, FAQ, and migration guide |
| AI dependency updater | Existing/equivalent | Dependabot is deterministic and reviewable |
| AI performance/security/translation services | Not applicable | Would add ungrounded external processing; benchmarks/audits/source docs are authoritative |
| OpenAPI/Swagger | Not applicable | This is a Rust calculation library, not an HTTP API |

## Content and infrastructure

| Backlog item | Status | Evidence or decision |
| --- | --- | --- |
| More examples and datasets | Implemented | Six programs and three synthetic CSV datasets |
| Four scenario case studies | Implemented | Hospital, digital ROI, AI diagnostic, prevention |
| Notebook | Implemented | Valid Rust-kernel Jupyter notebook with assertion checks |
| Interactive demo | Owner action | Requires approved browser bindings/site scope |
| Video, blog, podcast, talk, workshop, course | Implemented | Versionable scripts, outlines, exercises, checks, and curriculum; publishing is owner action |
| MSRV policy | Implemented | Rust 1.98 plus 1.99.1 matrix and local script |
| Issue/PR templates | Implemented | Structured bug, feature, question, and PR forms |
| Code of conduct/license clarity | Implemented | Reviewed conduct document and explicit multi-license `OR` explanation |
| Governance/release/maintainer guides | Implemented | Decision, access, triage, review, release, and continuity policies |
| Label system | Implemented | Primary-type and contributor-label rules match existing repository labels |
| Milestones/project board | Owner action | Need a release owner/date and actionable issues; tracker is currently empty |
| Automation bots | Implemented | Dependabot plus CI, quality, mutation, and Pages workflows |
| Backup strategy | Implemented | Canonical history, release artifacts, tags, authentication, and restore procedure |

The owner-action rows are the only unresolved items. They cannot be completed
correctly from repository evidence alone.
