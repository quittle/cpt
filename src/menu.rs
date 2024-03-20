use std::rc::Rc;

use crate::*;

pub struct Menu<T> {
    items: Vec<MenuItemRc<T>>,
    prev: Vec<MenuItemRc<T>>,
}

fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}

impl<T> Menu<T> {
    pub fn new(items: Vec<MenuItemRc<T>>) -> Menu<T> {
        Menu {
            items,
            prev: vec![],
        }
    }

    pub fn select_by_name(&mut self, name: &str) -> Option<T> {
        let normalized_name = normalize_name(name);
        if normalized_name.is_empty() {
            return None;
        }

        for item in &self.items {
            if normalize_name(item.label()).starts_with(&normalized_name) {
                match item.action() {
                    MenuAction::MenuItem(items) => {
                        self.prev = self.items.clone();
                        self.items = items;
                        return None;
                    }
                    MenuAction::Done(output) => {
                        return Some(output);
                    }
                }
            } else if "back".starts_with(&normalized_name) {
                self.items = self.prev.clone();
                return None;
            }
        }
        None
    }

    pub fn show(&self, terminal_block: &mut TerminalBlock) {
        let menu_str = "Menu:\r\n".to_string()
            + &self
                .items
                .iter()
                .map(|i| format!("- {}", i.label()))
                .collect::<Vec<String>>()
                .join("\r\n")
            + "\r\n- Back"
            + "\r\n> ";
        terminal_block.contents = menu_str;
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
