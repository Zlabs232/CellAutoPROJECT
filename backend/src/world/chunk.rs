use std::collections::HashSet;

pub const CHUNK_SIZE: i32 = 64;

#[derive(Debug, Clone)]
pub struct Chunk {
    active_cells: HashSet<(i32, i32)>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            active_cells: HashSet::new(),
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        self.active_cells.contains(&(x, y))
    }

    pub fn set_cell(&mut self, x: i32, y: i32, alive: bool) {
        if alive {
            self.active_cells.insert((x, y));
        } else {
            self.active_cells.remove(&(x, y));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.active_cells.is_empty()
    }

    pub fn active_count(&self) -> usize {
        self.active_cells.len()
    }

    pub fn iter_active(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.active_cells.iter().copied()
    }

    pub fn clear(&mut self) {
        self.active_cells.clear();
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_new() {
        let chunk = Chunk::new();
        assert!(chunk.is_empty());
        assert_eq!(chunk.active_count(), 0);
    }

    #[test]
    fn test_set_get_cell() {
        let mut chunk = Chunk::new();

        chunk.set_cell(5, 10, true);
        assert!(chunk.get_cell(5, 10));
        assert!(!chunk.get_cell(5, 11));
        assert_eq!(chunk.active_count(), 1);

        chunk.set_cell(5, 10, false);
        assert!(!chunk.get_cell(5, 10));
        assert!(chunk.is_empty());
    }

    #[test]
    fn test_iter_active() {
        let mut chunk = Chunk::new();
        chunk.set_cell(1, 1, true);
        chunk.set_cell(2, 2, true);
        chunk.set_cell(3, 3, true);

        let active: Vec<_> = chunk.iter_active().collect();
        assert_eq!(active.len(), 3);
        assert!(active.contains(&(1, 1)));
        assert!(active.contains(&(2, 2)));
        assert!(active.contains(&(3, 3)));
    }

    #[test]
    fn test_clear() {
        let mut chunk = Chunk::new();
        chunk.set_cell(1, 1, true);
        chunk.set_cell(2, 2, true);

        assert!(!chunk.is_empty());
        chunk.clear();
        assert!(chunk.is_empty());
    }
}
