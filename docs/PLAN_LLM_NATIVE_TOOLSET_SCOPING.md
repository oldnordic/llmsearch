# Plan: LLM-Native Toolset Scoping + GLM Scaffolding

**Status**: Handoff notes
**Date**: 2026-01-10

## Summary

Goal: build a standalone Rust toolset (CLI + JSON) for LLM-native workflows, then integrate into OdinCode. Use GLM for scaffolding to optimize budget; finish/harden later.

## Recommended Build Order

1. `llm-search` (structured ripgrep)
2. `llm-ast-search`
3. `llm-transform` (span-safe edits)
4. `llm-discover` (fd/fzf)

## GLM Scaffolding Tasks (Best ROI)

1. Create a new workspace repo `llm-native-tools` with 4 crates:
   - `llm-search`
   - `llm-ast-search`
   - `llm-transform`
   - `llm-discover`
2. Implement **full CLI + JSON output** for `llm-search` only.
3. Stub the other three:
   - CLI arg parsing (clap)
   - JSON schema structs (serde)
   - Return “not implemented” errors

## Inputs to Give GLM

- `docs/LLM_NATIVE_TOOLING_IDEAS.md` (CLI + JSON specs)
- Supported languages (match Magellan):
  - Rust, C/C++, Java, JS/TS, Python
- Deterministic rules:
  - Sorted output, stable ordering, explicit spans
- Suggested Rust crates:
  - `ignore`, `regex`, `tree-sitter`, `serde`, `serde_json`, `blake3`, `similar`, `skim`

## Follow-up Work (after GLM scaffolding)

- Finish `llm-search` (edge cases + tests)
- Implement `llm-ast-search` (tree-sitter + queries)
- Implement `llm-transform` (checksums + reverse-order edits)
- Implement `llm-discover` (deterministic scoring + stable sort)
- Integrate into OdinCode tool mapper + tool lists

## OdinCode Integration (later)

Add tools in OdinCode:
- `llm_search`, `llm_ast_search`, `llm_transform`, `llm_discover`
- Map in `src/execution_engine/tool_mapper/mod.rs`
- Classify in `src/execution_engine/tool_lists.rs`
- Log structured outputs in `execution_log.db`
