#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust --target riscv -i fu740.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
