all: pkg index.js

pkg: src
	wasm-pack build --target web

index.js: index.ts
	tsc

server: all
	python3 -m http.server

.PHONY: server
