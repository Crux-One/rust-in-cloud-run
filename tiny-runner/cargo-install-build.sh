#!/bin/sh

cargo install --path . && \
    cargo check && \
    cargo fmt && \
    cargo build
