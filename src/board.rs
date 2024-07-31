use serde::Serialize;

use crate::{CharacterId, Grid, GridDimension};

#[derive(Serialize, PartialEq)]
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

    pub fn find(&self, board_item: BoardItem) -> Option<(GridDimension, GridDimension)> {
        self.grid.find(|entry| entry == &board_item)
    }
}
