#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)
TARGET=$SCRIPT_DIR/picosakura-pack

# cargo
cargo build --release
cp target/release/picosakura $TARGET/

mkdir -p $TARGET

# zip
cd $SCRIPT_DIR
rm picosakura-pack.zip
zip picosakura-pack.zip -r picosakura-pack -x "*.DS_Store" "*__MACOSX*"
echo "ok"

