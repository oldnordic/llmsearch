mod cli;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    // Phase 2: File walking & ignore
    // TODO: Walk directory tree with .gitignore support
    let _files: Vec<String> = vec![];

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
