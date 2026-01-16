# ADR: LLM-Native Text Transformation Tools

**Status**: Proposed
**Date**: 2026-01-10
**Context**: Traditional text tools (sed/awk/grep) are optimized for human operators who can maintain long-term context. LLMs require grounded, structured facts to avoid drift after compaction. Magellan and Splice already provide LLM-native structure for codebases.

## Problem

LLMs operating on plain text tool output must parse unstructured streams and keep state in memory. After compaction, that state is lost, leading to guesswork and rework.

## Decision

Explore LLM-native text transformation tools that follow the same deterministic, auditable methodology as Magellan/Splice:

- **Structured inputs**: explicit file/symbol/span targets.
- **Structured outputs**: exact matches, spans, diffs, checksums.
- **Validation hooks**: pre/post checks and explicit error reporting.
- **Execution logging**: every run yields a durable execution ID.

## Scope

This is exploratory. It may be redundant with Splice, which already functions as an LLM-native transform tool for code symbols and spans. Any new tool should justify itself beyond Splice’s capabilities (e.g., non-code text, config formats, or multi-language plain text).

## Alternatives Considered

1. **Use sed/awk as-is**  
   Rejected: unstructured output and no grounding guarantees.
2. **Extend Splice only**  
   Viable for code; may not cover arbitrary text workflows.
3. **Create a narrow LLM-native transform API**  
   Favorable if it fills gaps outside Splice’s symbol-based edits.

## Consequences

- **Pros**: Grounded transformations, auditability, reduced drift.
- **Cons**: Overlap risk with Splice; more surface area to maintain.

## Follow-up Work

- Define where Splice is insufficient (non-code, config, docs).
- Draft a minimal API for deterministic text transforms.
- Prototype against one non-code use case.
