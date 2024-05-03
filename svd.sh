#!/bin/bash
# if ["$1" -eq "-i"]; then
#     cargo install -f svd2rust
#     cargo install -f form
rm -r src/
svd2rust -i XMC4300.svd --ident-formats-theme legacy
mkdir src
form -i lib.rs -o src/
cargo fmt
rm lib.rs
