# LLM-Native Tooling Ideas (Rust-first)

**Status**: Draft concept
**Date**: 2026-01-10
**Goal**: Rebuild common CLI tooling so results are structured, auditable, and deterministic for LLM workflows.

## Design Principles

- **Structured output**: JSON rows with explicit fields.
- **Stable identifiers**: execution_id, match_id, span_id.
- **Span-aware**: byte offsets + line/col for every match.
- **Deterministic ordering**: sorted output; no env-dependent behavior.
- **Validation hooks**: checksums and pre/post verification.
- **Persistence**: every run logged with execution_id.

## Supported Languages (initial)

Match Magellan support:
- Rust (.rs)
- C/C++ (.c, .h, .cpp, .cc, .cxx, .hpp)
- Java (.java)
- JavaScript (.js, .mjs)
- TypeScript (.ts, .tsx)
- Python (.py)

## Rust Crates to Use

### File walking / ignore rules
- `ignore` (used by ripgrep; respects .gitignore)
- `walkdir` (simple recursive traversal when ignore is not needed)

### Matching / parsing
- `regex` (line/substring matching)
- `regex-automata` (faster DFA for large scans)
- `aho-corasick` (multi-pattern search)
- `tree-sitter` + language grammars (AST extraction)

### Output / serialization
- `serde`, `serde_json` (structured output)
- `bincode` (optional binary storage)

### Fuzzy search / ranking
- `skim` (fuzzy scoring; deterministic if inputs are fixed)

### Diffs / checksums
- `sha2` or `blake3` (checksums)
- `similar` (diffs)

## Tool Concepts

### 1) LLM-Search (ripgrep replacement)

**Input**:
```
{ "pattern": "...", "root": "...", "file_glob": ["**/*.rs"] }
```

**Output**:
```
[
  { "match_id": "...", "path": "src/lib.rs", "byte_start": 120,
    "byte_end": 132, "line": 10, "column": 4,
    "match": "foo", "context_before": "...", "context_after": "..." }
]
```

**Implementation**:
- Use `ignore` + `regex` to scan files.
- Convert byte offsets to line/col by precomputed line index.
- Sort by path then byte_start.

### 2) LLM-AST Search (ast-grep replacement)

**Input**:
```
{ "language": "rust", "query": "(function_item name: (identifier) @id)" }
```

**Output**:
```
[
  { "symbol": "foo", "kind": "Function",
    "path": "src/main.rs", "byte_start": 100, "byte_end": 200 }
]
```

**Implementation**:
- Use `tree-sitter` + language query files.
- Normalize symbol kinds (Function/Method/Class/etc).
- Optional: emit parent symbol / module path.

### 3) LLM-Transform (sed/awk replacement)

**Input**:
```
{ "edits": [
  { "path": "src/main.rs", "byte_start": 120, "byte_end": 132,
    "replacement": "bar", "checksum_before": "..." }
] }
```

**Output**:
```
{ "applied": 1, "skipped": 0, "errors": [] }
```

**Implementation**:
- Verify checksum before applying edits.
- Apply edits in reverse order by byte offset.
- Recompute checksum after.
- Fail if spans drift.

### 4) LLM-Discover (fd/fzf replacement)

**Input**:
```
{ "query": "vector", "root": ".", "limit": 20 }
```

**Output**:
```
[
  { "path": "src/vector.rs", "score": 0.91, "reason": "filename match" }
]
```

**Implementation**:
- Use `skim` for scoring.
- Ensure deterministic ordering by stable sort on score then path.

## Execution Logging

Every tool run returns:
- `execution_id` (UUID)
- `timestamp`
- `arguments_json`
- `stdout_json` or `structured_output`
- `error_message`

This keeps the LLM grounded even after compaction.

## Notes

Splice already covers symbol-level editing for code. The LLM-Transform tool is only useful for:
- Non-code text files
- Config formats not covered by Splice
- Bulk edits with strict checksums

## Concrete Specs (CLI + JSON)

Order of implementation:
1) `llm_search` (foundation for other tools)
2) `llm_ast_search`
3) `llm_transform`
4) `llm_discover`

### 1) llm_search (structured ripgrep)

