publish-crates:
	cargo check
	cargo test
	cargo build --release
	cargo publish -p hitt-formatter
	cargo publish -p hitt-parser
	cargo publish -p hitt-request
	cargo publish -p hitt
