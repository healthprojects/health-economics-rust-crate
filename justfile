default: ci

test:
    bin/test

specs:
    bin/check-spec-coverage

ci:
    bin/ci

test-coverage:
    bin/test-coverage

msrv:
    bin/make-ci-minimum-supported-rust-version

package:
    cargo package --locked
