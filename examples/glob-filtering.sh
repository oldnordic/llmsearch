#!/usr/bin/env bash
# File filtering examples using glob patterns
#
# This script demonstrates how to use the -g flag to filter which files
# are searched. You can use multiple glob patterns for complex filtering.

set -e  # Exit on error

# Ensure llmsearch is built
if [ ! -f "../target/release/llmsearch" ]; then
    echo "Error: llmsearch binary not found."
    echo "Please build it first: cargo build --release"
    exit 1
fi

echo "=== Example 1: Single glob pattern ==="
echo "Search only in Rust source files (*.rs)"
echo ""
../target/release/llmsearch -p 'use\s+serde' -g '*.rs' -r ..
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 2: Multiple glob patterns ==="
echo "Search in both Rust files (*.rs) and TOML config files (*.toml)"
echo ""
../target/release/llmsearch -p 'test' -g '*.rs' -g '*.toml' -r ..
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 3: Complex wildcard patterns ==="
echo "Use wildcards to match specific file naming patterns"
echo "This example finds test files matching 'test_*.rs'"
echo ""
../target/release/llmsearch -p '#\[test\]' -g 'test_*.rs' -r ..
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 4: Directory-specific patterns ==="
echo "Search for 'mod' declarations only in src directory"
echo ""
../target/release/llmsearch -p '^mod\s+\w+' -g '*.rs' -r ../src
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 5: No glob filter ==="
echo "Search all text files (default behavior when -g is not specified)"
echo "This will search across all file types in the project"
echo ""
../target/release/llmsearch -p 'llmsearch' -r .. --limit 10
echo ""
echo "=== Glob filtering examples complete ==="
