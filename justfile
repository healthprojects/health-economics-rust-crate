default: ci

test:
    bin/test

specs:
    bin/check-spec-coverage

ci:
    bin/ci

coverage:
    bin/coverage

msrv:
    bin/test-minimum-supported-rust-version

package:
    cargo package --locked
