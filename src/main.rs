mod cli;

use clap::Parser;
use ignore::Walk;
use regex::Regex;
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Match {
    match_id: String,
    file: String,
    byte_start: usize,
    byte_end: usize,
    matched_text: String,
    line_number: usize,
    column_number: usize,
    context_before: String,
    context_after: String,
}

#[derive(Serialize)]
struct SearchOutput {
    execution_id: String,
    pattern: String,
    matches: Vec<Match>,
    match_count: usize,
}

fn is_text_file(path: &Path) -> bool {
    // Try to read first few bytes to check for binary content
    match std::fs::read(path) {
        Ok(contents) => {
            // Check first 8KB for null bytes (common in binary files)
            let sample_size = 8192.min(contents.len());
            let sample = &contents[..sample_size];

            // If we find a null byte, it's likely binary
            if sample.contains(&0u8) {
                return false;
            }

            // Check for UTF-8 validity
            std::str::from_utf8(sample).is_ok()
        }
        Err(_) => false, // Can't read file, skip it
    }
}

fn walk_files(root: &str, globs: &[String]) -> Vec<String> {
    let walker = Walk::new(root);

    let mut files = Vec::new();
    for result in walker {
        match result {
            Ok(entry) => {
                if let Some(path) = entry.path().to_str() {
                    // Only include files, not directories
                    if !entry.file_type().map_or(false, |ft| ft.is_file()) {
                        continue;
                    }

                    // Skip binary files
                    if !is_text_file(entry.path()) {
                        continue;
                    }

                    // Apply glob filtering if patterns provided
                    if !globs.is_empty() {
                        let filename = entry.file_name().to_string_lossy();
                        let matches: bool = globs.iter().any(|g| {
                            // Simple glob matching - check if filename matches pattern
                            // Convert "*.rs" style glob to a simple check
                            if g.starts_with("*.") {
                                let ext = &g[2..];
                                filename.ends_with(ext)
                            } else if g.contains('*') {
                                // Simple wildcard match
                                let pattern = g.replace(".", "\\.").replace("*", ".*");
                                if let Ok(re) = regex::Regex::new(&pattern) {
                                    re.is_match(&filename)
                                } else {
                                    false
                                }
                            } else {
                                // Exact match
                                filename == *g
                            }
                        });

                        if !matches {
                            continue; // Skip files not matching any glob
                        }
                    }

                    files.push(path.to_string());
                }
            }
            Err(err) => {
                eprintln!("Warning: {}", err);
            }
        }
    }
    files
}

