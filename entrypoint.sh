#!/usr/bin/env sh

sudo chown -R rust:rust /home/rust/src

cargo build --release --verbose

sudo chown -R 1001:1001 /home/rust/src
