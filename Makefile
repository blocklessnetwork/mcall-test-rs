all: build

build:
	cargo build --release --target wasm32-wasi

clean:
	cargo clean
