use clap_complete::shells::Bash;
use clap_complete::generate_to;
use clap::IntoApp;
use std::env;
use std::io::Error;

include!("src/command/args.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let name = "ledger-lint-complete".to_string();
    let mut app = Args::into_app();
    let path = generate_to(
        Bash,
        &mut app,
        &name,
        outdir,
    )?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}