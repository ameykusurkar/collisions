server: pkg
	python3 -m http.server

pkg: src
	wasm-pack build --target web

.PHONY: server
