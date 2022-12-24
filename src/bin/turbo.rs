//! Main module for the command-line application launcher.

use turbo::app;
use std::process;

/// Main function and entry-point for the operating system process.
fn main() {
    process::exit(match app::run() {
        Ok(()) => exitcode::OK,
        Err(e) => {
            eprint!("🚀 Launch failure: {}", &e.to_string());
            exitcode::IOERR
        }
    })
}
