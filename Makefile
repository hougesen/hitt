build:
	cargo check
	cargo build
	cargo build --release

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets --all-features -- -D warnings

test:
	make lint
	RUST_BACKTRACE=full cargo test --release

test-coverage:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	cargo llvm-cov --open

publish-crates:
	make build
	make test

	cargo publish -p hitt-formatter
	cargo publish -p hitt-parser
	cargo publish -p hitt-request
	cargo publish -p hitt
