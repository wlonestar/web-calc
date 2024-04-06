.DEFAULT_GOAL := build

build:
	@rm -f src/calculator.rs
	@cargo build
	@wasm-pack build --target web
	
format:
	@rustfmt src/*.rs

clean:
	@rm -f src/calculator.rs
	@rm -rf pkg
	@cargo clean
