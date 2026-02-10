use crate::world::{Coord, World};
use std::collections::HashSet;

pub trait Rule: Send + Sync {
    fn apply(&self, current: &World) -> World;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct GameOfLife;

impl GameOfLife {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GameOfLife {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GameOfLife {
    fn apply(&self, current: &World) -> World {
        let mut next = World::new();
        let mut candidates = HashSet::new();
        
        for cell in current.iter_active_cells() {
            candidates.insert(cell);
            for neighbor in cell.neighbors() {
                candidates.insert(neighbor);
            }
        }

        for coord in candidates {
            let alive = current.get_cell(coord);
            let neighbors = current.count_neighbors(coord);

            let next_state = match (alive, neighbors) {
                
                (true, 2) | (true, 3) => true,
                
                (false, 3) => true,
                
                _ => false,
            };

            if next_state {
                next.set_cell(coord, true);
            }
        }
        next
    }
    fn name(&self) -> &str {
        "Conway's Game of Life"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life_name() {
        let gol = GameOfLife::new();
        assert_eq!(gol.name(), "Conway's Game of Life");
    }

    #[test]
    fn test_empty_world() {
        let gol = GameOfLife::new();
        let world = World::new();
        let next = gol.apply(&world);

        assert_eq!(next.active_cell_count(), 0);
    }

    #[test]
    fn test_single_cell_dies() {
        let gol = GameOfLife::new();
        let mut world = World::new();
        world.set_cell(Coord::new(0, 0), true);

        let next = gol.apply(&world);
        assert_eq!(next.active_cell_count(), 0);
    }

    #[test]
    fn test_block_stable() {
        
        let gol = GameOfLife::new();
        let mut world = World::new();

        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(1, 0), true);
        world.set_cell(Coord::new(0, 1), true);
        world.set_cell(Coord::new(1, 1), true);

        let next = gol.apply(&world);
        assert_eq!(next.active_cell_count(), 4);
        assert!(next.get_cell(Coord::new(0, 0)));
        assert!(next.get_cell(Coord::new(1, 0)));
        assert!(next.get_cell(Coord::new(0, 1)));
        assert!(next.get_cell(Coord::new(1, 1)));
    }

    #[test]
    fn test_blinker_oscillates() {
        
        let gol = GameOfLife::new();
        let mut world = World::new();

        world.set_cell(Coord::new(0, -1), true);
        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(0, 1), true);
  
        let next = gol.apply(&world);
        assert_eq!(next.active_cell_count(), 3);
        assert!(next.get_cell(Coord::new(-1, 0)));
        assert!(next.get_cell(Coord::new(0, 0)));
        assert!(next.get_cell(Coord::new(1, 0)));

        
        let next2 = gol.apply(&next);
        assert_eq!(next2.active_cell_count(), 3);
        assert!(next2.get_cell(Coord::new(0, -1)));
        assert!(next2.get_cell(Coord::new(0, 0)));
        assert!(next2.get_cell(Coord::new(0, 1)));
    }

    #[test]
    fn test_glider_moves() {
        
        let gol = GameOfLife::new();
        let mut world = World::new();

        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(1, 0), true);
        world.set_cell(Coord::new(2, 0), true);
        world.set_cell(Coord::new(2, 1), true);
        world.set_cell(Coord::new(1, 2), true);

        let initial_count = world.active_cell_count();
        let (min1, _) = world.get_bounds().unwrap();

        let mut current = world;
        for _ in 0..4 {
            current = gol.apply(&current);
        }

        assert_eq!(current.active_cell_count(), initial_count);

        let (min2, _) = current.get_bounds().unwrap();
        assert!(min1.x != min2.x || min1.y != min2.y);
    }

    #[test]
    fn test_overpopulation() {
        
        let gol = GameOfLife::new();
        let mut world = World::new();

        let center = Coord::new(0, 0);
        world.set_cell(center, true);

        world.set_cell(Coord::new(-1, 0), true);
        world.set_cell(Coord::new(1, 0), true);
        world.set_cell(Coord::new(0, -1), true);
        world.set_cell(Coord::new(0, 1), true);

        let next = gol.apply(&world);

        assert!(!next.get_cell(center));
    }

    #[test]
    fn test_birth() {
        
        let gol = GameOfLife::new();
        let mut world = World::new();

        let center = Coord::new(0, 0);

        world.set_cell(Coord::new(-1, 0), true);
        world.set_cell(Coord::new(1, 0), true);
        world.set_cell(Coord::new(0, -1), true);

        assert!(!world.get_cell(center));

        let next = gol.apply(&world);

        assert!(next.get_cell(center));
    }
}