fn search_files(files: &[String], regex: &Regex) -> Vec<Match> {
    let mut matches = Vec::new();

    for file_path in files {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                // Build line index for this file
                let line_index = build_line_index(&content);

                for mat in regex.find_iter(&content) {
                    let byte_start = mat.start();
                    let byte_end = mat.end();
                    let line_num = byte_to_line(byte_start, &line_index);
                    let line_start_byte = line_index[line_num - 1];
                    let col_num = byte_to_column(byte_start, line_start_byte, &content);

                    // Extract context before match (up to 100 chars or line start)
                    let context_start = line_start_byte.max(byte_start.saturating_sub(100));
                    let context_before = content[context_start..byte_start].to_string();

                    // Extract context after match (up to 100 chars or line end)
                    let remaining_line = &content[byte_end..];
                    let line_end_offset = remaining_line.find('\n').unwrap_or(remaining_line.len());
                    let context_end = byte_end + line_end_offset.min(100);
                    let context_after = content[byte_end..context_end.min(content.len())].to_string();

                    matches.push(Match {
                        match_id: Uuid::new_v4().to_string(),
                        file: file_path.clone(),
                        byte_start,
                        byte_end,
                        matched_text: mat.as_str().to_string(),
                        line_number: line_num,
                        column_number: col_num,
                        context_before,
                        context_after,
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

/// Builds an index of newline positions for byte-to-line conversion
/// Returns a Vec where index[i] = byte offset of line i (0-indexed)
fn build_line_index(content: &str) -> Vec<usize> {
    let mut line_starts = vec![0]; // Line 0 starts at byte 0

    for (byte_offset, ch) in content.char_indices() {
        if ch == '\n' {
            // Next line starts after this newline
            line_starts.push(byte_offset + 1);
        }
    }

    line_starts
}

/// Given a byte offset and line index, returns the line number (1-indexed)
fn byte_to_line(byte_offset: usize, line_index: &[usize]) -> usize {
    // Find the last line that starts at or before our byte offset
    match line_index.binary_search(&byte_offset) {
        Ok(i) => i + 1,      // Exact match - byte is at line start
        Err(i) => i,         // byte is in line i (i lines start before it)
    }
}

/// Given a byte offset and line start, returns the column (1-indexed)
/// Counts Unicode codepoints, not bytes, for correct column numbers
fn byte_to_column(byte_offset: usize, line_start: usize, content: &str) -> usize {
    // Get the slice from line start to our byte offset
    let line_prefix = &content[line_start..byte_offset];

    // Count characters (codepoints), not bytes
    // This handles multi-byte UTF-8 correctly
    line_prefix.chars().count() + 1  // +1 for 1-indexed columns
}

fn main() {
    let args = cli::Cli::parse();

    // Phase 6: Generate execution_id
    let execution_id = Uuid::new_v4().to_string();
    eprintln!("Execution ID: {}", execution_id);

    // Phase 2: File walking & ignore
    let files = walk_files(&args.root, &args.glob);
    eprintln!("Found {} files", files.len());

    // Phase 3: Pattern matching
    // Compile the regex pattern
    let regex = match Regex::new(&args.pattern) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Error: Invalid regex pattern: {}", e);
            std::process::exit(1);
        }
    };
    eprintln!("Pattern compiled successfully");

    // Search files for pattern matches
    let mut matches = search_files(&files, &regex);
    eprintln!("Found {} matches", matches.len());

    // Phase 7: Deterministic ordering
    // Sort by file path, then byte_start for consistent output
    matches.sort();
    eprintln!("Matches sorted deterministically");

    // Debug: print first few matches with context
    for (i, m) in matches.iter().take(3).enumerate() {
        eprintln!("  Match {}: {} (id: {})", i + 1, m.file, m.match_id);
        eprintln!("    {}:{}:{}", m.file, m.line_number, m.column_number);
        eprintln!("    Before: \"{}\"", m.context_before);
        eprintln!("    Match:  \"{}\"", m.matched_text.chars().take(30).collect::<String>());
        eprintln!("    After:  \"{}\"", m.context_after);
    }

    // Phase 4: Line/column calculation
    // Line/column numbers calculated using byte_to_line and byte_to_column

    // Phase 5: Context extraction
    // TODO: Extract before/after context for each match

    // Phase 6: JSON output
    let output = SearchOutput {
        execution_id: execution_id.clone(),
        pattern: args.pattern.clone(),
        matches: matches.clone(),
        match_count: matches.len(),
    };

    // Serialize to JSON (pretty-printed for now, compact in Phase 8 with --json flag)
    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            // For now, print to stderr for debugging
            // Phase 8 will handle --json flag and stdout routing
            eprintln!();
            eprintln!("JSON Output:");
            eprintln!("{}", json);
        }
        Err(e) => {
            eprintln!("Error serializing to JSON: {}", e);
            std::process::exit(1);
        }
    }

    // Phase 7: Deterministic ordering
    // TODO: Sort by path then byte_start
    // TODO: Apply limit

    // Phase 8: CLI polish
    // TODO: Error handling, --json flag logic

    // UTF-8 handling verification
    eprintln!();
    eprintln!("UTF-8 handling: Column numbers count Unicode characters, not bytes");
    eprintln!("  Multi-byte characters (emoji, accented letters) count as 1 column");

    // Temporary: print args to show CLI works
    eprintln!();
    eprintln!("llmsearch scaffolding complete!");
    eprintln!("Args: {:#?}", args);
    eprintln!();
    eprintln!("Next: Implement phases 2-10 to add functionality.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_column_calculation() {
        // Test with multi-byte UTF-8 characters
        let content = "Hello 世界🌍";  // "世界" = 6 bytes, "🌍" = 4 bytes
        let line_index = build_line_index(content);

        // "世" starts at byte 6
        let byte_offset = 6;
        let line_num = byte_to_line(byte_offset, &line_index);
        let line_start = line_index[line_num - 1];
        let col_num = byte_to_column(byte_offset, line_start, content);

        assert_eq!(line_num, 1);
        assert_eq!(col_num, 7);  // "Hello " (6 chars) + "世" (1 char) = position 7

        // "🌍" starts at byte 12 (6 + 6 for "世界")
        let byte_offset_emoji = 12;
        let col_num_emoji = byte_to_column(byte_offset_emoji, line_start, content);
        assert_eq!(col_num_emoji, 9);  // "Hello 世界" (8 chars) + "🌍" (1 char) = position 9
    }

    #[test]
    fn test_line_index_multibyte() {
        let content = "a\n世界🌍\nb";
        let line_index = build_line_index(content);

        // Line 1 starts at 0
        assert_eq!(line_index[0], 0);
        // Line 2 starts at byte 2 (after "a\n")
        assert_eq!(line_index[1], 2);
        // Line 3 starts at byte 13 (after "a\n世界🌍\n" = 1+1+6+4+1 = 13 bytes)
        assert_eq!(line_index[2], 13);
    }

    #[test]
    fn test_context_extraction_edge_cases() {
        let content = "fn main() {\n    println!(\"Hello\");\n}";

        // Match at start of file
        let re = Regex::new(r"fn main").unwrap();
        let mat = re.find(content).unwrap();

        let line_index = build_line_index(content);
        let byte_start = mat.start();
        let line_num = byte_to_line(byte_start, &line_index);
        let line_start_byte = line_index[line_num - 1];

        // Context before at start of file should be empty
        let context_start = line_start_byte.max(byte_start.saturating_sub(100));
        let context_before = &content[context_start..byte_start];
        assert_eq!(context_before, "");  // Nothing before match at start

        // Context after should capture "()" after "fn main"
        let byte_end = mat.end();
        let remaining_line = &content[byte_end..];
        let line_end_offset = remaining_line.find('\n').unwrap_or(remaining_line.len());
        let context_end = byte_end + line_end_offset.min(100);
        let context_after = &content[byte_end..context_end.min(content.len())];
        assert!(context_after.contains("()"));  // Should have "()" after
    }
}
