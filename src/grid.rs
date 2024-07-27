use serde::Serialize;

pub type GridDimension = usize;

#[derive(Serialize)]
pub struct Grid<T> {
    members: Vec<Vec<Option<T>>>,
    width: GridDimension,
    height: GridDimension,
}

impl<T> Grid<T> {
    pub fn new(width: GridDimension, height: GridDimension) -> Self {
        let mut members: Vec<Vec<Option<T>>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(None);
            }
            members.push(row);
        }
        Self {
            width,
            height,
            members,
        }
    }

    pub fn width(&self) -> GridDimension {
        self.width
    }

    pub fn height(&self) -> GridDimension {
        self.height
    }

    pub fn get(&self, x: GridDimension, y: GridDimension) -> Option<&T> {
        if self.is_valid(x, y) {
            self.members[y][x].as_ref()
        } else {
            None
        }
    }

    pub fn is_set(&self, x: GridDimension, y: GridDimension) -> bool {
        self.get(x, y).is_some()
    }

    pub fn set(&mut self, x: GridDimension, y: GridDimension, value: T) -> Option<T> {
        if self.is_valid(x, y) {
            self.members[y][x].replace(value)
        } else {
            None
        }
    }

    pub fn clear(&mut self, x: GridDimension, y: GridDimension) -> Option<T> {
        if self.is_valid(x, y) {
            self.members[y][x].take()
        } else {
            None
        }
    }

    pub fn is_valid(&self, x: GridDimension, y: GridDimension) -> bool {
        x < self.width && y < self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    pub fn test_grid() {
        let mut grid = Grid::new(2, 3);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.get(0, 0), None);
        assert_eq!(grid.get(100, 100), None);
        assert_eq!(grid.set(1, 1, 'a'), None);
        assert_eq!(grid.set(1, 1, 'b'), Some('a'));
        assert_eq!(grid.get(1, 1), Some(&'b'));
        assert!(!grid.is_set(0, 0));
        assert!(grid.is_set(1, 1));

        assert!(grid.is_valid(1, 1));
        assert!(!grid.is_valid(100, 100));
        grid.set(100, 100, 'z');
        assert_eq!(grid.get(100, 100), None);

        assert_eq!(grid.clear(1, 1), Some('b'));
        assert_eq!(grid.get(1, 1), None);
        assert_eq!(grid.clear(1, 1), None);
    }
}
