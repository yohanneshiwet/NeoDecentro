// src/main.rs
/*
 * Main executable for DecentroBridgePro
 */

use clap::Parser;
use decentrobridgepro::{Result, run};

#[derive(Parser)]
#[command(version, about = "DecentroBridgePro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
