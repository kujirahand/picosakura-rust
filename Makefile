# Makefile

# Default target
all: build_picogui

wasm:
	wasm-pack build --target web --release

# sakuramml-v2 compiler (default is sakuramml)
wasm-v2:
	wasm-pack build --target web --release --no-default-features --features sakuramml-v2

# Copy files
build_picogui:
	mkdir -p tools/picogui/bin/fonts
	cargo build --release
	cp target/release/picosakura tools/picogui/bin/
	cp fonts/*.sf2 tools/picogui/bin/fonts/
	cd tools/picogui && make
	
