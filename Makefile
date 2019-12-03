all:
	cargo fmt
	cargo build
	cargo test
	cargo doc
	cp -r ./target/doc docs
	cargo run

clean:
	cargo clean
