#!/bin/bash

# Start cargo run in the background
cargo run --release --features test &

# Wait for the webserver to be ready
until curl -s http://localhost:8080/api/v1 > /dev/null; do
  sleep 1
done

# Run tests
cargo test --target-dir target/test_build
