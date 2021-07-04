#!/bin/bash
cargo build --release
strip target/release/drop
sstrip target/release/drop
upx --best --lzma target/release/drop