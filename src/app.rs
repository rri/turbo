//! Core top-level application library where command-line options are evaluated.

use clap::{ColorChoice, Parser};
use std::io::Result;

/// Arguments to the command-line application.
#[derive(Parser)]
#[clap(about)]
#[clap(version)]
#[clap(color(ColorChoice::Auto))]
#[clap(disable_version_flag(true))]
struct Args {
    /// Print version information and exit.
    #[arg(short, long)]
    version: bool,
}

/// Run the application and return a result to exit the process.
pub fn run() -> Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        return Ok(());
    }

    Ok(())
}
