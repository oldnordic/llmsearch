# llmsearch

## What This Is

A standalone CLI tool that provides LLM-native code search. Think of it as ripgrep with structured JSON output designed for AI agent workflows. Every match includes byte offsets, line/column numbers, execution IDs for auditability, and deterministic ordering so LLMs get consistent, queryable results instead of unstructured text streams.

## Core Value

Deterministic, structured search output that LLMs can rely on without guessing. Every run returns the same ordered results with exact locations, spans, and an execution ID for retrieval.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Regex pattern matching with file paths and byte offsets
- [ ] Line/column calculation for every match
- [ ] Execution ID (UUID) for every run
- [ ] Context before/after each match (configurable lines)
- [ ] Deterministic ordering (sorted by path, then byte_start)
- [ ] File glob filtering (`--glob` multiple)
- [ ] Result limiting (`--limit`)
- [ ] .gitignore-aware file walking
- [ ] JSON output mode (`--json`)
- [ ] UTF-8 text file support

### Out of Scope

- Library API — CLI tool only, JSON output via stdout
- Binary file search — UTF-8 text files only
- Other LLM tools (llm-ast-search, llm-transform, llm-discover) — those are separate standalone projects
- Windows/macOS support — Linux-first

## Context

This is the first of 4 planned LLM-native tools. Each tool lives in its own repository to keep codebases small, focused, and easy to maintain. The design draws from `docs/LLM_NATIVE_TOOLING_IDEAS.md` which specifies that LLMs require structured, auditable tool outputs with stable identifiers and deterministic behavior.

Traditional CLI tools (ripgrep, grep, sed, awk) output unstructured text designed for human operators. LLMs consuming this output must parse streams, maintain state in memory, and lose that state after context compaction — leading to guesswork. llmsearch fixes this by returning structured JSON with execution IDs that can be queried later.

## Constraints

- **Platform**: Linux-only — simplifies initial development, avoids cross-platform edge cases
- **Language**: Rust — performance, safety, ecosystem for CLI tools
- **Output**: JSON-only mode — no human-readable formatting needed (LLM consumer)
- **Determinism**: Sorted output required — same inputs must yield identical ordered results

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Standalone tool (not monorepo) | Keep each tool small, focused, independently maintainable | — Pending |
| Complete spec in v1 | LLM consumers need full structured data, no partial implementations | — Pending |
| Linux-first | Avoid cross-platform complexity in initial release | — Pending |
| Flexible crate selection | Docs suggest crates but better alternatives may exist | — Pending |

## JSON Output Schema

```json
{
  "execution_id": "uuid-v4",
  "matches": [
    {
      "match_id": "uuid-v4",
      "path": "src/main.rs",
      "byte_start": 120,
      "byte_end": 132,
      "line": 10,
      "column": 4,
      "match": "foo",
      "context_before": "let x = ",
      "context_after": ";\n"
    }
  ]
}
```

## CLI Interface

```
llm-search --root <DIR> --pattern <REGEX> [--glob <GLOB>]... [--limit <N>] [--json]
```

---
*Last updated: 2026-01-16 after initialization*
