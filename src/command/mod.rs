use std::{io, path::Path};

mod check;

pub fn exec(file: &Path, cmd: &Command) -> io::Result<()> {
    match cmd {
        Command::Check(opt) => check::exec(file, opt),
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Check(check::CheckOpts),
}
