use std::collections::VecDeque;

use serde::Serialize;

pub type GridDimension = usize;

#[derive(Serialize)]
pub struct Grid<T> {
    members: Vec<Vec<Option<T>>>,
    width: GridDimension,
    height: GridDimension,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GridLocation {
    pub x: GridDimension,
    pub y: GridDimension,
}

impl GridLocation {
    pub fn is_adjacent(&self, other: &GridLocation) -> bool {
        self.distance(other) == 1
    }

    pub fn distance(&self, other: &GridLocation) -> GridDimension {
        (self.x).abs_diff(other.x) + (self.y).abs_diff(other.y)
    }

    pub fn get_surrounding(
        &self,
        width: GridDimension,
        height: GridDimension,
    ) -> Vec<GridLocation> {
        let mut ret = vec![];
        if self.x > 0 {
            ret.push(GridLocation {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            ret.push(GridLocation {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width - 1 {
            ret.push(GridLocation {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height - 1 {
            ret.push(GridLocation {
                x: self.x,
                y: self.y + 1,
            });
        }
        ret
    }
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

    pub fn find<F>(&self, predicate: F) -> Option<(GridDimension, GridDimension)>
    where
        F: Fn(&T) -> bool,
    {
        for (y, row) in self.members.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if let Some(item) = item {
                    if predicate(item) {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }

    pub fn find_in_range<F>(
        &self,
        location: GridLocation,
        range: GridDimension,
        predicate: F,
    ) -> Vec<GridLocation>
    where
        F: Fn(&T) -> bool,
    {
        let mut ret = vec![];
        for x in location.x - range..=location.x + range {
            for y in location.y - range..=location.y + range {
                if location.distance(&GridLocation { x, y }) > range {
                    continue;
                }
                if let Some(value) = self.get(x, y) {
                    if predicate(value) {
                        ret.push(GridLocation { x, y });
                    }
                }
            }
        }
        ret
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

    pub fn shortest_path<F>(
        &self,
        from: GridLocation,
        to: GridLocation,
        is_open: F,
    ) -> Option<Vec<GridLocation>>
    where
        F: Fn(&T) -> bool,
    {
        let mut track = vec![vec![u64::MAX; self.width]; self.height];
        let mut options = VecDeque::from([from.clone()]);

        track[from.y][from.x] = 0;

        while let Some(cur) = options.pop_front() {
            if cur == to {
                break;
            }

            for loc in cur.get_surrounding(self.width, self.height) {
                if track[loc.y][loc.x] < u64::MAX {
                    continue;
                }
                if let Some(entry) = self.get(loc.x, loc.y) {
                    if is_open(entry) {
                        track[loc.y][loc.x] = track[cur.y][cur.x] + 1;
                        options.push_back(loc);
                    }
                } else {
                    track[loc.y][loc.x] = track[cur.y][cur.x] + 1;
                    options.push_back(loc);
                }
            }
        }
        if track[to.y][to.x] == u64::MAX {
            return None;
        }

        let mut cur_loc = to.clone();
        let mut directions = vec![to.clone()];
        while cur_loc != from {
            let mut min_distance = track[cur_loc.y][cur_loc.x];
            let mut next_loc = cur_loc.clone();
            for option in cur_loc.get_surrounding(self.width, self.height) {
                let dist = track[option.y][option.x];
                if dist < min_distance {
                    min_distance = dist;
                    next_loc = option;
                }
            }
            if next_loc == cur_loc {
                return None;
            }
            directions.push(next_loc.clone());
            cur_loc = next_loc;
        }
        directions.reverse();
        Some(directions)
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    use super::GridLocation;

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

    #[test]
    pub fn test_find() {
        let mut grid = Grid::new(3, 3);

        assert_eq!(
            grid.find(|_value| true),
            None,
            "Try to find anything in an empty grid",
        );

        for x in 0..3 {
            for y in 0..3 {
                grid.set(x, y, format!("{}{}", x, y));
            }
        }
        assert_eq!(grid.find(|value| value == "12"), Some((1, 2)));
        grid.clear(1, 2);
        assert_eq!(
            grid.find(|value| value == "12"),
            None,
            "This value was cleared and should no longer match"
        );
        assert_eq!(
            grid.find(|_value| false),
            None,
            "Nothing is ever good enough"
        );
    }

    #[test]
    pub fn test_find_in_range() {
        let mut grid = Grid::new(3, 3);
        for x in 0..3 {
            for y in 0..3 {
                grid.set(x, y, format!("{}{}", x, y));
            }
        }
        assert_eq!(
            grid.find_in_range(GridLocation { x: 1, y: 1 }, 0, |_value| true),
            vec![GridLocation { x: 1, y: 1 }],
            "Range of 0 should only match itself"
        );

        assert_eq!(
            grid.find_in_range(GridLocation { x: 1, y: 1 }, 1, |_value| true),
            vec![
                GridLocation { x: 0, y: 1 },
                GridLocation { x: 1, y: 0 },
                GridLocation { x: 1, y: 1 },
                GridLocation { x: 1, y: 2 },
                GridLocation { x: 2, y: 1 }
            ],
            "Range of 1 should only include directly above and to the side"
        );
    }

    #[test]
    pub fn test_shortest_path() {
        let mut grid = Grid::new(3, 3);
        assert_eq!(
            grid.shortest_path(
                GridLocation { x: 0, y: 0 },
                GridLocation { x: 2, y: 2 },
                |entry| *entry == 0,
            ),
            Some(vec![
                GridLocation { x: 0, y: 0 },
                GridLocation { x: 0, y: 1 },
                GridLocation { x: 0, y: 2 },
                GridLocation { x: 1, y: 2 },
                GridLocation { x: 2, y: 2 }
            ]),
        );

        grid.set(0, 1, 1);
        grid.set(1, 1, 1);
        grid.set(2, 1, 0);
        assert_eq!(
            grid.shortest_path(
                GridLocation { x: 0, y: 0 },
                GridLocation { x: 0, y: 2 },
                |entry| *entry == 0,
            ),
            Some(vec![
                GridLocation { x: 0, y: 0 },
                GridLocation { x: 1, y: 0 },
                GridLocation { x: 2, y: 0 },
                GridLocation { x: 2, y: 1 },
                GridLocation { x: 2, y: 2 },
                GridLocation { x: 1, y: 2 },
                GridLocation { x: 0, y: 2 },
            ]),
            "Path around a blockade"
        );

        grid.set(2, 1, 1);
        assert_eq!(
            grid.shortest_path(
                GridLocation { x: 0, y: 0 },
                GridLocation { x: 0, y: 2 },
                |entry| *entry == 0,
            ),
            None,
            "Fully blocked"
        );
    }
}
