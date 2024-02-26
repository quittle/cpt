use std::io::Write;
use term_size;

use crate::*;

pub struct TerminalUi {}

impl TerminalUi {
    pub fn draw(blocks: &[TerminalBlock]) -> Result<(), ActionError> {
        let (width, _height) = Self::get_dimensions();

        let mut raw_stdout = std::io::stdout();
        write!(raw_stdout, "{}\r", termion::clear::All)?;
        for (i, block) in blocks.iter().enumerate() {
            let content_width =
                block.contents.len() + block.suffix.as_ref().map_or(0, |s| s.contents.len());
            write!(raw_stdout, "{}", block.contents)?;
            if let Some(suffix) = &block.suffix {
                write!(raw_stdout, "{}", suffix.contents)?;
            }
            if i < blocks.len() - 1 {
                write!(raw_stdout, "{}", " ".repeat(width - content_width))?;
            }
        }
        raw_stdout.flush()?;
        Ok(())
    }

    pub fn get_dimensions() -> (usize, usize) {
        term_size::dimensions().unwrap_or((100, 10))
    }
}

pub struct TerminalSpan {
    pub contents: String,
}
pub struct TerminalBlock {
    pub contents: String,
    pub suffix: Option<TerminalSpan>,
}

impl TerminalBlock {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            suffix: None,
        }
    }
    pub fn new_with_suffix(contents: String, suffix: String) -> Self {
        Self {
            contents,
            suffix: Some(TerminalSpan { contents: suffix }),
        }
    }
}
