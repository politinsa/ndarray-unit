all: tests doc
	cargo run

tests:
	cargo fmt
	cargo build
	cargo test

doc:
	cargo doc --no-deps
	cp -r ./target/doc/* docs
	echo "<script>window.location.href=\"./ndarray_unit/index.html\";</script>" > ./docs/index.html
	cargo readme > README.md

clean:
	cargo clean
	rm -fr docs/*

