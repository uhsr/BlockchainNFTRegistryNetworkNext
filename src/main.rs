// src/main.rs
/*
 * Main executable for BlockchainNFTRegistryNetworkNext
 */

use clap::Parser;
use blockchainnftregistrynetworknext::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTRegistryNetworkNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
