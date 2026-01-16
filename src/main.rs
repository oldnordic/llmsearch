mod cli;

use clap::Parser;
use ignore::Walk;

fn walk_files(root: &str, globs: &[String]) -> Vec<String> {
    let walker = Walk::new(root);

    // Filter by glob patterns if provided
    if !globs.is_empty() {
        // Build a combined glob filter
        // For now, just collect all files - glob filtering comes in 02-02
    }

    let mut files = Vec::new();
    for result in walker {
        match result {
            Ok(entry) => {
                if let Some(path) = entry.path().to_str() {
                    // Only include files, not directories
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        files.push(path.to_string());
                    }
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
