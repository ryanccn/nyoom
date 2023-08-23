#!/bin/bash
set -eo pipefail

print_command() {
    echo -e "\033[2m\$ $*\033[0m"
}

apt_updated=""
apt_install() {
    if [ ! "$apt_updated" ]; then
        sudo apt-get update
        apt_updated="y"
    fi

    print_command "sudo apt-get install $*"
    sudo apt-get install "$@"
}

export RUSTFLAGS="-C lto=thin -C embed-bitcode=yes -C strip=symbols -C codegen-units=1 -C opt-level=z"

if [[ "$TARGET" = "aarch64-unknown-linux-"* ]]; then
    apt_install gcc-aarch64-linux-gnu
    export RUSTFLAGS="$RUSTFLAGS -C linker=aarch64-linux-gnu-gcc"
fi
if [[ "$TARGET" = *"-linux-musl" ]]; then
    apt_install musl-dev musl-tools
    export RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static"
fi
if [[ "$TARGET" = "aarch64-unknown-linux-musl" ]]; then
    apt_install clang llvm
    export CC_aarch64_unknown_linux_musl="clang"
    export AR_aarch64_unknown_linux_musl="llvm-ar"
    export RUSTFLAGS="$RUSTFLAGS -C link-self-contained=yes -C linker=rust-lld"
fi

echo -e "\033[2m>\033[0m RUSTFLAGS=\033[36m\"$RUSTFLAGS\"\033[0m"

print_command "cargo build -r --target $TARGET --locked"
cargo build -r --target "$TARGET" --locked

artifact_suffix=""
[[ "$TARGET" == *"windows"* ]] && artifact_suffix=".exe"
[[ "$TARGET" == *"musl" ]] && artifact_suffix="-static"

artifact_path="target/$TARGET/release/nyoom-$TARGET$artifact_suffix"

cp "target/$TARGET/release/nyoom$artifact_suffix" "$artifact_path"

[ -f "$GITHUB_OUTPUT" ] && echo "path=$artifact_path" >> "$GITHUB_OUTPUT"
