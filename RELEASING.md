# Release process

1. Confirm the worktree is clean and the intended commit is on `main`.
2. Run `bin/ci`, `bin/coverage`, and the MSRV check.
3. Update `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `llms.json`, and citation
   metadata for the release version.
4. Run `cargo package --locked` and inspect the packaged file list.
5. Commit the release changes and create an annotated `vX.Y.Z` tag.
6. Publish with `cargo publish --locked` only from the reviewed tag.
7. Push the commit and tag, create GitHub release notes from the changelog, and
   verify docs.rs and crates.io metadata.
8. Add the next `[Unreleased]` comparison link if the release changed it.

Publishing and pushing are external, irreversible actions and require explicit
maintainer authorization.
