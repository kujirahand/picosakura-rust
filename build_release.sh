#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)
TARGET=$SCRIPT_DIR/picosakura-pack
ZIPFILE=picosakura-pack_macos.zip

# set root
cd $SCRIPT_DIR

# cargo
cargo build --release

# copy files
mkdir -p $TARGET
mkdir -p $TARGET/fonts
mkdir -p $TARGET/samples
cp ./target/release/picosakura $TARGET/
cp ./fonts/* $TARGET/fonts/
cp ./samples/* $TARGET/samples/
cp README.md $TARGET/
cp LICENSE $TARGET/

# zip
rm $ZIPFILE
zip $ZIPFILE -r picosakura-pack -x "*.DS_Store" "*__MACOSX*"
echo "ok"

