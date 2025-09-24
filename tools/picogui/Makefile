# Makefile for PicoGUI

# Variables
PYTHON = python
PIP = pip
PYINSTALLER = pyinstaller
APP_NAME = picogui
MAIN_SCRIPT = picogui.py
BUILD_DIR = build
DIST_DIR = dist
SPEC_FILE = $(APP_NAME).spec

# Default target
all: build

# Install dependencies
install:
	$(PIP) install -r requirements.txt
	$(PIP) install pyinstaller

# Create PyInstaller spec file
spec:
	$(PYINSTALLER) --onefile --windowed --name $(APP_NAME) \
		--add-data "bin:bin" \
		--add-data "bin/fonts:bin/fonts" \
		--hidden-import tkinter \
		--hidden-import TkEasyGUI \
		$(MAIN_SCRIPT)

# Build executable using existing spec file
build: install
	@if [ ! -f $(SPEC_FILE) ]; then \
		echo "Creating spec file..."; \
		$(MAKE) spec; \
	fi
	$(PYINSTALLER) $(SPEC_FILE)

# Build executable with one file option
build-onefile: install
	$(PYINSTALLER) --onefile --windowed --name $(APP_NAME) \
		--add-data "bin:bin" \
		--add-data "bin/fonts:bin/fonts" \
		--hidden-import tkinter \
		--hidden-import TkEasyGUI \
		$(MAIN_SCRIPT)

# Build executable with one directory option (default)
build-onedir: install
	$(PYINSTALLER) --onedir --windowed --name $(APP_NAME) \
		--add-data "bin:bin" \
		--add-data "bin/fonts:bin/fonts" \
		--hidden-import tkinter \
		--hidden-import TkEasyGUI \
		$(MAIN_SCRIPT)

# Clean build artifacts
clean:
	rm -rf $(BUILD_DIR)
	rm -rf $(DIST_DIR)
	rm -f *.spec
	rm -rf __pycache__

# Run the Python script directly
run:
	$(PYTHON) $(MAIN_SCRIPT)

# Run the built executable
run-exe:
	@if [ -f $(DIST_DIR)/$(APP_NAME) ]; then \
		./$(DIST_DIR)/$(APP_NAME); \
	elif [ -f $(DIST_DIR)/$(APP_NAME)/$(APP_NAME) ]; then \
		./$(DIST_DIR)/$(APP_NAME)/$(APP_NAME); \
	else \
		echo "Executable not found. Run 'make build' first."; \
	fi

# Create distribution package
package: build
	@echo "Creating distribution package..."
	cd $(DIST_DIR) && tar -czf ../$(APP_NAME)-macos.tar.gz *

# Display help
help:
	@echo "Available targets:"
	@echo "  install      - Install Python dependencies"
	@echo "  spec         - Create PyInstaller spec file"
	@echo "  build        - Build executable (default)"
	@echo "  build-onefile - Build single executable file"
	@echo "  build-onedir  - Build executable with dependencies in directory"
	@echo "  clean        - Remove build artifacts"
	@echo "  run          - Run Python script directly"
	@echo "  run-exe      - Run built executable"
	@echo "  package      - Create distribution package"
	@echo "  help         - Show this help message"

.PHONY: all install spec build build-onefile build-onedir clean run run-exe package help
