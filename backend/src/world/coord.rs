use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn chunk_coord(&self, chunk_size: i32) -> (i32, i32) {
        (
            self.x.div_euclid(chunk_size),
            self.y.div_euclid(chunk_size),
        )
    }

    pub fn local_coord(&self, chunk_size: i32) -> (i32, i32) {
        (
            self.x.rem_euclid(chunk_size),
            self.y.rem_euclid(chunk_size),
        )
    }

    pub fn neighbors(&self) -> [Coord; 8] {
        [
            Coord::new(self.x - 1, self.y - 1), // верхний левый
            Coord::new(self.x, self.y - 1),     // верхний
            Coord::new(self.x + 1, self.y - 1), // верхний правый
            Coord::new(self.x - 1, self.y),     // левый
            Coord::new(self.x + 1, self.y),     // правый
            Coord::new(self.x - 1, self.y + 1), // нижний левый
            Coord::new(self.x, self.y + 1),     // нижний
            Coord::new(self.x + 1, self.y + 1), // нижний правый
        ]
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coord() {
        let coord = Coord::new(65, 130);
        assert_eq!(coord.chunk_coord(64), (1, 2));

        let coord = Coord::new(-10, -20);
        assert_eq!(coord.chunk_coord(64), (-1, -1));
    }

    #[test]
    fn test_local_coord() {
        let coord = Coord::new(65, 130);
        assert_eq!(coord.local_coord(64), (1, 2));

        let coord = Coord::new(-10, -20);
        assert_eq!(coord.local_coord(64), (54, 44));
    }

    #[test]
    fn test_neighbors() {
        let coord = Coord::new(5, 5);
        let neighbors = coord.neighbors();
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&Coord::new(4, 4)));
        assert!(neighbors.contains(&Coord::new(6, 6)));
    }
}
