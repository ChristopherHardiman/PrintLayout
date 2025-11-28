.PHONY: build run test clean fmt clippy release

build:
	cargo build

run:
	RUST_LOG=info cargo run

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

release:
	cargo build --release

check: fmt clippy test
	@echo "All checks passed!"
