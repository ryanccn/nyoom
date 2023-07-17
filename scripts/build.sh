#!/bin/bash
set -eo pipefail

apt_updated=""
apt_update() {
    if [ ! "$apt_updated" ]; then
        sudo apt-get update
        apt_updated="y"
    fi
}

if [[ "$TARGET" = "aarch64-unknown-linux-"* ]]; then
    apt_update
    sudo apt-get install gcc-aarch64-linux-gnu

    export RUSTFLAGS="--codegen linker=aarch64-linux-gnu-gcc"
fi
if [[ "$TARGET" = *"-linux-musl" ]]; then
    apt_update
    sudo apt-get install musl-dev
fi

rustup target add "$TARGET"
cargo build -r --target "$TARGET" --locked

artifact_suffix=""
[[ "$TARGET" == *"windows"* ]] && artifact_suffix=".exe"

artifact_path="target/$TARGET/release/nyoom-$TARGET$artifact_suffix"

cp "target/$TARGET/release/nyoom$artifact_suffix" "$artifact_path"

echo "path=$artifact_path" >> "$GITHUB_OUTPUT"
