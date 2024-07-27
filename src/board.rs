use serde::Serialize;

use crate::{CharacterId, Grid, GridDimension};

#[derive(Serialize)]
pub enum BoardItem {
    Character(CharacterId),
}

#[derive(Serialize)]
pub struct Board {
    pub grid: Grid<BoardItem>,
}

impl Board {
    pub fn new(width: GridDimension, height: GridDimension) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }
}
