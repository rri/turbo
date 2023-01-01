//! Top-level editor window and associated structures.
use std::io::Result;

use crate::buf::Buf;

/// Editor window serving as a top-level structure for working with buffers.
#[derive(Default)]
pub struct Win {
    /// Buffers managed by this window.
    pub buf: Vec<Buf>,
}

impl Win {
    /// Run the editor window until the user explicitly exits.
    pub fn run(_buf: &mut Vec<Buf>) -> Result<()> {
        Ok(())
    }
}
