# ADR: LSP Feedback Gate for Grounded Edits

**Status**: Proposed
**Date**: 2026-01-10
**Context**: OdinCode tool substrate enforces "zero guessing" via Magellan facts and Splice edits, but current LSP diagnostics are passive and do not actively gate LLM progress.

## Problem

LLMs often proceed based on prediction rather than verification. After a `file_write` or `splice` edit, the model can continue without acknowledging LSP/compiler errors. This creates drift (e.g., introducing a new identifier like `datetime` when `timedate` exists elsewhere) and floods context when raw diagnostics are pasted verbatim.

We need an active feedback loop where:
- Every write or splice action is followed by LSP validation.
- Failures are summarized and returned immediately to the LLM.
- Diagnostics and tool results are persisted so the LLM can query facts instead of guessing.

This aligns with the "factual feedback loop" and "zero guessing" mandates. See `docs/CORE_LOOP.md` and `docs/ODINCODE_WORKFLOW.md`.

## Decision

Implement an **LSP Feedback Gate**:

1. **Mandatory validation** after any mutation tool (`file_write`, `file_create`, `splice_patch`, `splice_plan`).
2. **Summarized diagnostics** returned to the LLM (concise, structured, de-duplicated).
3. **Persistence** of tool invocations and validation results in the execution database.
4. **No forward progress** for the LLM until the most recent mutation has a passing LSP check.

## Scope

- Applies to all language-aware projects; validation tool chosen per language or workspace config.
- Summary format must be deterministic and compact (no raw flood unless explicitly requested).
- Storage schema must capture:
  - tool name + args
  - execution id + timestamp
  - LSP diagnostics (file, line, code, message)
  - summary string for quick retrieval

## Alternatives Considered

1. **Passive LSP (status-only)**: leaves drift unchecked; does not enforce grounding.
2. **Raw diagnostic dump**: accurate but overwhelms context; harms usability.
3. **Human-in-the-loop only**: slows workflow; defeats automation goal.

## Consequences

- **Pros**:
  - Prevents "forward progress" without verified code state.
  - Reduces hallucinated identifiers and missing imports.
  - Enables factual querying from the DB instead of context flooding.
- **Cons**:
  - Adds latency after each edit.
  - Requires DB schema changes and migration discipline.
  - Requires robust summary logic to avoid losing critical details.

## Follow-up Work

- Identify LSP entry points and tool call flow using Magellan.
- Specify DB schema changes for tool call + diagnostic persistence.
- Implement deterministic summarization rules for diagnostics.
- Update documentation in `docs/TOOL_MANUALS.md` and relevant workflow guides.
