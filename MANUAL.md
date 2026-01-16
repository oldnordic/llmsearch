# llmsearch(1)

## NAME

llmsearch - a simple code search tool with JSON output

## SYNOPSIS

```
llmsearch [OPTIONS]
```

## DESCRIPTION

Searches text files recursively using regex patterns. Outputs results in JSON format with byte offsets, line/column numbers, and context.

## OPTIONS

| Option | Description |
|--------|-------------|
| `-r, --root <DIR>` | Root directory to search (default: current directory). Respects .gitignore files. |
| `-p, --pattern <REGEX>` | Regex pattern to search for (required). Uses Rust regex syntax. |
| `-g, --glob <GLOB>` | File glob filter. Can be specified multiple times. Example: `*.rs`, `*.toml`. |
| `-l, --limit <N>` | Maximum matches to return (default: 100). Results are sorted before limiting. |
| `--json` | Output JSON to stdout. Suppresses debug output. |
| `-h, --help` | Print help message. |

## EXIT STATUS

- **0** - Success (even if no matches found)
- **1** - Error (invalid pattern, nonexistent directory, empty pattern, limit <= 0)

## JSON OUTPUT FORMAT

```json
{
  "execution_id": "uuid-v4",
  "pattern": "searched_pattern",
  "matches": [
    {
      "match_id": "uuid-v4",
      "file": "relative/path/to/file",
      "byte_start": 0,
      "byte_end": 10,
      "matched_text": "matched string",
      "line_number": 1,
      "column_number": 1,
      "context_before": "before",
      "context_after": "after"
    }
  ],
  "match_count": 1
}
```

### Field Descriptions

- `execution_id` - UUID v4 identifying this search run
- `pattern` - The regex pattern that was searched
- `matches` - Array of match objects, sorted by file path then byte_start
- `match_id` - UUID v4 for each match
- `file` - Relative path from search root
- `byte_start` / `byte_end` - UTF-8 byte offsets in the file
- `matched_text` - The actual matched text
- `line_number` - 1-indexed line number
- `column_number` - 1-indexed column (Unicode codepoints)
- `context_before` - Text immediately before match
- `context_after` - Text immediately after match
- `match_count` - Total matches returned

## EXAMPLES

Search for "main" in current directory:
```bash
llmsearch -p "main"
```

Search specific directory with JSON output:
```bash
llmsearch -r ./src -p "struct" --json
```

Filter by file type:
```bash
llmsearch -p "test" -g "*.rs"
```

Multiple file types:
```bash
llmsearch -p "version" -g "*.rs" -g "*.toml"
```

Limit results:
```bash
llmsearch -p "async" -l 20
```

## NOTES

- Only searches UTF-8 text files (skips binary files)
- Results are sorted deterministically: by file path, then byte offset
- Linux-only, not tested on other platforms
- Respects .gitignore files when walking directories

## BUGS

Learning project - not production-ready. Report issues at GitHub.

## SEE ALSO

ripgrep(1), grep(1)

## LICENSE

GPL-3.0-only
