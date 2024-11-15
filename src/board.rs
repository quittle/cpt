use serde::Serialize;

use crate::{CardId, CharacterId, Grid, GridDimension, GridLocation};

#[derive(Serialize, PartialEq, Debug)]
pub enum BoardItem {
    Character(CharacterId),
    Card(CardId),
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

    pub fn find(&self, board_item: &BoardItem) -> Option<(GridDimension, GridDimension)> {
        self.grid.find(|entry| entry == board_item)
    }

    pub fn find_chars_in_range(
        &self,
        location: GridLocation,
        range: GridDimension,
    ) -> Vec<CharacterId> {
        self.grid
            .find_in_range(location, range, |entry| {
                matches!(entry, BoardItem::Character(_))
            })
            .iter()
            .map(|location| match self.grid.get(location.x, location.y) {
                Some(BoardItem::Character(id)) => *id,
                _ => {
                    unreachable!("Only valid characters available due to filterin in find_in_range")
                }
            })
            .collect()
    }

    pub fn distance(&self, a: BoardItem, b: BoardItem) -> Option<u64> {
        if let (Some((ax, ay)), Some((bx, by))) = (self.find(&a), self.find(&b)) {
            Some((ax as u64).abs_diff(bx as u64) + (ay as u64).abs_diff(by as u64))
        } else {
            None
        }
    }

    pub fn shortest_path(&self, a: BoardItem, b: BoardItem) -> Option<Vec<GridLocation>> {
        if let (Some((ax, ay)), Some((bx, by))) = (self.find(&a), self.find(&b)) {
            self.grid.shortest_path(
                GridLocation { x: ax, y: ay },
                GridLocation { x: bx, y: by },
                |item| match item {
                    BoardItem::Card(_) => true,
                    character @ BoardItem::Character(_) => &b == character,
                },
            )
        } else {
            None
        }
    }

    pub fn require_distance(&self, a: BoardItem, b: BoardItem) -> u64 {
        self.distance(a, b).unwrap()
    }
}
