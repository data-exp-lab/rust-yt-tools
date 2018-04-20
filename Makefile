SHELL := /bin/bash
default: all

static/binary_data.dat:
	cd static && curl -OJ http://use.yt/upload/215d89a5

all: static/binary_data.dat
	cargo web start --target=wasm32-unknown-unknown
