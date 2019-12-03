all:
	cargo fmt
	cargo build
	cargo test
	cargo doc --target-dir doc
	cargo run

clean:
	cargo clean
