build:
	@cargo build

clean:
	@cargo clean

docs: build
	@cargo doc --no-deps

run:
	@cargo run

.PHONY: build test docs style-check lint