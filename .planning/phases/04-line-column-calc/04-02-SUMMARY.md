---
phase: 04-line-column-calc
plan: 02
subsystem: search-indexing
tags: [rust, column-numbers, utf-8, unicode-codepoints, grep-format]

# Dependency graph
requires:
  - phase: 04-line-column-calc-01
    provides: build_line_index and byte_to_line functions
provides:
  - Match struct with line_number and column_number fields
  - byte_to_column function for Unicode-aware column calculation
  - Complete line/col position tracking for all matches
affects: [05-context-extraction, 06-json-output]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - chars().count() for Unicode codepoint counting
    - Line index rebuilt per file for accurate position mapping
    - 1-indexed line/col for human readability

key-files:
  created: []
  modified:
    - src/main.rs: Added line/col fields to Match, byte_to_column function, updated search_files

key-decisions:
  - "Column numbers count Unicode codepoints not bytes - handles multi-byte UTF-8"
  - "Line index rebuilt per file - each file has different line structure"
  - "1-indexed columns (col 1 not col 0) - matches standard grep format"

patterns-established:
  - "Pattern: Position conversion pipeline - byte offset -> line number -> column number"
  - "Pattern: Unicode safety - use chars() for counting, not bytes()"

issues-created: []

# Metrics
duration: 2min
completed: 2026-01-16
---

# Phase 4 Plan 2: Line/Column Integration Summary

**Match struct extended with line_number and column_number fields using Unicode-aware byte-to-column conversion**

## Performance

- **Duration:** 2 min
- **Started:** 2026-01-16T02:12:00Z
- **Completed:** 2026-01-16T02:14:10Z
- **Tasks:** 1/1
- **Files modified:** 1

## Accomplishments
- Extended Match struct with line_number and column_number fields
- Implemented byte_to_column() function using chars().count() for Unicode support
- Integrated line/col calculation into search_files() with per-file line indexing
- Updated debug output to show file:line:col format (standard grep format)
- Verified UTF-8 multi-byte character handling (e.g., Chinese characters counted as single columns)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add line/col to Match struct and integrate calculation** - `561186d` (feat)

**Plan metadata:** N/A (will be in docs commit)

## Files Created/Modified
- `src/main.rs` - Extended Match struct with line/col fields, added byte_to_column function, updated search_files to calculate positions

## Decisions Made
- Column numbers count Unicode codepoints (not bytes) for proper multi-byte UTF-8 handling
- Line index rebuilt per file since each file has different line structure
- Using chars().count() correctly handles multi-byte UTF-8 sequences as single columns

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## Next Phase Readiness
- Line/column calculation complete for all matches
- Match struct now has all position fields needed for context extraction
- Ready for 05-01 (context extraction) which will use line/col to extract before/after text
- No blockers or concerns

---
*Phase: 04-line-column-calc*
*Completed: 2026-01-16*
