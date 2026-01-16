# llmsearch v1.0.0 Release

**Release Date:** 2026-01-16

## Overview

llmsearch is an LLM-native code search tool that provides deterministic, structured JSON output designed for AI agent workflows. Unlike traditional search tools that produce unstructured text, llmsearch delivers queryable results with byte offsets, line/column numbers, execution IDs, and consistent ordering.

## What's Included

This release completes all 10 development phases:

- Phase 1: Project scaffolding and CLI structure
- Phase 2: Gitignore-aware file walking
- Phase 3: Pattern matching with regex
- Phase 4: Line/column calculation with UTF-8 support
- Phase 5: Context extraction
- Phase 6: JSON output structure
- Phase 7: Integration and optimization
- Phase 8: CLI polish and validation
- Phase 9: Comprehensive testing (29 tests)
- Phase 10: Documentation and release preparation

## Features

- **Deterministic Output:** Matches sorted by file path and byte offset for consistency
- **LLM-Optimized JSON:** Structured output designed for parsing by AI agents
- **Execution Tracking:** UUID v4 execution IDs for auditability
- **Context Windows:** Configurable before/after context for each match
- **Gitignore Integration:** Automatically respects .gitignore patterns
- **Glob Filtering:** Multiple file pattern support (e.g., `*.rs`, `*.toml`)
- **UTF-8 Support:** Correctly handles multi-byte Unicode characters
- **Binary Detection:** Automatically skips binary files
- **Comprehensive Testing:** 29 tests covering unit, integration, and determinism

## Installation

### From Source

```bash
cargo install --path .
```

### From crates.io (when published)

```bash
cargo install llmsearch
```

## Usage

```bash
# Search for pattern in current directory
llmsearch -p 'fn main'

# Search with glob filter and JSON output
llmsearch -p 'TODO:' -g '*.rs' --json

# Search in specific directory with limit
llmsearch -r ./src -p 'Error' -l 50
```

## Output Format

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

- **Platform:** Linux-only (uses platform-specific file system features)
- **File Types:** Text files only (automatically detects and skips binary files)
- **Regex:** Uses Rust regex syntax (see: https://docs.rs/regex/latest/regex/#syntax)

## System Requirements

- Rust 1.70+ (edition 2021)
- Linux operating system
- 50MB disk space for release build

## Testing

All 29 tests pass:

- 19 unit tests (binary detection, line indexing, glob filtering, context extraction)
- 6 integration tests (CLI behavior, error handling, validation)
- 4 determinism tests (output consistency, UUID uniqueness, limit functionality)

Run tests:
```bash
cargo test
```

## Future Work

Potential enhancements for future releases:

- Windows and macOS platform support
- Case-insensitive search option
- Invert match functionality
- Custom context window sizes
- Parallel file processing
- Binary file search modes
- Performance profiling for large codebases

## License

MIT License - See LICENSE file for details

## Repository

https://github.com/yourusername/llmsearch

---

**Version:** 1.0.0
**Build:** Tested with Rust 2021 edition
**Status:** Production-ready
