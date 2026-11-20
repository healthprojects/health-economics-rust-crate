# Contributing to Health Economics Rust crate

Thanks for helping improve this project.

## Ways to contribute

- **Report a problem:** open an issue describing what is wrong, unclear, or missing.
- **Fix or improve something:** open a pull request. Small fixes can go straight to a pull request; for larger changes, please open an issue first so we can agree on the approach.
- **Add content or examples:** follow the structure and style of the existing files.

## Development

Read [AGENTS.md](AGENTS.md) and [spec/README.md](spec/README.md) first: the `spec/` directory is the source of truth, and a change to a formula, signature, or `None`/`Err` condition updates the spec, the function in `src/`, its rustdoc, and its tests together.

Run the same checks CI runs (format, clippy, tests, doctests, docs):

```sh
make ci                                     # runs bin/ci
make check-spec-coverage                    # all specs are implemented and indexed
make ci-minimum-supported-rust-version    # MSRV (1.98) and the next release
make test-coverage                               # 100% line/function/region coverage
```

The project supports `just ci`, `just coverage`, and `just msrv` as aliases.
An optional local pre-commit hook is configured in `.pre-commit-config.yaml`;
installing the third-party `pre-commit` runner is not required to contribute.

Code review follows [REVIEW.md](REVIEW.md). Maintainer triage and release
processes are documented in [MAINTAINERS.md](MAINTAINERS.md) and
[RELEASING.md](RELEASING.md).

## Code of conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contribution is offered under the terms in [LICENSE.md](LICENSE.md).
