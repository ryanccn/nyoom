#!/bin/bash
set -eo pipefail

apt_updated=""
apt_install() {
    if [ ! "$apt_updated" ]; then
        sudo apt-get update
        apt_updated="y"
    fi

    sudo apt-get install "$@"
}

export RUSTFLAGS="-C strip=symbols -C codegen-units=1 -C opt-level=z"

if [[ "$TARGET" = "aarch64-unknown-linux-"* ]]; then
    apt_install gcc-aarch64-linux-gnu
    export RUSTFLAGS="$RUSTFLAGS --codegen linker=aarch64-linux-gnu-gcc"
fi
if [[ "$TARGET" = *"-linux-musl" ]]; then
    apt_install musl-dev musl-tools
fi

rustup target add "$TARGET"
cargo build -r --target "$TARGET" --locked

artifact_suffix=""
[[ "$TARGET" == *"windows"* ]] && artifact_suffix=".exe"

artifact_path="target/$TARGET/release/nyoom-$TARGET$artifact_suffix"

cp "target/$TARGET/release/nyoom$artifact_suffix" "$artifact_path"

echo "path=$artifact_path" >> "$GITHUB_OUTPUT"
