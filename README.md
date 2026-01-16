# llmsearch

**Deterministic, structured search for LLM workflows**

llmsearch is an LLM-native code search tool that provides structured JSON output designed for AI agent workflows. Think of it as ripgrep with deterministic, queryable results that LLMs can rely on without guessing.

## What is llmsearch?

Traditional CLI tools (ripgrep, grep) output unstructured text designed for human operators. LLMs consuming this output must parse streams, maintain state in memory, and lose that state after context compaction — leading to guesswork.

llmsearch fixes this by returning structured JSON with:
- **Exact byte offsets** for precise code navigation
- **Line/column numbers** for editor integration
- **Execution IDs** (UUID v4) for auditability and retrieval
- **Deterministic ordering** — same inputs always yield identical ordered results
- **Match-specific IDs** for referencing individual results

Every match includes context before/after the matched text, making it easy for LLMs to understand surrounding code without additional file reads.

## Key Features

- **Regex pattern matching** with full Rust regex syntax
- **File glob filtering** (e.g., `*.rs`, `*.toml`, `test_*.rs`)
- **Result limiting** for large codebases
- **.gitignore-aware** file walking (respects your ignore rules)
- **UTF-8 text file support** (skips binary files automatically)
- **JSON output mode** for structured consumption
- **Deterministic ordering** sorted by file path, then byte offset
- **Execution tracking** with unique IDs for every search run

## Installation

### From source

```bash
# Install locally
cargo install --path .

# Or build release binary
cargo build --release
# Binary at target/release/llmsearch
```

### Requirements

- Rust 2024 edition or later
- Linux-only (initial release)

## Quick Start

```bash
# Basic search in current directory
llmsearch -p 'fn main'

# Search specific directory with file filter
llmsearch -r ./src -p 'struct.*Error' -g '*.rs'

# JSON output for LLM consumption
llmsearch -p 'TODO:' --json

# Multiple file types
llmsearch -p 'test' -g '*.rs' -g '*.toml'

# Limit results in large codebase
llmsearch -p 'async fn' -l 50
```

## JSON Output Schema

When using `--json`, llmsearch outputs structured JSON to stdout:

```json
{
  "execution_id": "550e8400-e29b-41d4-a716-446655440000",
  "pattern": "fn main",
  "matches": [
    {
      "match_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "file": "src/main.rs",
      "byte_start": 120,
      "byte_end": 132,
      "matched_text": "fn main() {",
      "line_number": 10,
      "column_number": 4,
      "context_before": "pub ",
      "context_after": "    println!(\"Hello\");\n}"
    }
  ],
  "match_count": 1
}
```

### Field descriptions

- **execution_id**: UUID v4 identifying this specific search run (useful for caching/auditing)
- **pattern**: The regex pattern that was searched
- **matches**: Array of match objects, sorted deterministically by file path then byte_start
- **match_id**: UUID v4 for each individual match (for referencing specific results)
- **file**: Relative path from search root
- **byte_start/byte_end**: Exact byte offsets in the file (UTF-8 aware)
- **matched_text**: The actual text that matched the pattern
- **line_number**: 1-indexed line number where match occurs
- **column_number**: 1-indexed column number (Unicode codepoints, not bytes)
- **context_before**: Text immediately before the match
- **context_after**: Text immediately after the match (includes newline)
- **match_count**: Total number of matches returned

## CLI Reference

```
llmsearch [OPTIONS]

OPTIONS:
    -r, --root <DIR>          Root directory to search [default: .]
                              Uses .gitignore for skipping ignored files

    -p, --pattern <REGEX>     Regular expression pattern to search for
                              Uses Rust regex syntax (see link below)

    -g, --glob <GLOB>         File glob patterns [can be specified multiple times]
                              Examples: '*.rs', '*.toml', 'test_*.rs'
                              If not specified, searches all text files

    -l, --limit <N>           Maximum matches to return [default: 100]
                              Results sorted by file path, then byte offset

    --json                    Output JSON to stdout
                              Suppresses debug output and human-readable messages

    -h, --help                Print help
    -V, --version             Print version
```

**Regex syntax:** https://docs.rs/regex/latest/regex/#syntax

## Examples

### 1. Basic pattern search

Search for function definitions in current directory:

```bash
llmsearch -p 'fn \w+'
```

Output (human-readable mode):
```
Searching in "." for pattern: "fn \w+"
Found 3 matches

src/main.rs:1:1: fn main
src/cli.rs:4:1: fn parse_args
src/utils.rs:10:5: fn helper
```

### 2. Search specific directory with glob filter

Find error structs in Rust files:

```bash
llmsearch -r ./src -p 'struct.*Error' -g '*.rs'
```

### 3. JSON output for LLM consumption

Pipe to jq for processing:

```bash
llmsearch -p 'TODO:' --json | jq '.matches[] | {file, line, text}'
```

### 4. Multiple file types

Search across Rust and TOML files:

```bash
llmsearch -p 'version' -g '*.rs' -g '*.toml'
```

### 5. Limited results for large codebases

Get top 50 async functions:

```bash
llmsearch -p 'async fn' -l 50
```

### 6. Complex regex with capture groups

Find test functions with specific patterns:

```bash
llmsearch -p '#\[test\]\s+fn (test_|should_)'
```

## LLM Integration Notes

### Why deterministic ordering matters

When LLMs process search results, they need:
1. **Stable references** — Match IDs allow LLMs to reference specific results in follow-up queries
2. **Consistent ordering** — Same query always returns results in same order, enabling reliable indexing
3. **Audit trails** — Execution IDs let LLMs cache results or verify they're working with current data

Without these features, LLMs must:
- Parse unstructured text (error-prone)
- Maintain state in memory (lost after context compaction)
- Guess about result order (unreliable)
- Re-fetch files to verify changes (inefficient)

llmsearch eliminates these problems by providing structured, auditable output designed for AI workflows.

### Example workflow

```bash
# LLM searches for TODOs
RESULT=$(llmsearch -p 'TODO:' --json)

# LLM extracts execution_id for caching
EXEC_ID=$(echo $RESULT | jq -r '.execution_id')

# LLM processes matches deterministically
echo $RESULT | jq '.matches[] | select(.file | endswith(".rs"))'
```

## .gitignore Integration

llmsearch automatically respects `.gitignore` files in your project. Ignored files and directories are skipped during search, just like with `git ls-files`.

To disable this behavior, search from a directory without a `.gitignore`, or modify your ignore rules.

## License

MIT

## Contributing

This is the first of 4 planned LLM-native tools. Each tool lives in its own repository to keep codebases small, focused, and easy to maintain.

Related tools (planned):
- llm-ast-search: AST-aware code navigation
- llm-transform: Structured code modification
- llm-discover: Project structure analysis
