use std::collections::HashMap;
use super::chunk::{Chunk, CHUNK_SIZE};
use super::coord::Coord;

/// мир - сетка из чанков
#[derive(Debug, Clone)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }
    pub fn get_cell(&self, coord: Coord) -> bool {
        let (chunk_x, chunk_y) = coord.chunk_coord(CHUNK_SIZE);
        let (local_x, local_y) = coord.local_coord(CHUNK_SIZE);

        self.chunks
            .get(&(chunk_x, chunk_y))
            .map(|chunk| chunk.get_cell(local_x, local_y))
            .unwrap_or(false)
    }

    pub fn set_cell(&mut self, coord: Coord, alive: bool) {
        let (chunk_x, chunk_y) = coord.chunk_coord(CHUNK_SIZE);
        let (local_x, local_y) = coord.local_coord(CHUNK_SIZE);

        if alive {
            let chunk = self.chunks
                .entry((chunk_x, chunk_y))
                .or_insert_with(Chunk::new);
            chunk.set_cell(local_x, local_y, true);
        } else {
            if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_y)) {
                chunk.set_cell(local_x, local_y, false);
                if chunk.is_empty() {
                    self.chunks.remove(&(chunk_x, chunk_y));
                }
            }
        }
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn active_cell_count(&self) -> usize {
        self.chunks.values().map(|chunk| chunk.active_count()).sum()
    }

    pub fn iter_active_cells(&self) -> impl Iterator<Item = Coord> + '_ {
        self.chunks.iter().flat_map(|((chunk_x, chunk_y), chunk)| {
            let chunk_offset_x = chunk_x * CHUNK_SIZE;
            let chunk_offset_y = chunk_y * CHUNK_SIZE;

            chunk.iter_active().map(move |(local_x, local_y)| {
                Coord::new(chunk_offset_x + local_x, chunk_offset_y + local_y)
            })
        })
    }

    pub fn get_bounds(&self) -> Option<(Coord, Coord)> {
        let mut active_cells: Vec<Coord> = self.iter_active_cells().collect();

        if active_cells.is_empty() {
            return None;
        }

        active_cells.sort_by_key(|c| c.x);
        let min_x = active_cells.first().unwrap().x;
        let max_x = active_cells.last().unwrap().x;

        active_cells.sort_by_key(|c| c.y);
        let min_y = active_cells.first().unwrap().y;
        let max_y = active_cells.last().unwrap().y;

        Some((Coord::new(min_x, min_y), Coord::new(max_x, max_y)))
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    pub fn count_neighbors(&self, coord: Coord) -> u8 {
        coord.neighbors()
            .iter()
            .filter(|&&neighbor| self.get_cell(neighbor))
            .count() as u8
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_new() {
        let world = World::new();
        assert_eq!(world.chunk_count(), 0);
        assert_eq!(world.active_cell_count(), 0);
    }

    #[test]
    fn test_set_get_cell() {
        let mut world = World::new();

        let coord = Coord::new(100, 200);
        world.set_cell(coord, true);

        assert!(world.get_cell(coord));
        assert!(!world.get_cell(Coord::new(101, 200)));
        assert_eq!(world.active_cell_count(), 1);
    }

    #[test]
    fn test_cross_chunk_boundaries() {
        let mut world = World::new();

        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(64, 64), true);
        world.set_cell(Coord::new(-1, -1), true);

        assert_eq!(world.chunk_count(), 3);
        assert_eq!(world.active_cell_count(), 3);
    }

    #[test]
    fn test_chunk_cleanup() {
        let mut world = World::new();

        let coord = Coord::new(50, 50);
        world.set_cell(coord, true);
        assert_eq!(world.chunk_count(), 1);

        world.set_cell(coord, false);
        assert_eq!(world.chunk_count(), 0);
    }

    #[test]
    fn test_count_neighbors() {
        let mut world = World::new();

        let center = Coord::new(0, 0);
        world.set_cell(Coord::new(-1, -1), true);
        world.set_cell(Coord::new(0, -1), true);
        world.set_cell(Coord::new(1, -1), true);

        assert_eq!(world.count_neighbors(center), 3);
    }

    #[test]
    fn test_get_bounds() {
        let mut world = World::new();
        assert!(world.get_bounds().is_none());

        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(100, 200), true);
        world.set_cell(Coord::new(-50, -100), true);

        let (min, max) = world.get_bounds().unwrap();
        assert_eq!(min, Coord::new(-50, -100));
        assert_eq!(max, Coord::new(100, 200));
    }
}
