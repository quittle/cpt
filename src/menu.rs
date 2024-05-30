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

pub struct Menu<State, T> {
    items: Vec<MenuItemRc<State, T>>,
    prev: Vec<Vec<MenuItemRc<State, T>>>,
    selected: Option<usize>,
}

pub enum MenuDirection {
    Up,
    Down,
}

fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}

impl<State, T> Menu<State, T> {
    pub fn new(items: Vec<MenuItemRc<State, T>>) -> Menu<State, T> {
        Menu {
            items,
            prev: vec![],
            selected: None,
        }
    }

    pub fn move_focus(&mut self, direction: MenuDirection) {
        let last_entry_index = if self.has_back() {
            self.items.len()
        } else {
            self.items.len() - 1
        };
        match direction {
            MenuDirection::Up => {
                if let Some(cur) = self.selected {
                    self.selected = Some(max(cur, 1) - 1)
                } else {
                    self.selected = Some(last_entry_index);
                }
            }
            MenuDirection::Down => {
                if let Some(cur) = self.selected {
                    self.selected = Some(min(cur + 1, last_entry_index))
                } else {
                    self.selected = Some(0);
                }
            }
        }
    }

    pub fn go_back(&mut self) {
        if !self.prev.is_empty() {
            self.items = self.prev.pop().unwrap();
        }
        self.selected = None;
    }

    pub fn select_current_selection(&mut self, state: &State) -> Option<T> {
        if let Some(selected) = self.selected {
            if let Some(item) = self.items.get(selected) {
                match item.action(state) {
                    MenuAction::MenuItem(items) => {
                        self.prev.push(self.items.clone());
                        self.items = items;
                        self.selected = None;
                        None
                    }
                    MenuAction::Done(output) => Some(output),
                }
            } else if self.selected == Some(self.items.len()) {
                self.go_back();
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn select_by_name(&mut self, name: &str, state: &State) -> Option<T> {
        let normalized_name = normalize_name(name);
        if normalized_name.is_empty() {
            return None;
        }

        for (index, item) in self.items.iter().enumerate() {
            if normalize_name(&item.label(state)).starts_with(&normalized_name) {
                self.selected = Some(index);
            } else if self.has_back() && "back".starts_with(&normalized_name) {
                self.selected = Some(self.items.len());
            }
        }

        self.select_current_selection(state)
    }

    pub fn show(&self, block: &mut TerminalBlock, state: &State) {
        let prefix = |index| {
            if Some(index) == self.selected {
                ">"
            } else {
                "-"
            }
        };
        let back_entry = if self.has_back() {
            format!("\r\n{} Back", prefix(self.items.len()))
        } else {
            String::new()
        };
        let menu_str = "Menu:\r\n".to_string()
            + &self
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| format!("{} {}", prefix(index), item.label(state)))
                .collect::<Vec<String>>()
                .join("\r\n")
            + &back_entry;
        block.contents = menu_str;
    }

    pub fn wait_for_selection(
        &mut self,
        blocks: &mut [TerminalBlock],
        state: &State,
    ) -> Result<T, ActionError> {
        let (_raw_out, _raw_err) = (
            std::io::stdout().into_raw_mode()?,
            std::io::stderr().into_raw_mode()?,
        );

        self.show(&mut blocks[blocks.len() - 2], state);
        blocks.last_mut().unwrap().contents = "> ".to_string();
        TerminalUi::draw(blocks)?;

        for c in std::io::stdin().events() {
            let evt = c.unwrap();
            let terminal_block = blocks.last_mut().unwrap();
            terminal_block.prefix.contents.clear();
            let result = match evt {
                Event::Key(Key::Char('\n')) => self.select_current_selection(state),
                Event::Key(Key::Char(c)) => {
                    terminal_block.suffix.contents.push(c);
                    None
                }
                Event::Key(Key::Backspace) => {
                    terminal_block.suffix.contents.pop();
                    None
                }
                Event::Key(Key::Ctrl('c' | 'd')) => {
                    return Err(ActionError::Exit(13));
                }
                Event::Key(Key::Esc) => {
                    self.go_back();
                    None
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
            self.show(&mut blocks[blocks.len() - 2], state);
            TerminalUi::draw(blocks)?;
        }
        Err(ActionError::fail("Exited input loop early"))
    }

    fn has_back(&self) -> bool {
        !self.prev.is_empty()
    }
}

type MenuItemRc<State, T> = Rc<dyn MenuItem<State, T>>;

pub enum MenuAction<State, T> {
    MenuItem(Vec<MenuItemRc<State, T>>),
    Done(T),
}

pub trait MenuItem<State, T> {
    fn label(&self, state: &State) -> String;
    fn action(&self, state: &State) -> MenuAction<State, T>;
}

pub struct StatelessMenuItem<State, T> {
    pub label: String,
    pub action: dyn Fn() -> MenuAction<State, T>,
}

impl<State, T> MenuItem<State, T> for StatelessMenuItem<State, T> {
    fn label(&self, _state: &State) -> String {
        self.label.clone()
    }

    fn action(&self, _state: &State) -> MenuAction<State, T> {
        (self.action)()
    }
}

pub struct BackMenuItem<State, T> {
    pub prev: Vec<MenuItemRc<State, T>>,
}

impl<State, T> MenuItem<State, T> for BackMenuItem<State, T> {
    fn label(&self, _state: &State) -> String {
        "Back".to_string()
    }

    fn action(&self, _state: &State) -> MenuAction<State, T> {
        MenuAction::MenuItem(self.prev.clone())
    }
}
