test : core serve

prepare : core zip

core : build copy download

build : 
	cargo build --release --target wasm32-unknown-unknown

copy :
	cp -r ../assets .
	cp ../target/wasm32-unknown-unknown/release/*.wasm ./game.wasm

# Get latest versions of js files
# wget -nc is "no clobber"
download :
	wget -nc https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js \
	wget -nc https://raw.githubusercontent.com/gamma-delta/quad-wasmnastics/main/js/wasmnastics.js \

zip : 
	zip -r game.zip assets game.wasm *.js index.html 

# Handy thing to serve the folder
serve :
	python -m http.server
