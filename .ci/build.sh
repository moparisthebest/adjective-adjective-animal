#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=adjective_adjective_animal

DISABLE_TESTS=${DISABLE_TESTS:-0}

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"

# build library
cross build --target $TARGET --release
# build example binary
cross build --target $TARGET --release --manifest-path cli/Cargo.toml
# build docs
cross doc --target $TARGET

# to check how they are built
file "target/$TARGET/release/aaa$SUFFIX"

if [ $DISABLE_TESTS -ne 1 ]
then
    cross test --target $TARGET
fi

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/aaa$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
cp "target/$TARGET/release/aaa$SUFFIX" "release/aaa-$TARGET$SUFFIX"

echo 'build success!'
exit 0
