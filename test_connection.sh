#!/bin/bash

echo "Testing Tenor Docker connection..."
echo ""

# Build the project
echo "Building..."
cargo build --quiet 2>&1 | grep -v "warning:" || true

# Run for a few seconds and capture output
echo ""
echo "Running Tenor TUI (will auto-exit after 2 seconds)..."
echo "Press Ctrl+C if you need to exit earlier"
echo ""

timeout 2 cargo run 2>&1 || true

echo ""
echo "If you saw the TUI interface above, the connection is working!"
echo "Run 'cargo run' to use the application interactively."
