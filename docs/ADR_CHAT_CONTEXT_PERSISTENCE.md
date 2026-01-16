# ADR: Persist Full Chat Context + Compaction Summary Reinjection

**Status**: Proposed
**Date**: 2026-01-10
**Context**: OdinCode auto-compacts tool results in memory, but full chat context is not persisted. This forces the LLM to guess or rely on truncated context instead of querying authoritative history. Tool executions are already logged in `execution_log.db`, but full conversation history is not.

## Problem

- Compaction currently only marks tool results as compacted in-memory.
- After compaction, the LLM loses full conversation context unless it was manually retrieved.
- There is no database-backed audit trail of complete chat exchanges.
- The system lacks a consistent, enforced summary format that the LLM can use to resume without guessing.

## Decision

Introduce **full chat context persistence** in a dedicated database/table and **summary reinjection** after auto-compaction:

1. **Persist full chat frames** (user, assistant, tool result) with timestamp and session id.
2. **Persist tool results separately** in the existing `execution_log.db` (no change to its responsibility).
3. **Inject a compact summary** after compaction containing:
   - Goal
   - Current task
   - Files modified
   - Files pending
   - Key tool execution IDs to retrieve details
4. **Honesty gate** appended to the summary:
   ```
   BE HONEST: DO YOU HAVE ENOUGH INFORMATION TO CONTINUE?
   If NO: query past context and tool results before continuing.
   If YES: proceed.
   ```

## Scope

- The full chat context is stored in a **separate table or database** from tool execution logs.
- Summaries are deterministic and concise (no raw transcript dumps).
- Retrieval is explicit via a query tool (e.g., `chat_context_query`).

## Alternatives Considered

1. **Store full context in execution_log.db**  
   Rejected: mixes tool execution logs with chat transcript; harder to query and maintain.
2. **Do nothing; rely on in-memory FrameStack**  
   Rejected: compaction discards context; not auditable.
3. **Only store summaries**  
   Rejected: summaries can omit important details; not fully auditable.

## Consequences

- **Pros**:
  - Full audit trail of LLM activity.
  - Deterministic resumption after compaction.
  - LLM can query exact past context instead of guessing.
- **Cons**:
  - Additional DB schema + migration effort.
  - Larger storage footprint.
  - Requires consistent summary formatting.

## Follow-up Work

- Design schema for chat context storage (session, timestamp, role, content).
- Define summary format and enforce it at compaction time.
- Implement query tool for past chat context retrieval.
- Update docs and prompts to point the LLM to retrieval tools.
