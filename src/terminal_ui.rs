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
                block.prefix.contents.len() + block.contents.len() + block.suffix.contents.len();
            block.prefix.write(&mut raw_stdout)?;
            raw_stdout.write_all(block.contents.as_bytes())?;
            block.suffix.write(&mut raw_stdout)?;
            if i < blocks.len() - 1 {
                raw_stdout.write_all(" ".repeat(width - content_width).as_bytes())?;
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
    pub fn new(contents: String) -> Self {
        Self {
            prefix: Default::default(),
            contents,
            suffix: Default::default(),
        }
    }
}
