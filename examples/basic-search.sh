#!/usr/bin/env bash
# Basic pattern matching demo for llmsearch
#
# This script demonstrates simple pattern searches in the llmsearch codebase.
# It shows the default human-readable output format with file paths and line numbers.

set -e  # Exit on error

# Ensure llmsearch is built
if [ ! -f "../target/release/llmsearch" ]; then
    echo "Error: llmsearch binary not found."
    echo "Please build it first: cargo build --release"
    exit 1
fi

echo "=== Example 1: Search for 'fn main' patterns ==="
echo "This finds all main function definitions in the codebase."
echo ""
../target/release/llmsearch -p 'fn main' -r ..
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 2: Search for 'struct' declarations ==="
echo "This finds all struct definitions in Rust source files."
echo ""
../target/release/llmsearch -p 'struct\s+\w+' -g '*.rs' -r ..
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 3: Search for 'impl' blocks ==="
echo "This finds all implementation blocks in the codebase."
echo ""
../target/release/llmsearch -p 'impl\s+\w+' -g '*.rs' -r ..
echo ""
echo "=== Basic search examples complete ==="
