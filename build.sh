#!/bin/bash
cargo build --release
strip target/release/drop
sstrip target/release/drop
upx --ultra-brute target/release/drop
