//! Main module for the command-line application launcher.

use std::process;
use turbo::app;

/// Main function and entry-point for the operating system process.
fn main() {
    process::exit(match app::run() {
        Ok(()) => exitcode::OK,
        Err(e) => {
            eprintln!("🚀 Launch failure: {}", &e.to_string());
            exitcode::IOERR
        }
    })
}
