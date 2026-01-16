# Phase 03-02 Summary: Search file contents with regex and collect byte offsets

**Status:** ✅ Completed
**Date:** 2026-01-16
**Tasks:** 1/1

## Objective

Search file contents with regex and collect byte offsets. Iterate through files, read their contents, and find all regex matches with their byte positions. Byte offsets are critical for later line/column calculation.

## Implementation

### Task 1: Create search function with byte offset tracking

**File modified:** `src/main.rs`

**Changes:**

1. **Added Match struct:**
   ```rust
   struct Match {
       file: String,
       byte_start: usize,
       byte_end: usize,
   }
   ```

2. **Implemented search_files function:**
   ```rust
   fn search_files(files: &[String], regex: &Regex) -> Vec<Match> {
       let mut matches = Vec::new();

       for file_path in files {
           match std::fs::read_to_string(file_path) {
               Ok(content) => {
                   for mat in regex.find_iter(&content) {
                       matches.push(Match {
                           file: file_path.clone(),
                           byte_start: mat.start(),
                           byte_end: mat.end(),
                       });
                   }
               }
               Err(e) => {
                   eprintln!("Warning: Could not read {}: {}", file_path, e);
               }
           }
       }

       matches
   }
   ```

3. **Updated main function:**
   - Removed old placeholder: `let _matches: Vec<String> = vec![];`
   - Added search call:
     ```rust
     let matches = search_files(&files, &regex);
     eprintln!("Found {} matches", matches.len());
     ```

**Key design decisions:**
- Match struct holds file path and byte positions (simple foundation for expansion)
- `regex.find_iter()` returns all non-overlapping matches with byte offsets
- `byte_start` and `byte_end` are byte positions (NOT character positions)
- `std::fs::read_to_string()` handles UTF-8 decoding automatically
- Graceful error handling with `eprintln` for unreadable files (no crashes)
- Match count printed to stderr for user feedback
- Match struct fields are currently unused (expected - will be used in phase 4)

## Verification

All criteria met:
- ✅ `cargo build` succeeds (only expected dead_code warnings)
- ✅ `cargo run -- --pattern "fn main" --root src` finds 1 match
- ✅ `cargo run -- --pattern "use" --root src` finds 5 matches
- ✅ `cargo run -- --pattern "fn " --root src` finds 4 matches
- ✅ `cargo run -- --pattern "Match" --root src` finds 3 matches
- ✅ Byte offsets tracked correctly (grep -b confirms offset 3732 for "fn main")
- ✅ Files with read errors skipped gracefully (tested with /nonexistent)

**Test output:**
```bash
$ cargo run -- --pattern "fn main" --root src
Found 2 files
Pattern compiled successfully
Found 1 matches

$ cargo run -- --pattern "use" --root src
Found 2 files
Pattern compiled successfully
Found 5 matches

$ cargo run -- --pattern "fn " --root src
Found 2 files
Pattern compiled successfully
Found 4 matches
```

## Output

Vector of Match objects containing:
- File path for each match
- Byte start position (offset from file beginning)
- Byte end position (offset from file beginning)

The search functionality now finds all regex matches across all files with accurate byte position tracking. These byte offsets will be converted to line/column numbers in phase 4.

## Next Steps

Plan 03-03 (next phase) will:
- Convert byte offsets to line/column positions
- Handle multi-byte UTF-8 characters correctly
- Add line and column fields to Match struct

## Technical Notes

**Byte vs character positions:**
- Rust's regex.find_iter() returns byte offsets, not character offsets
- This is correct for our use case - we need byte positions for file seeking
- UTF-8 handling: read_to_string() validates UTF-8, find_iter() works on &str
- Multi-byte characters will be handled correctly in phase 4 line/column conversion

**Performance considerations:**
- All file contents read into memory (acceptable for codebase search use case)
- Regex engine is highly optimized (Rust regex crate uses finite automata)
- Linear scan through each file - O(total_file_size) complexity
