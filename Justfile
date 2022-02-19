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

# Lint it
lint:
	cargo +nightly fmt --all -- --check
	cargo +nightly clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
	cargo +nightly check
	cargo +nightly audit

# Format what can be formatted
fmt:
	cargo +nightly fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery
	cargo +nightly fmt --all
	npx prettier --write **.yml

# Clean the build directory
clean:
	cargo clean
