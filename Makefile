ci-coverage:
	cargo tarpaulin --out Xml
	curl -s https://codecov.io/bash | bash

ci-lint:
	rustfmt --edition 2018 --check src/*.rs
	rustfmt --edition 2018 --check src/**/*.rs

ci-test:
	cargo test

dev-coverage:
	cargo tarpaulin --out Html

dev-format:
	rustfmt --edition 2018 --check src/{**/,}*.rs

dev-run:
	RUST_LOG=debug cargo run

dev-test:
	cargo test

install-rustfmt:
	rustup component add rustfmt

install-tarpaulin:
	cargo install cargo-tarpaulin
