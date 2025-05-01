use clap::Parser;
use std::io::Result;
use std::path::Path;

use crate::util::parser::ledgerfile;

/// Format a ledgerfile.
#[derive(Parser, Debug)]
pub struct FmtOpts {}

#[allow(unused)]
#[derive(Debug, Default)]
struct State {}

pub fn exec(file: &Path, _opts: &FmtOpts) -> Result<()> {
    let file_content = std::fs::read_to_string(file)?;
    let ledgerfile = ledgerfile(&file_content).unwrap().1;
    std::fs::write(file, format!("{ledgerfile}"))?;
    Ok(())
}
