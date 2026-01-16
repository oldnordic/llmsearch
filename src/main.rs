mod cli;

use clap::Parser;
use ignore::Walk;
use regex::Regex;
use std::path::Path;

struct Match {
    file: String,
    byte_start: usize,
    byte_end: usize,
    matched_text: String,
    line_number: usize,
    column_number: usize,
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
                    let line_num = byte_to_line(byte_start, &line_index);
                    let line_start_byte = line_index[line_num - 1];
                    let col_num = byte_to_column(byte_start, line_start_byte, &content);

                    matches.push(Match {
                        file: file_path.clone(),
                        byte_start,
                        byte_end: mat.end(),
                        matched_text: mat.as_str().to_string(),
                        line_number: line_num,
                        column_number: col_num,
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
    let matches = search_files(&files, &regex);
    eprintln!("Found {} matches", matches.len());

    // Debug: print first few matches to verify
    for (i, m) in matches.iter().take(3).enumerate() {
        eprintln!("  Match {}: {}:{}:{} - \"{}\"",
            i + 1, m.file, m.line_number, m.column_number,
            m.matched_text.chars().take(50).collect::<String>()
        );
    }

    // Phase 4: Line/column calculation
    // Line/column numbers calculated using byte_to_line and byte_to_column

    // Phase 5: Context extraction
    // TODO: Extract before/after context for each match

    // Phase 6: JSON output
    // TODO: Generate execution_id (UUID)
    // TODO: Serialize to JSON schema

    // Phase 7: Deterministic ordering
    // TODO: Sort by path then byte_start
    // TODO: Apply limit

    // Phase 8: CLI polish
    // TODO: Error handling, --json flag logic

    // Temporary: print args to show CLI works
    eprintln!("llmsearch scaffolding complete!");
    eprintln!("Args: {:#?}", args);
    eprintln!();
    eprintln!("Next: Implement phases 2-10 to add functionality.");
}
