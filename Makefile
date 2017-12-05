SHELL := /bin/bash
default: all

binary_data.dat:
	curl -OJ http://use.yt/upload/215d89a5

all: binary_data.dat
	cargo build --target=wasm32-unknown-emscripten --release
	mkdir -p site
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} cp {} site/site.wasm
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} site/site.js
