use clap::Parser;

/// LLM-native code search with structured JSON output
#[derive(Parser, Debug)]
#[command(name = "llmsearch")]
#[command(about = "Deterministic, structured search for LLM workflows")]
#[command(long_about = "
llmsearch is an LLM-native code search tool that provides structured JSON output
designed for AI agent workflows.

Every match includes byte offsets, line/column numbers, execution IDs for
auditability, and deterministic ordering so LLMs get consistent, queryable
results instead of unstructured text streams.

EXAMPLES:
    # Search for 'fn main' in current directory
    llmsearch -p 'fn main'

    # Search in specific directory with glob filter
    llmsearch -r ./src -p 'struct.*Error' -g '*.rs'

    # Search with JSON output for LLM consumption
    llmsearch -p 'TODO:' --json

    # Limit results and filter by file type
    llmsearch -p 'test' -g '*.rs' -g '*.toml' -l 50

OUTPUT FORMAT (JSON):
    {
      \"execution_id\": \"uuid-v4\",
      \"pattern\": \"search_pattern\",
      \"matches\": [
        {
          \"match_id\": \"uuid-v4\",
          \"file\": \"path/to/file\",
          \"byte_start\": 0,
          \"byte_end\": 10,
          \"matched_text\": \"match\",
          \"line_number\": 1,
          \"column_number\": 1,
          \"context_before\": \"before\",
          \"context_after\": \"after\"
        }
      ],
      \"match_count\": 1
    }
")]
pub struct Cli {
    /// Root directory to search recursively (default: current directory)
    /// Uses .gitignore for skipping ignored files/directories
    #[arg(short, long, default_value = ".")]
    pub root: String,

    /// Regular expression pattern to search for in file contents
    /// Uses Rust regex syntax: https://docs.rs/regex/latest/regex/#syntax
    #[arg(short, long)]
    pub pattern: String,

    /// File glob patterns to filter search (can be specified multiple times)
    /// Examples: '*.rs', '*.toml', 'test_*.rs'
    /// If not specified, all text files are searched
    #[arg(short, long)]
    pub glob: Vec<String>,

    /// Maximum number of matches to return (default: 100)
    /// Results are sorted by file path, then byte offset before limiting
    #[arg(short, long, default_value = "100")]
    pub limit: usize,

    /// Output results as JSON to stdout (structured format for LLMs)
    /// When enabled, suppresses debug output and human-readable messages
    #[arg(long, default_value_t = false)]
    pub json: bool,
}
