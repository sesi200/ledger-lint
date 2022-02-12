use clap::Parser;

/// Helps you to clean up a ledger-cli file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to input file
    pub infile: String,
}
