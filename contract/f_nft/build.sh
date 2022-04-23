#!/bin/bash
set -e

export CONTRACT=f_nft.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/$CONTRACT ./res/
wasm-opt -Os -o res/output_s.wasm res/$CONTRACT
ls res -lh
# gzip -9 res/output_s.wasm | wc -c 