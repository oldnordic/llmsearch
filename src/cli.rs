use clap::Parser;

/// LLM-native code search with structured JSON output
#[derive(Parser, Debug)]
#[command(name = "llmsearch")]
#[command(about = "Deterministic, structured search for LLM workflows", long_about = None)]
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
