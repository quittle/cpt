use std::{
    cmp::{max, min},
    rc::Rc,
};

use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

use crate::*;

pub struct Menu<T> {
    items: Vec<MenuItemRc<T>>,
    prev: Vec<MenuItemRc<T>>,
    selected: Option<usize>,
}

pub enum MenuDirection {
    Up,
    Down,
}

fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}

impl<T> Menu<T> {
    pub fn new(items: Vec<MenuItemRc<T>>) -> Menu<T> {
        Menu {
            items,
            prev: vec![],
            selected: None,
        }
    }

    pub fn move_focus(&mut self, direction: MenuDirection) {
        match direction {
            MenuDirection::Up => {
                if let Some(cur) = self.selected {
                    self.selected = Some(max(cur, 1) - 1)
                } else {
                    self.selected = Some(self.items.len());
                }
            }
            MenuDirection::Down => {
                if let Some(cur) = self.selected {
                    self.selected = Some(min(cur + 1, self.items.len()))
                } else {
                    self.selected = Some(0);
                }
            }
        }
    }

    pub fn select_current_selection(&mut self) -> Option<T> {
        if let Some(selected) = self.selected {
            if let Some(item) = self.items.get(selected) {
                match item.action() {
                    MenuAction::MenuItem(items) => {
                        self.prev = self.items.clone();
                        self.items = items;
                        self.selected = None;
                        None
                    }
                    MenuAction::Done(output) => Some(output),
                }
            } else if self.selected == Some(self.items.len()) {
                self.items = self.prev.clone();
                self.selected = None;
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn select_by_name(&mut self, name: &str) -> Option<T> {
        let normalized_name = normalize_name(name);
        if normalized_name.is_empty() {
            return None;
        }

        for (index, item) in self.items.iter().enumerate() {
            if normalize_name(item.label()).starts_with(&normalized_name) {
                self.selected = Some(index);
            } else if "back".starts_with(&normalized_name) {
                self.selected = Some(self.items.len());
            }
        }

        self.select_current_selection()
    }

    pub fn show(&self, terminal_block: &mut TerminalBlock) {
        let prefix = |index| {
            if Some(index) == self.selected {
                ">"
            } else {
                "-"
            }
        };
        let menu_str = "Menu:\r\n".to_string()
            + &self
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| format!("{} {}", prefix(index), item.label()))
                .collect::<Vec<String>>()
                .join("\r\n")
            + &format!("\r\n{} Back", prefix(self.items.len()))
            + "\r\n> ";
        terminal_block.contents = menu_str;
    }

    pub fn wait_for_selection(
        &mut self,
        blocks: &mut Vec<TerminalBlock>,
    ) -> Result<T, ActionError> {
        let (_raw_out, _raw_err) = (
            std::io::stdout().into_raw_mode()?,
            std::io::stderr().into_raw_mode()?,
        );

        self.show(blocks.last_mut().unwrap());
        TerminalUi::draw(blocks)?;

        for c in std::io::stdin().events() {
            let evt = c.unwrap();
            let terminal_block = blocks.last_mut().unwrap();
            let result = match evt {
                Event::Key(Key::Char('\n')) => self.select_current_selection(),
                Event::Key(Key::Char(c)) => {
                    terminal_block.suffix.contents.push(c);
                    None
                }
                Event::Key(Key::Backspace) => {
                    terminal_block.suffix.contents.pop();
                    None
                }
                Event::Key(Key::Ctrl('c' | 'd')) => {
                    println!("Exiting");
                    return Err(ActionError::Exit(13));
                }
                Event::Key(Key::Up) => {
                    self.move_focus(MenuDirection::Up);
                    None
                }
                Event::Key(Key::Down) => {
                    self.move_focus(MenuDirection::Down);
                    None
                }
                Event::Key(k) => {
                    terminal_block.prefix.contents = format!("Key: {:?} ", k);
                    None
                }
                _ => None,
            };
            if let Some(output) = result {
                return Ok(output);
            }
            self.show(terminal_block);
            TerminalUi::draw(blocks)?;
        }
        Err(ActionError::fail("Exited input loop early"))
    }
}

type MenuItemRc<T> = Rc<dyn MenuItem<T>>;

pub enum MenuAction<T> {
    MenuItem(Vec<MenuItemRc<T>>),
    Done(T),
}

pub trait MenuItem<T> {
    fn label(&self) -> &str;
    fn action(&self) -> MenuAction<T>;
}

pub struct StatelessMenuItem<T> {
    pub label: String,
    pub action: dyn Fn() -> MenuAction<T>,
}

impl<T> MenuItem<T> for StatelessMenuItem<T> {
    fn label(&self) -> &str {
        &self.label
    }

    fn action(&self) -> MenuAction<T> {
        (self.action)()
    }
}

pub struct BackMenuItem<T> {
    pub prev: Vec<MenuItemRc<T>>,
}

impl<T> MenuItem<T> for BackMenuItem<T> {
    fn label(&self) -> &str {
        "Back"
    }

    fn action(&self) -> MenuAction<T> {
        MenuAction::MenuItem(self.prev.clone())
    }
}
