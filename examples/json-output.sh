#!/usr/bin/env bash
# JSON mode demonstration for llmsearch
#
# This script shows how to use the --json flag to get structured output
# that can be piped to jq for filtering and processing.

set -e  # Exit on error

# Ensure llmsearch is built
if [ ! -f "../target/release/llmsearch" ]; then
    echo "Error: llmsearch binary not found."
    echo "Please build it first: cargo build --release"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required for this example."
    echo "Install it with: sudo apt-get install jq"
    exit 1
fi

echo "=== Example 1: Basic JSON output ==="
echo "Search for 'pub fn' and display formatted JSON."
echo ""
../target/release/llmsearch -p 'pub fn' -g '*.rs' -r .. --json | jq '.'
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 2: Extract execution_id ==="
echo "Every run gets a unique UUID for auditability and tracking."
echo ""
EXECUTION_ID=$(../target/release/llmsearch -p 'fn main' -g '*.rs' -r .. --json | jq -r '.execution_id')
echo "Execution ID: $EXECUTION_ID"
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 3: Filter specific fields ==="
echo "Extract only file paths and line numbers from matches."
echo ""
../target/release/llmsearch -p 'struct\s+\w+' -g '*.rs' -r .. --json | jq '.matches[] | {file: .file, line: .line_number}'
echo ""
echo "Press Enter to continue..."
read

echo "=== Example 4: Match details breakdown ==="
echo "Show all available fields for each match:"
echo "  - match_id: Unique UUID for this match"
echo "  - file: Path to the matched file"
echo "  - byte_start, byte_end: Byte offsets in the file"
echo "  - line_number, column_number: Human-readable position"
echo "  - matched_text: The actual text that matched"
echo ""
../target/release/llmsearch -p 'pub\s+struct\s+Result' -g '*.rs' -r .. --json | jq '.matches[0]'
echo ""
echo "=== JSON output examples complete ==="