**CLI**
```
llm-search --root <DIR> --pattern <REGEX> [--glob <GLOB>]... [--limit <N>] [--json]
```

**JSON Output**
```
{
  "execution_id": "...",
  "matches": [
    {
      "match_id": "...",
      "path": "src/main.rs",
      "byte_start": 120,
      "byte_end": 132,
      "line": 10,
      "column": 4,
      "match": "foo",
      "context_before": "...",
      "context_after": "..."
    }
  ]
}
```

**Determinism**
- Sort by `(path, byte_start)`.
- `limit` applied after sorting.

**Tests**
- Stable ordering across runs.
- Line/column accuracy on UTF-8.

### 2) llm_ast_search (structured AST query)

**CLI**
```
llm-ast-search --root <DIR> --language <LANG> --query <TS_QUERY> [--glob <GLOB>]... [--json]
```

**JSON Output**
```
{
  "execution_id": "...",
  "symbols": [
    {
      "symbol": "foo",
      "kind": "Function",
      "path": "src/main.rs",
      "byte_start": 100,
      "byte_end": 200,
      "line": 5,
      "column": 0
    }
  ]
}
```

**Determinism**
- Sort by `(path, byte_start)`.

**Tests**
- Query returns correct symbol kinds.
- Span matches actual source bytes.

### 3) llm_transform (span-safe text edits)

**CLI**
```
llm-transform --file <PATH> --edits <JSON_FILE> [--json]
```

**Edits JSON**
```
{
  "edits": [
    {
      "byte_start": 120,
      "byte_end": 132,
      "replacement": "bar",
      "checksum_before": "..."
    }
  ]
}
```

**JSON Output**
```
{
  "execution_id": "...",
  "applied": 1,
  "skipped": 0,
  "errors": [],
  "checksum_after": "..."
}
```

**Rules**
- Verify `checksum_before` before applying.
- Apply edits in reverse byte order.
- Fail if spans drift or checksum mismatch.

**Tests**
- Reject mismatched checksum.
- Multiple edits apply without offset drift.

### 4) llm_discover (structured fd/fzf)

**CLI**
```
llm-discover --root <DIR> --query <STRING> [--glob <GLOB>]... [--limit <N>] [--json]
```

**JSON Output**
```
{
  "execution_id": "...",
  "results": [
    {
      "path": "src/vector.rs",
      "score": 0.91,
      "reason": "filename match"
    }
  ]
}
```

**Determinism**
- Stable sort by `(score desc, path asc)`.

**Tests**
- Same inputs yield same order and scores.

## OdinCode Integration Plan (First Pass)

### Integration Model

- Each tool is a **self-contained crate** with a minimal CLI wrapper.
- OdinCode integrates via:
  1. Direct library calls (preferred).
  2. CLI invocation fallback (optional).
- All tool outputs are captured as structured JSON and logged in `execution_log.db`.

### Tool Registry Additions

Add to tool mapper with explicit names:
- `llm_search` (ripgrep replacement)
- `llm_ast_search` (ast-grep replacement)
- `llm_transform` (sed/awk replacement)
- `llm_discover` (fd/fzf replacement)

Each tool must return:
- `execution_id`
- `structured_output` (JSON)
- `diagnostics` (optional)

### Target OdinCode Modules

- `src/execution_engine/tool_mapper/mod.rs`  
  Add new tool mappings.
- `src/execution_engine/tool_lists.rs`  
  Classify as AUTO (read-only) or GATED (mutations).
- `src/execution_engine/output_kind.rs`  
  Add tool output kinds if needed (SearchResults, TransformResult, etc.).
- `src/execution_tools/db_schema.rs`  
  Ensure artifacts can store tool output JSON.

### Deterministic Output Rules

- Always sorted by `(path, byte_start)` or stable key.
- Always include byte spans and line/col.
- Always include a checksum if edits are applied.

### Platform Strategy

Phase 1: Linux + macOS
- POSIX file paths
- Rust crate dependency only (no system binaries)

Phase 2: Windows
- Path normalization
- UTF-16 edge cases
- Explicit test matrix for NTFS path behavior

### Acceptance Criteria

- All tools return structured JSON with stable ordering.
- Tool outputs are stored and retrievable via execution IDs.
- Mutation tools enforce checksums and refuse drifted spans.
