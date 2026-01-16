# llmsearch Examples

This directory contains runnable examples demonstrating common llmsearch workflows. Each script is self-contained and works on the llmsearch codebase itself.

## Prerequisites

Before running the examples, ensure llmsearch is built:

```bash
cargo build --release
```

This creates the binary at `target/release/llmsearch` which the examples use.

## Running the Examples

Each script can be run directly from this directory:

```bash
# Basic pattern matching examples
./basic-search.sh

# JSON output and jq integration
./json-output.sh

# File glob filtering patterns
./glob-filtering.sh
```

## What Each Example Demonstrates

### basic-search.sh

Shows simple pattern searches with human-readable output:
- Searching for function definitions (`fn main`)
- Finding struct declarations
- Locating implementation blocks
- Default output format with file paths and line numbers

### json-output.sh

Demonstrates structured JSON output for LLM workflows:
- Using the `--json` flag for machine-readable output
- Piping to `jq` for filtering and formatting
- Extracting execution IDs for auditability
- Accessing all match fields (file, byte offsets, line/column numbers, context)

**Note**: Requires `jq` to be installed (`sudo apt-get install jq`)

### glob-filtering.sh

Shows file filtering using glob patterns:
- Single glob patterns (e.g., `*.rs`)
- Multiple glob patterns (e.g., `*.rs` and `*.toml`)
- Complex wildcard patterns (e.g., `test_*.rs`)
- Directory-specific searches

## Expected Output

The examples search the llmsearch codebase, so you'll see matches from:
- `src/` - Rust source files
- `tests/` - Test files
- `docs/` - Documentation files
- This examples directory itself

File paths will vary based on your system, but the structure and content of matches will be consistent.

## Learning Path

1. Start with `basic-search.sh` to understand default behavior
2. Move to `json-output.sh` for LLM integration patterns
3. Explore `glob-filtering.sh` to learn file filtering

Each script includes comments explaining what's happening and prompts to continue step-by-step.
