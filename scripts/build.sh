#!/bin/bash
set -eo pipefail

exec_print() {
    echo -e "\033[2m\$\033[0m $*"
    "$@"
}

apt_updated=""
apt_install() {
    if [ ! "$apt_updated" ]; then
        exec_print sudo apt-get update
        apt_updated="y"
    fi

    exec_print sudo apt-get install "$@"
}

if [[ -z "$TARGET" ]]; then
    echo -e "\033[31mNo TARGET provided! Exiting.\033[0m"
    exit 1
fi

export RUSTFLAGS=""
export CARGO_PROFILE_RELEASE_LTO="fat"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1"

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
echo -e "\033[2m>\033[0m CARGO_PROFILE_RELEASE_LTO=\033[36m\"$CARGO_PROFILE_RELEASE_LTO\"\033[0m"
echo -e "\033[2m>\033[0m CARGO_PROFILE_RELEASE_CODEGEN_UNITS=\033[36m\"$CARGO_PROFILE_RELEASE_CODEGEN_UNITS\"\033[0m"

exec_print cargo build --release --locked --target "$TARGET"

result_suffix=""
artifact_basename_suffix=""
[[ "$TARGET" == *"windows"* ]] && result_suffix=".exe"
[[ "$TARGET" == *"musl" ]] && artifact_basename_suffix="-static"

result_path="nyoom-$TARGET$artifact_basename_suffix$result_suffix"

exec_print cp "target/$TARGET/release/nyoom$result_suffix" "$result_path"

[ -f "$GITHUB_OUTPUT" ] && echo "path=$result_path" >> "$GITHUB_OUTPUT"
