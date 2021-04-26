all : prepare serve

prepare : build copy

build : 
	cargo build --release --target wasm32-unknown-unknown

copy :
	cp -r ../assets .
	cp ../target/wasm32-unknown-unknown/release/*.wasm ./game.wasm

# Handy thing to serve the folder
serve :
	python -m http.server

