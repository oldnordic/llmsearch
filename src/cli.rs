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
    /// Root directory to search (default: current directory)
    #[arg(short, long, default_value = ".")]
    pub root: String,

    /// Regex pattern to search for
    #[arg(short, long)]
    pub pattern: String,

    /// File glob patterns (can be specified multiple times)
    #[arg(short, long)]
    pub glob: Vec<String>,

    /// Maximum number of results to return
    #[arg(short, long, default_value = "100")]
    pub limit: usize,

    /// Output results as JSON (structured format for LLMs)
    #[arg(long, default_value_t = false)]
    pub json: bool,
}
