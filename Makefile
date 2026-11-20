##
# help: Display this help message.
##
.PHONY: help
help:
        @awk '/^##/{a=1-a}a' $(MAKEFILE_LIST) | cut -c3-

# newline: This is a print helper when you want to embed a newline in a message.
define newline


endef

##
# make test: test everything
##
.PHONY: test
test:
	bin/test

##
# make check-spec-coverage: verify every spec is implemented and indexed.
##
.PHONY: check-spec-coverage
check-spec-coverage:
	bin/check-spec-coverage

##
# make ci-minimum-supported-rust-version: test with the minimum supported Rust version
##
.PHONY: ci-minimum-supported-rust-version
ci-minimum-supported-rust-version:
	bin/make-ci-minimum-supported-rust-version

##
# make test-benchmarks: run the dependency-free core performance benchmarks.
##
.PHONY: test-benchmarks
test-benchmarks:
	bin/test-benchmarks

##
# make ci: run bin/ci to do continuous integration.
##
.PHONY: ci
ci:
	bin/ci

##
# make test-coverage: enforce 100% coverage and generate an HTML report.
##
.PHONY: test-coverage
test-coverage:
	bin/test-coverage
