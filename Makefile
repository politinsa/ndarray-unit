all:
	cargo build
	cargo test
	cargo doc
	cargo run

clean:
	cargo clean
