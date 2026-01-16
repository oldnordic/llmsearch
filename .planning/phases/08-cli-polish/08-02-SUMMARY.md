---
phase: 08-cli-polish
plan: 02
subsystem: cli
tags: [json-output, stdout-routing, conditional-output]

# Dependency graph
requires:
  - phase: 06-json-output
    provides: JSON serialization and SearchOutput struct
  - phase: 07-deterministic-ordering
    provides: Sorted, limited matches
provides:
  - JSON output to stdout when --json flag is set
  - Compact JSON format for LLM consumption
  - Clean stdout (no debug output) in JSON mode
affects: [08-03-error-handling]

# Tech tracking
tech-stack:
  added: []
  patterns: [conditional output routing, clean JSON mode]

key-files:
  created: []
  modified: [src/main.rs]

key-decisions:
  - "Use stdout for JSON, stderr for debug/errors (separation of concerns)"
  - "Compact JSON (not pretty) for --json mode (LLM consumer)"
  - "Suppress all debug output in JSON mode (clean pipeable output)"

patterns-established:
  - "Pattern: Output routing based on mode flag (human vs machine)"
  - "Pattern: Keep stdout clean for data, stderr for metadata"

issues-created: []

# Metrics
duration: 2min
completed: 2026-01-16
---

# Phase 8 Plan 2: JSON Flag Implementation Summary

**Implemented --json flag with stdout routing, compact JSON format, and suppressed debug output**

## Performance

- **Duration:** 2 min
- **Started:** 2026-01-16T14:24:00Z (estimated)
- **Completed:** 2026-01-16T14:26:00Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

1. **Task 1: Route JSON to stdout when --json is enabled**
   - Implemented conditional output routing: `if args.json`
   - JSON mode: `serde_json::to_string()` → stdout (compact)
   - Debug mode: `serde_json::to_string_pretty()` → stderr (pretty)
   - Error messages always route to stderr

2. **Task 2: Suppress debug output in JSON mode**
   - Wrapped all debug `eprintln!` statements with `if !args.json`
   - 9 debug output locations now conditional
   - JSON mode produces clean, pipeable output

## Task Commits

Each task was committed atomically:

1. **Task 1: Route JSON to stdout** - `d63b072` (feat)
2. **Task 2: Suppress debug output** - `16a0928` (feat)

## Files Created/Modified

- `src/main.rs` - Added conditional JSON output routing
  - Modified JSON serialization section (lines ~254-275)
  - Wrapped 9 debug `eprintln!` statements with `if !args.json`
  - Execution ID, file count, pattern, matches, sorting, limit all conditional
  - Match debug loop, UTF-8 messages, scaffolding messages all conditional

## Deviations from Plan

None - plan executed exactly as specified.

## Issues Encountered

None.

## Next Phase Readiness

- JSON flag fully functional
- Clean stdout for LLM consumption
- Debug mode remains for human troubleshooting
- Ready for error handling improvements in 08-03

---
*Phase: 08-cli-polish*
*Completed: 2026-01-16*
