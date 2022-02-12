use std::io;
use args::Command;

pub mod args;
pub mod check;

pub fn exec(file: &str, cmd: &Command) -> io::Result<()> {
    match cmd {
        Command::Check => {check::exec(file)}
    }
}