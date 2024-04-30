use std::io::Write;
use term_size;

use crate::*;

fn get_len_of_block(block: &TerminalBlock) -> usize {
    if let Some(index) = block.suffix.contents.rfind('\n') {
        block.suffix.contents.len() - index - 1
    } else if let Some(index) = block.contents.rfind('\n') {
        block.suffix.contents.len() + (block.contents.len() - index) - 1
    } else if let Some(index) = block.prefix.contents.rfind('\n') {
        block.suffix.contents.len() + block.contents.len() + (block.prefix.contents.len() - index)
            - 1
    } else {
        block.suffix.contents.len() + block.contents.len() + block.prefix.contents.len()
    }
}

pub struct TerminalUi {}

impl TerminalUi {
    pub fn draw(blocks: &[TerminalBlock]) -> Result<(), ActionError> {
        let (width, _height) = Self::get_dimensions();

        let mut raw_stdout = std::io::stdout();
        write!(raw_stdout, "{}\r", termion::clear::All)?;
        for (i, block) in blocks.iter().enumerate() {
            block.prefix.write(&mut raw_stdout)?;
            raw_stdout.write_all(block.contents.as_bytes())?;
            block.suffix.write(&mut raw_stdout)?;
            if i < blocks.len() - 1 {
                raw_stdout.write_all(" ".repeat(width - get_len_of_block(block)).as_bytes())?;
            }
        }
        raw_stdout.flush()?;
        Ok(())
    }

    pub fn get_dimensions() -> (usize, usize) {
        term_size::dimensions().unwrap_or((100, 10))
    }
}

#[derive(Default)]
pub struct TerminalSpan {
    pub contents: String,
    pub color: Option<Box<dyn termion::color::Color>>,
}

impl From<&str> for TerminalSpan {
    fn from(val: &str) -> Self {
        val.to_string().into()
    }
}

impl From<String> for TerminalSpan {
    fn from(val: String) -> Self {
        TerminalSpan {
            contents: val,
            color: None,
        }
    }
}

impl TerminalSpan {
    pub fn write(&self, write: &mut dyn Write) -> std::io::Result<()> {
        if let Some(color) = &self.color {
            write!(write, "{}", termion::color::Fg(color.as_ref()))?;
        }
        write.write_all(self.contents.as_bytes())?;
        write.write_all(termion::color::Reset.fg_str().as_bytes())?;
        Ok(())
    }
}

#[derive(Default)]
pub struct TerminalBlock {
    pub prefix: TerminalSpan,
    pub contents: String,
    pub suffix: TerminalSpan,
}

impl TerminalBlock {
    pub fn new(contents: impl AsRef<str>) -> Self {
        Self {
            contents: contents.as_ref().to_string(),
            ..Default::default()
        }
    }
}
