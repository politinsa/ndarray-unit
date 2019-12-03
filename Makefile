all:
	cargo fmt
	cargo build
	cargo test
	cargo doc
	cp -r ./target/doc doc
	cargo run

clean:
	cargo clean
