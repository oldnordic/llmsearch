# llmsearch

A simple code search tool written in Rust as a learning project.

## What It Does

Searches text files recursively using regex patterns and outputs results in JSON format.

## What's Actually Tested

This project has 29 tests covering:

**Unit tests (19):**
- `is_text_file`: null byte detection, valid/invalid UTF-8, nonexistent files
- `build_line_index`: empty strings, no newlines, multiple newlines, trailing newlines
- `byte_to_line`: byte zero, exact match, end of file
- `walk_files`: extension filtering, multiple globs, no globs
- Context extraction edge cases
- Match ordering
- Limit functionality
- UTF-8 column calculation

**Integration tests (6):**
- JSON output validation
- Error handling: nonexistent directory, empty pattern, zero limit
- No matches case
- Glob filtering

**Determinism tests (4):**
- Same inputs produce identical outputs
- Sorting consistency (file path, then byte offset)
- Unique execution IDs per run
- Limit returns first N sorted results

## Installation

```bash
# Build from source
cargo build --release

# Install binary to PATH
cp target/release/llmsearch ~/.local/bin/
```

## Usage

```bash
# Basic search (current directory)
llmsearch -p "pattern"

# Search specific directory
llmsearch -r ./src -p "pattern"

# JSON output
llmsearch -p "pattern" --json

# Filter by file type (glob)
llmsearch -p "pattern" -g "*.rs"

# Limit results
llmsearch -p "pattern" -l 10
```

## CLI Options

| Option | Description |
|--------|-------------|
| `-r, --root <DIR>` | Root directory to search (default: current directory) |
| `-p, --pattern <REGEX>` | Regex pattern to search for (required) |
| `-g, --glob <GLOB>` | File glob filter (can be used multiple times) |
| `-l, --limit <N>` | Maximum matches to return (default: 100) |
| `--json` | Output JSON to stdout |
| `-h, --help` | Print help |

## JSON Output Format

```json
{
  "execution_id": "uuid-v4",
  "pattern": "search_pattern",
  "matches": [
    {
      "match_id": "uuid-v4",
      "file": "path/to/file",
      "byte_start": 0,
      "byte_end": 10,
      "matched_text": "match",
      "line_number": 1,
      "column_number": 1,
      "context_before": "before",
      "context_after": "after"
    }
  ],
  "match_count": 1
}
```

## Known Limitations

- **Linux-only** - Not tested on other platforms
- **UTF-8 only** - Skips binary files automatically
- **No Windows/macOS support**
- **Not published to crates.io** - Build from source only
- **Learning project** - Not production-ready

## Development

```bash
# Run all tests
cargo test

# Run with debug output
./target/debug/llmsearch -p "test"

# Build release binary
cargo build --release
```

## Dependencies

- `clap` 4.5 - CLI argument parsing
- `ignore` 0.4 - .gitignore-aware file walking
- `regex` 1.10 - Pattern matching
- `serde/serde_json` 1.0 - JSON serialization
- `uuid` 1.6 - Execution/match IDs

## License

GPL-3.0
