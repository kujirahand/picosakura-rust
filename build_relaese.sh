#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)
TARGET=$SCRIPT_DIR/picosakura-pack
cargo build --release
cp target/release/picosakura $TARGET/
echo "ok"

