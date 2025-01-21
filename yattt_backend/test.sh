#!/bin/bash

cargo run --release --features test &

cargo test --target-dir target/test_build