#! /usr/bin/env bash
cargo build --release
strip target/release/gitswitch
tar -czvf target/release/gitswitch.tar.gz target/release/gitswitch