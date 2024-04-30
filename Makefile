.PHONY: watch coverage test lint lint-ci format format-ci audit

watch:
	@cargo watch -x check

# The entire development loop cycle, compile, test, and re-run
watch-cycle:
	@cargo watch -x check -x test -x run

# cargo install cargo-tarpaulin
coverage:
	@cargo tarpaulin --ignore-tests

test:
	@cargo test

# rustup component add clippy
lint:
	@cargo clippy

# this will fail on any linter warnings
lint-ci:
	@cargo clippy -- -D warnings

# rustup component add rustfmt
format:
	@cargo fmt

format-ci:
	@cargo fmt -- --check

# Security audit of all dependencies
# cargo install cargo-audit
audit:
	@cargo audit