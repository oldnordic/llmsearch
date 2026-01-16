mod cli;

use clap::Parser;
use ignore::Walk;

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

fn main() {
    let args = cli::Cli::parse();

    // Phase 2: File walking & ignore
    let files = walk_files(&args.root, &args.glob);
    eprintln!("Found {} files", files.len());

    // Phase 3: Pattern matching
    // TODO: Search file contents with regex, track byte offsets
    let _matches: Vec<String> = vec![];

    // Phase 4: Line/column calculation
    // TODO: Convert byte offsets to line/col positions
    // TODO: Handle multi-byte UTF-8 correctly

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
