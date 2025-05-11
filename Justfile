# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Build release version
build:
	cargo build --release

# Check performance
bench:
	cargo bench

# Lint the project
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features
	cargo check
	cargo audit

# Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo clippy --allow-dirty --allow-staged --fix --all-features
	cargo fmt --all
	npx prettier --write **.yml

# Clean the build directory
clean:
	cargo clean
