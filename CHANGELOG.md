# Changelog

All notable changes to llmsearch are documented in this file.

## [1.0.0] - 2026-01-16

### Added
- Regex pattern matching with byte offset tracking
- Line/column number calculation (UTF-8 aware)
- JSON output with execution_id and match_id (UUID v4)
- Context extraction (before/after matched text)
- File glob filtering (multiple patterns supported)
- Result limiting (default: 100)
- .gitignore-aware file walking
- Text file detection (skips binary files)
- Deterministic result ordering (file path, then byte offset)

### Testing
- 29 tests: 19 unit, 6 integration, 4 determinism
- All tests passing on Linux

### Known Issues
- Linux-only, not tested on other platforms
- Not published to crates.io
- Learning project, not intended for production use
