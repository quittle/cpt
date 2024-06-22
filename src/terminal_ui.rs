use regex::Regex;
use std::io::Write;
use term_size;
use unicode_width::UnicodeWidthStr;

use crate::*;

fn get_raw_str_width(s: &str) -> usize {
    // https://superuser.com/a/380778
    let ansi_color: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    ansi_color.replace_all(s, "").width()
}

pub struct TerminalUi {}

impl TerminalUi {
    pub fn draw(blocks: &[TerminalBlock]) -> Result<(), ActionError> {
        let (width, _height) = Self::get_dimensions();

        let mut raw_stdout = std::io::stdout();
        write!(raw_stdout, "\r{}\r", termion::clear::All)?;
        for block in blocks.iter() {
            block.draw(width, &mut raw_stdout)?;
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
        assert!(!self.contents.contains('\n'));
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

    pub fn draw(&self, width: usize, writer: &mut dyn Write) -> std::io::Result<()> {
        self.prefix.write(writer)?;
        for line in self.contents.lines() {
            writer.write_all(line.as_bytes())?;
            writer.write_all(
                " ".repeat(width - (get_raw_str_width(line) % width))
                    .as_bytes(),
            )?;
        }
        self.suffix.write(writer)?;
        Ok(())
    }
}
