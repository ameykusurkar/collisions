server: pkg index.js
	python3 -m http.server

pkg: src
	wasm-pack build --target web

index.js: index.ts
	tsc

.PHONY: server
