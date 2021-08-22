#!/bin/bash
# Build for Linux.
cargo build --release
strip target/release/drop
sstrip target/release/drop
upx --lzma target/release/drop