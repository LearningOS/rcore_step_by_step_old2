#!/usr/bin/env bash

set -euxo pipefail

if [ -n "${TARGET:-}" ]; then
    cargo check --target $TARGET

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features inline-asm
    fi
fi

if [ -n "${CHECK_BLOBS:-}" ]; then
    PATH="$PATH:$PWD/gcc/bin"
    ./check-blobs.sh
fi
