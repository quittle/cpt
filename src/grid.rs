use std::collections::HashMap;

type GridDimension = u32;

pub struct Grid<T> {
    pub width: GridDimension,
    pub height: GridDimension,
    members: HashMap<(GridDimension, GridDimension), T>,
}

impl<T> Grid<T> {
    pub fn new(width: GridDimension, height: GridDimension) -> Self {
        Self {
            width,
            height,
            members: Default::default(),
        }
    }

    pub fn get(&self, x: GridDimension, y: GridDimension) -> Option<&T> {
        self.members.get(&(x, y))
    }

    pub fn is_set(&self, x: GridDimension, y: GridDimension) -> bool {
        self.members.contains_key(&(x, y))
    }

    pub fn set(&mut self, x: GridDimension, y: GridDimension, value: T) -> Option<T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.members.insert((x, y), value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    pub fn test_grid() {
        let mut grid = Grid::new(2, 3);
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(0, 0), None);
        assert_eq!(grid.get(100, 100), None);
        assert_eq!(grid.set(1, 1, 'a'), None);
        assert_eq!(grid.set(1, 1, 'b'), Some('a'));
        assert_eq!(grid.get(1, 1), Some(&'b'));
        assert!(!grid.is_set(0, 0));
        assert!(grid.is_set(1, 1));

        grid.set(100, 100, 'z');
        assert_eq!(grid.get(100, 100), None);
    }
}
