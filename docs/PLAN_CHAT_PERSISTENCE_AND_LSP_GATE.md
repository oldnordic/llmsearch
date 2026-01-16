# Implementation Plan: LSP Feedback Gate + Chat Context Persistence

**Status**: Planning doc for handoff
**Date**: 2026-01-10
**Audience**: Next LLM / maintainer

This document captures the concrete steps, target files, and design decisions so another LLM can implement without guessing.

## Goals

1. **LSP Feedback Gate**: After any mutation tool runs, LSP validation is mandatory. The LLM cannot proceed while errors exist.
2. **Full Chat Persistence**: Store all chat frames (user/assistant/tool) in `execution_log.db` with timestamps and session IDs.
3. **Compaction Summary Reinjection**: After auto-compaction, inject a deterministic summary + honesty check so the LLM re-queries facts instead of guessing.

## GLM-4.7 Notes (for implementer context)

Sources (quick web check):
- Z.AI developer docs: http://docs.z.ai/guides/llm/glm-4.7
- Third-party analysis (use with caution): http://llm-stats.com/blog/research/glm-4.7-launch

Observed capabilities in Z.AI docs:
- Tool invocation + function calling support.
- Structured output support.
- Context caching.
- "Thinking mode" / interleaved reasoning support.

No official weaknesses are listed in the docs. Treat all weaknesses as **standard LLM risks**:
- Tool-call formatting drift under long context.
- Overconfidence without validation.
- Hallucinated identifiers or file paths.

This plan’s gate + persistence + compaction summary exists to neutralize those risks regardless of model.

## Non-Goals

- No new external database files.
- No semantic inference beyond explicit text and tool facts.
- No changes to the Magellan DB or Splice behavior in this phase.

## ADRs

- LSP Gate: `docs/ADR_LSP_FEEDBACK_GATE.md`
- Chat Context Persistence: `docs/ADR_CHAT_CONTEXT_PERSISTENCE.md`

## Design Summary

### A) DB Schema (execution_log.db)

Add tables:

1. `chat_sessions`
   - `session_id TEXT PRIMARY KEY NOT NULL`
   - `created_at INTEGER NOT NULL`
   - `project_root TEXT`
   - `llm_provider TEXT`
   - `model TEXT`

2. `chat_frames`
   - `id INTEGER PRIMARY KEY AUTOINCREMENT`
   - `session_id TEXT NOT NULL`
   - `timestamp INTEGER NOT NULL`
   - `role TEXT NOT NULL` (`user|assistant|tool`)
   - `content TEXT NOT NULL`
   - `tool_name TEXT` (nullable)
   - `execution_id TEXT` (nullable, links to `executions.id`)
   - `compacted BOOLEAN NOT NULL DEFAULT 0`
   - `sequence_index INTEGER NOT NULL`
   - `FOREIGN KEY (session_id) REFERENCES chat_sessions(session_id)`

3. `chat_compactions`
   - `id INTEGER PRIMARY KEY AUTOINCREMENT`
   - `session_id TEXT NOT NULL`
   - `timestamp INTEGER NOT NULL`
   - `compacted_count INTEGER NOT NULL`
   - `retained_count INTEGER NOT NULL`
   - `summary TEXT NOT NULL`
   - `resume_cursor TEXT`
   - `FOREIGN KEY (session_id) REFERENCES chat_sessions(session_id)`

Indexes:
- `idx_chat_frames_session_timestamp` on `(session_id, timestamp)`
- `idx_chat_frames_session_seq` on `(session_id, sequence_index)`
- `idx_chat_frames_execution_id` on `(execution_id)`
- `idx_chat_compactions_session_timestamp` on `(session_id, timestamp)`

### B) Summary Format (deterministic)

```
=== COMPACTION SUMMARY ===
Goal: ...
Current task: ...
Files modified: [sorted list]
Files mentioned: [sorted list]
Files pending: ...
Key tool executions: [exec_id1, exec_id2, ...]
BE HONEST: DO YOU HAVE ENOUGH INFORMATION TO CONTINUE?
If NO: query past context + tool results before continuing.
If YES: proceed.
```

Rules:
- **Goal**: latest user directive; else `(not explicitly stated)`.
- **Current task**: latest assistant “Plan/Next/I will”; else `(not captured)`.
- **Files modified**: mutation tools only (`file_write`, `file_edit`, `file_create`, `splice_patch`, `splice_plan`).
- **Files mentioned**: parse file-like tokens in user/assistant text.
- **Files pending**: only if explicitly listed.
- **Key tool executions**: last N tool execution IDs + all failures in compaction window.

### C) LSP Feedback Gate

After any mutation tool:
- Run language-appropriate validation (Rust: `cargo check`; other languages via configured validator).
- If errors, summarize and **block** next LLM step until fixed.
- Log diagnostics + summary in execution DB.

## Target Files (by change area)

### 1) DB Schema and API

- `src/execution_tools/db_schema.rs`  
  Add `chat_sessions`, `chat_frames`, `chat_compactions` + indexes.
- `src/execution_tools/db.rs`  
  Add API methods: `record_chat_session`, `record_chat_frame`, `record_chat_compaction`, `query_chat_frames`.

### 2) FrameStack persistence

- `src/llm/frame_stack.rs`  
  - Track `sequence_index`.
  - On `add_user`, `add_assistant`, `add_tool_result` call `ExecutionDb::record_chat_frame`.
  - After `auto_compact_if_needed`, generate summary and store `chat_compactions`.
  - Inject summary as a new frame (pseudo-tool `compaction_summary`).

### 3) Compaction summary generation

- `src/llm/compaction.rs`  
  Add `generate_compaction_summary_block(...)` implementing the deterministic rules.

### 4) Retrieval tool

- `src/execution_engine/tool_mapper/mod.rs`  
  Add `chat_context_query` mapping.
- `src/execution_engine/tool_mapper/memory_ops.rs` (or new module)  
  Implement the query tool using `ExecutionDb::query_chat_frames`.
- `src/execution_engine/tool_lists.rs`  
  Add `chat_context_query` to `AUTO_TOOLS`.

### 5) LSP feedback gate

Choose integration point:
- **Preferred**: `src/llm/chat_loop/event_handler.rs`  
  After any mutation tool result, run `lsp_check` automatically and insert errors as ToolResult.
- Ensure this applies to GATED tools (`file_write`, etc.) by adding a post‑approval validation step.

### 6) Prompt updates

- `src/llm/contracts/prompts.rs`  
  - Mention `chat_context_query`.
  - Emphasize honesty gate after compaction.

## Tests (minimal)

- `tests/chat_context_persistence_tests.rs`  
  - Insert session + frames, query back, verify ordering.
  - Verify execution_id linkage stored.
- `tests/compaction_summary_tests.rs`  
  - Given synthetic frames, verify summary fields and honesty block.
- `tests/lsp_gate_tests.rs`  
  - After a mutation tool, ensure LSP is invoked and failure blocks progress (can be a unit test on gating logic).

## Handoff Notes

- No destructive commands.
- Maintain 300 LOC/module rule; split if needed.
- Use deterministic sorting and explicit fields; avoid heuristics.
- Keep `execution_log.db` the single source for tool logs and chat context.
