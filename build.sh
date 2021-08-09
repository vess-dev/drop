#!/bin/bash
# Build Linux.
cargo build --release
strip target/release/drop
sstrip target/release/drop
upx --lzma target/release/drop
# Crosscompile Windows.
# You will need to execute these manually.
# cargo build --release --target x86_64-pc-windows-gnu
# strip target/x86_64-pc-windows-gnu/release/drop.exe
# upx --lzma target/x86_64-pc-windows-gnu/release/drop.exe