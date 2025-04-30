use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use std::env;
use std::io::Error;
use std::path::PathBuf;

#[path = "src/args.rs"]
mod args;
use crate::args::Args;

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let out_path = PathBuf::from(outdir.clone());

    let mut cmd = <Args as CommandFactory>::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, "ppd", outdir.clone())?;
    }

    clap_mangen::generate_to(cmd, out_path)?;

    Ok(())
}
