use clap::Parser;
use std::{io, path::PathBuf};

mod command;
mod util;

/// Helps you to clean up a ledger-cli file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to input file
    pub infile: PathBuf,

    ///Command to use
    #[clap(subcommand)]
    pub command: command::Command,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    command::exec(&args.infile, &args.command)
}
