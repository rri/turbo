//! Buffer and related logic.

use std::fs;
use std::io::Result;

/// Buffer object for manipulating and viewing structured text.
#[derive(Default)]
pub struct Buf {
    /// Raw string backing this buffer.
    pub raw: String,
    /// Byte offset of the cursor relative to start of the raw string.
    pub cur: usize,
    /// Indicates if the buffer is in 'escape mode' calling for special handling of the next input.
    pub esc: bool,
    /// Indicates whether the buffer has been terminated.
    pub trm: bool,
}

impl Buf {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            cur: 0,
            esc: false,
            trm: false,
        }
    }

    /// Load a file into this buffer.
    pub fn load(mut self, file: String) -> Result<Self> {
        self.raw = fs::read_to_string(file)?;
        Ok(self)
    }
}
