#!/usr/bin/sh

sudo chown -R rust:rust /home/rust/src

cargo build --release --verbose