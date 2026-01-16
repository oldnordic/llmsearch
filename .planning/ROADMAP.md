# Roadmap: llmsearch

## Overview

Build an LLM-native code search tool (ripgrep replacement) from scratch. Starting with Rust project scaffolding, we'll implement file walking with .gitignore support, regex pattern matching with byte offset tracking, line/column calculation, context extraction, structured JSON output, and ensure deterministic ordering. The journey concludes with testing, documentation, and release packaging.

## Domain Expertise

None

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Project Scaffolding** - Cargo project setup with CLI argument structure (Complete: 2026-01-16)
- [x] **Phase 2: File Walking & Ignore** - Gitignore-aware directory traversal (Complete: 2026-01-16)
- [x] **Phase 3: Pattern Matching** - Regex search with byte offset tracking (Complete: 2026-01-16)
- [x] **Phase 4: Line/Column Calculation** - Convert byte offsets to line/col positions (Complete: 2026-01-16)
- [x] **Phase 5: Context Extraction** - Extract before/after context for each match (Complete: 2026-01-16)
- [x] **Phase 6: JSON Output Schema** - Structured output with execution_id (Complete: 2026-01-16)
- [ ] **Phase 7: Deterministic Ordering** - Sorted, limited, repeatable results
- [ ] **Phase 8: CLI Interface Polish** - Help text, error handling, --json flag
- [ ] **Phase 9: Testing & Validation** - Test suite for determinism and edge cases
- [ ] **Phase 10: Documentation & Release** - README, examples, packaging

## Phase Details

### Phase 1: Project Scaffolding
**Goal**: Cargo workspace with CLI argument parsing structure
**Depends on**: Nothing (first phase)
**Research**: Unlikely (standard Rust CLI patterns)
**Plans**: TBD

Plans:
- [x] 01-01: Initialize Cargo project with dependencies
- [x] 01-02: Define CLI argument structure with clap
- [x] 01-03: Create basic binary entry point

### Phase 2: File Walking & Ignore
**Goal**: Gitignore-aware directory traversal
**Depends on**: Phase 1
**Research**: Unlikely (ignore crate is well-established)
**Plans**: TBD

Plans:
- [x] 02-01: Integrate ignore crate for file walking
- [x] 02-02: Implement glob pattern filtering
- [x] 02-03: Handle UTF-8 text file detection

### Phase 3: Pattern Matching
**Goal**: Regex search with byte offset tracking
**Depends on**: Phase 2
**Research**: Unlikely (regex crate is standard)
**Plans**: TBD

Plans:
- [x] 03-01: Integrate regex crate
- [x] 03-02: Search file contents with byte offsets
- [x] 03-03: Track match start/end positions

### Phase 4: Line/Column Calculation
**Goal**: Convert byte offsets to line/col positions
**Depends on**: Phase 3
**Research**: Unlikely (byte-to-line-col is straightforward)
**Plans**: TBD

Plans:
- [x] 04-01: Build line index from file content
- [x] 04-02: Convert byte offsets to line/col
- [x] 04-03: Handle multi-byte UTF-8 characters correctly

### Phase 5: Context Extraction
**Goal**: Extract before/after context for each match
**Depends on**: Phase 4
**Research**: Unlikely (string slicing is standard)
**Plans**: TBD

Plans:
- [x] 05-01: Extract context before match
- [x] 05-02: Extract context after match
- [x] 05-03: Handle edge cases (start/end of file)

### Phase 6: JSON Output Schema
**Goal**: Structured output with execution_id and match_id
**Depends on**: Phase 5
**Research**: Unlikely (serde patterns are standard)
**Plans**: TBD

Plans:
- [x] 06-01: Define JSON output structures
- [x] 06-02: Generate UUID for execution_id
- [x] 06-03: Serialize matches to JSON format

### Phase 7: Deterministic Ordering
**Goal**: Sorted, limited, repeatable results
**Depends on**: Phase 6
**Research**: Unlikely (sorting is deterministic)
**Plans**: TBD

Plans:
- [ ] 07-01: Sort results by path then byte_start
- [ ] 07-02: Implement limit functionality
- [ ] 07-03: Verify ordering consistency across runs

### Phase 8: CLI Interface Polish
**Goal**: Help text, error handling, --json flag
**Depends on**: Phase 7
**Research**: Unlikely (CLI polish is internal)
**Plans**: TBD

Plans:
- [ ] 08-01: Add comprehensive help text
- [ ] 08-02: Implement --json flag
- [ ] 08-03: Add error handling for invalid inputs

### Phase 9: Testing & Validation
**Goal**: Test suite for determinism and edge cases
**Depends on**: Phase 8
**Research**: Unlikely (standard testing patterns)
**Plans**: TBD

Plans:
- [ ] 09-01: Unit tests for core functions
- [ ] 09-02: Integration tests for CLI
- [ ] 09-03: Determinism verification tests

### Phase 10: Documentation & Release
**Goal**: README, examples, packaging
**Depends on**: Phase 9
**Research**: Unlikely (documentation is internal)
**Plans**: TBD

Plans:
- [ ] 10-01: Write README with examples
- [ ] 10-02: Add example usage patterns
- [ ] 10-03: Package for release

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Project Scaffolding | 3/3 | Complete | 2026-01-16 |
| 2. File Walking & Ignore | 3/3 | Complete | 2026-01-16 |
| 3. Pattern Matching | 3/3 | Complete | 2026-01-16 |
| 4. Line/Column Calculation | 3/3 | Complete | 2026-01-16 |
| 5. Context Extraction | 3/3 | Complete | 2026-01-16 |
| 6. JSON Output Schema | 3/3 | Complete | 2026-01-16 |
| 7. Deterministic Ordering | 0/3 | Not started | - |
| 8. CLI Interface Polish | 0/3 | Not started | - |
| 9. Testing & Validation | 0/3 | Not started | - |
| 10. Documentation & Release | 0/3 | Not started | - |
