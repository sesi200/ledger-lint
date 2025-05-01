use std::{io, path::Path};

mod check;
mod fmt;

pub fn exec(file: &Path, cmd: &Command) -> io::Result<()> {
    match cmd {
        Command::Check(opt) => check::exec(file, opt),
        Command::Fmt(opt) => fmt::exec(file, opt),
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Check(check::CheckOpts),
    Fmt(fmt::FmtOpts),
}
