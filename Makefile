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

publish-crates:
	make build
	make test

	cargo publish -p hitt-formatter
	cargo publish -p hitt-parser
	cargo publish -p hitt-request
	cargo publish -p hitt
