.DEFAULT_GOAL := build

build:
	@rm -f src/calculator.rs
	@cargo build
	@wasm-pack build --target bundler
	@cd site && npm i -D -S

deploy:
	@cd site && npm run serve

format:
	@rustfmt src/*.rs

clean:
	@rm -f src/calculator.rs
	@rm -rf pkg
	@cargo clean
