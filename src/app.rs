//! Core top-level application library where command-line options are evaluated.

use crate::buf::Buf;
use crate::edt::Win;
use clap::{ColorChoice, Parser};
use std::io::Result;

/// Arguments to the command-line application.
#[derive(Parser)]
#[command(about)]
#[command(version)]
#[command(color(ColorChoice::Always))]
#[command(disable_version_flag(true))]
#[command(help_template(
    "{before-help}{name} v{version}\n{about}\n\nUSAGE:\n{tab}{usage}\n\nARGUMENTS:\n{positionals}\n\nOPTIONS:\n{options}{after-help}"
))]
struct Args {
    /// (Optional) Paths to file(s) to create (upon save) or open.
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

    let mut buffers: Vec<Buf> = vec![];

    for file in args.file {
        buffers.push(Buf::new().load(file)?)
    }

    Win::run(&mut buffers)
}
