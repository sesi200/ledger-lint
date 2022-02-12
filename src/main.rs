use std::io;
use clap::Parser;

mod command;
mod util;

fn main() -> io::Result<()> {
    let args = command::args::Args::parse();

    command::exec(&args.infile, &args.command)
}
