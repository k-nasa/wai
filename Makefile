.PHONY: build
build:
	cargo build --release

.PHONY: wasm
wasm:
	wat2wasm ./wasm/wat/add.wat -o  ./wasm/add.wasm
	wat2wasm ./wasm/wat/if.wat -o  ./wasm/if.wasm
