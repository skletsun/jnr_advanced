#!/usr/bin/env bash

# Cleanup of previous iteration
rm -fR target
rm -r ./libfirst.dylib
rm -r ./libsecond.dylib
rm -r ./libwrapper.dylib

# Build java code first.
mvn compile assembly:single

# Prepare environment variables for building and running tests.
# This requires executables and libraries to be able to find libjvm and Rust stdlib.
export JAVA_HOME="$(java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | awk '{print $3}')"
echo "JAVA_HOME=${JAVA_HOME}"

# Find the directory containing Rust libstd.
export RUST_LIB_DIR=$(rustup run stable rustc --print sysroot)/lib
echo "RUST_LIB_DIR: ${RUST_LIB_DIR}"

NEW_RUSTFLAGS="-C link-arg=-Wl,-rpath,${RUST_LIB_DIR}"

if [[ "${RUSTFLAGS:-}" != "" && "${RUSTFLAGS}" != "${NEW_RUSTFLAGS}" ]]; then
    echo "[WARNING]: RUSTFLAGS variable is set and will be overridden."
    echo "Set RUSTFLAGS=${RUSTFLAGS}"
    echo "New RUSTFLAGS=${NEW_RUSTFLAGS}"
fi
export RUSTFLAGS="${NEW_RUSTFLAGS}"
echo "RUSTFLAGS=${RUSTFLAGS}"

cargo build --all

cp -v ./target/debug/libfirst.dylib .
cp -v ./target/debug/libsecond.dylib .
cp -v ./target/debug/libwrapper.dylib .

# And execute every test separately...
java -jar ./target/jnr-advanced-1.0-SNAPSHOT-jar-with-dependencies.jar