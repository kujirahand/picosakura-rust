#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)
TARGET=$SCRIPT_DIR/picosakura-pack

# set root
cd $SCRIPT_DIR

# cargo
cargo build --release

# copy files
mkdir -p $TARGET
cp ./target/release/picosakura $TARGET/
cp -r ./fonts $TARGET/fonts
cp -r ./samples $TARGET/samples
cp README.md $TARGET/
cp LICENSE $TARGET/

# zip
rm picosakura-pack.zip
zip picosakura-pack.zip -r picosakura-pack -x "*.DS_Store" "*__MACOSX*"
echo "ok"

