# Makefile

# Default target
all: build_picogui

wasm:
	wasm-pack build --target web --release

# Copy files
build_picogui:
	mkdir -p tools/picogui/bin/fonts
	cargo build --release
	cp target/release/picosakura tools/picogui/bin/
	cp fonts/*.sf2 tools/picogui/bin/fonts/
	cd tools/picogui && make
	
