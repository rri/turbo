//! Core top-level application library where command-line options are evaluated.

use clap::{ColorChoice, Parser};
use std::io::Result;

/// Arguments to the command-line application.
#[derive(Parser)]
#[command(about)]
#[command(version)]
#[command(arg_required_else_help(true))]
#[command(color(ColorChoice::Always))]
#[command(disable_version_flag(true))]
#[command(help_template("{name} v{version}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}"))]
struct Args {
    /// (Optional) Paths to file(s) to open (or create upon save).
    file: Vec<String>,

    /// Print version information and exit.
    #[arg(short, long)]
    version: bool,
}

/// Run the application and return a result to exit the process.
pub fn run() -> Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        return Ok(());
    }

    Ok(())
}
