use crate::world::{Coord, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    
    pub name: String,
    
    pub description: String,
    
    pub cells: Vec<(i32, i32)>,
}

impl Preset {
    pub fn new(name: impl Into<String>, description: impl Into<String>, cells: Vec<(i32, i32)>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            cells,
        }
    }
    
    pub fn load_into(&self, world: &mut World, offset: Coord) {
        world.clear();
        for &(x, y) in &self.cells {
            let coord = Coord::new(offset.x + x, offset.y + y);
            world.set_cell(coord, true);
        }
    }
    
    pub fn to_world(&self) -> World {
        let mut world = World::new();
        for &(x, y) in &self.cells {
            world.set_cell(Coord::new(x, y), true);
        }
        world
    }
  
    pub fn all() -> Vec<Preset> {
        vec![
            Self::empty(),
            Self::random_small(),
            Self::random_medium(),
            Self::blinker(),
            Self::toad(),
            Self::beacon(),
            Self::pentadecathlon(),
            Self::pulsar(),
            Self::glider(),
            Self::lightweight_spaceship(),
            Self::mwss(),
            Self::hwss(),
            Self::gosper_glider_gun(),
            Self::r_pentomino(),
            Self::diehard(),
            Self::acorn(),
            Self::block(),
            Self::beehive(),
            Self::loaf(),
        ]
    }
 
    pub fn find(name: &str) -> Option<Preset> {
        Self::all().into_iter().find(|p| p.name.to_lowercase() == name.to_lowercase())
    }
}

impl Preset { 
    pub fn empty() -> Self {
        Self::new(
            "Empty",
            "Пустой мир без активных ячеек",
            vec![],
        )
    }

    pub fn random_small() -> Self {
        let mut cells = Vec::new();
        for x in -10..=10 {
            for y in -10..=10 {
                if (x * 7 + y * 13) % 3 == 0 {
                    cells.push((x, y));
                }
            }
        }
        Self::new(
            "Random Small",
            "Случайное распределение ячеек (20x20)",
            cells,
        )
    }

    pub fn random_medium() -> Self {
        let mut cells = Vec::new();
        for x in -25..=25 {
            for y in -25..=25 {
                if (x * 11 + y * 17) % 4 == 0 {
                    cells.push((x, y));
                }
            }
        }
        Self::new(
            "Random Medium",
            "Случайное распределение ячеек (50x50)",
            cells,
        )
    }
    
    pub fn blinker() -> Self {
        Self::new(
            "Blinker",
            "Простой осциллятор с периодом 2",
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
            ],
        )
    }
    
    pub fn toad() -> Self {
        Self::new(
            "Toad",
            "Осциллятор 'жаба' с периодом 2",
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (0, 1),
                (1, 1),
                (2, 1),
            ],
        )
    }
    
    pub fn beacon() -> Self {
        Self::new(
            "Beacon",
            "Осциллятор 'маяк' с периодом 2",
            vec![
                (0, 0), (1, 0),
                (0, 1), (1, 1),
                (2, 2), (3, 2),
                (2, 3), (3, 3),
            ],
        )
    }
    
    pub fn pulsar() -> Self {
        Self::new(
            "Pulsar",
            "Красивый симметричный осциллятор периода 3",
            vec![
                
                (-6, -4), (-6, -3), (-6, -2),
                (-4, -6), (-3, -6), (-2, -6),
                (-4, -1), (-3, -1), (-2, -1),
                (-1, -4), (-1, -3), (-1, -2),
                
                (6, -4), (6, -3), (6, -2),
                (4, -6), (3, -6), (2, -6),
                (4, -1), (3, -1), (2, -1),
                (1, -4), (1, -3), (1, -2),
                
                (-6, 4), (-6, 3), (-6, 2),
                (-4, 6), (-3, 6), (-2, 6),
                (-4, 1), (-3, 1), (-2, 1),
                (-1, 4), (-1, 3), (-1, 2),
                
                (6, 4), (6, 3), (6, 2),
                (4, 6), (3, 6), (2, 6),
                (4, 1), (3, 1), (2, 1),
                (1, 4), (1, 3), (1, 2),
            ],
        )
    }
    
    pub fn glider() -> Self {
        Self::new(
            "Glider",
            "Классический глайдер - движется по диагонали",
            vec![
                (1, 0),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ],
        )
    }
 
    pub fn lightweight_spaceship() -> Self {
        Self::new(
            "LWSS",
            "Легкий космический корабль - движется горизонтально",
            vec![
                (1, 0),
                (4, 0),
                (0, 1),
                (0, 2),
                (4, 2),
                (0, 3),
                (1, 3),
                (2, 3),
                (3, 3),
            ],
        )
    }
  
    pub fn gosper_glider_gun() -> Self {
        Self::new(
            "Gosper Glider Gun",
            "Легендарная пушка глайдеров - производит бесконечный поток глайдеров",
            vec![
                
                (0, 4),
                (0, 5),
                (1, 4),
                (1, 5),
                
                (10, 4),
                (10, 5),
                (10, 6),
                (11, 3),
                (11, 7),
                (12, 2),
                (12, 8),
                (13, 2),
                (13, 8),
                (14, 5),
                (15, 3),
                (15, 7),
                (16, 4),
                (16, 5),
                (16, 6),
                (17, 5),
                
                (20, 2),
                (20, 3),
                (20, 4),
                (21, 2),
                (21, 3),
                (21, 4),
                (22, 1),
                (22, 5),
                (24, 0),
                (24, 1),
                (24, 5),
                (24, 6),
                
                (34, 2),
                (34, 3),
                (35, 2),
                (35, 3),
            ],
        )
    }
 
    pub fn block() -> Self {
        Self::new(
            "Block",
            "Блок 2x2 - простейшая стабильная структура",
            vec![
                (0, 0),
                (1, 0),
                (0, 1),
                (1, 1),
            ],
        )
    }

    
    pub fn beehive() -> Self {
        Self::new(
            "Beehive",
            "Улей - стабильная шестиугольная структура",
            vec![
                (1, 0),
                (2, 0),
                (0, 1),
                (3, 1),
                (1, 2),
                (2, 2),
            ],
        )
    }
   
    pub fn loaf() -> Self {
        Self::new(
            "Loaf",
            "Буханка - стабильная асимметричная структура",
            vec![
                (1, 0),
                (2, 0),
                (0, 1),
                (3, 1),
                (1, 2),
                (3, 2),
                (2, 3),
            ],
        )
    }
    
    pub fn pentadecathlon() -> Self {
        Self::new(
            "Pentadecathlon",
            "Осциллятор с периодом 15 - один из самых известных долгопериодных осцилляторов",
            vec![
                (0, 0), (1, 0), (2, 0), (3, 0), (4, 0),
                (5, 0), (6, 0), (7, 0), (8, 0), (9, 0),
            ],
        )
    }

    pub fn mwss() -> Self {
        Self::new(
            "MWSS",
            "Средний космический корабль - движется горизонтально",
            vec![
                (3, 0),
                (0, 1), (4, 1),
                (5, 2),
                (0, 3), (5, 3),
                (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
            ],
        )
    }
    
    pub fn hwss() -> Self {
        Self::new(
            "HWSS",
            "Тяжёлый космический корабль - крупнейший стандартный корабль",
            vec![
                (3, 0), (4, 0),
                (0, 1), (5, 1),
                (6, 2),
                (0, 3), (6, 3),
                (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4),
            ],
        )
    }
   
    pub fn r_pentomino() -> Self {
        Self::new(
            "R-pentomino",
            "Метузелах из 5 клеток - эволюционирует 1103 поколения перед стабилизацией",
            vec![
                (1, 0), (2, 0),
                (0, 1), (1, 1),
                (1, 2),
            ],
        )
    }
    
    pub fn diehard() -> Self {
        Self::new(
            "Diehard",
            "Метузелах, который полностью исчезает после 130 поколений",
            vec![
                (6, 0),
                (0, 1), (1, 1),
                (1, 2), (5, 2), (6, 2), (7, 2),
            ],
        )
    }

    pub fn acorn() -> Self {
        Self::new(
            "Acorn",
            "Метузелах из 7 клеток - стабилизируется после 5206 поколений",
            vec![
                (1, 0),
                (3, 1),
                (0, 2), (1, 2), (4, 2), (5, 2), (6, 2),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_new() {
        let preset = Preset::new("Test", "Test preset", vec![(0, 0), (1, 1)]);
        assert_eq!(preset.name, "Test");
        assert_eq!(preset.description, "Test preset");
        assert_eq!(preset.cells.len(), 2);
    }

    #[test]
    fn test_preset_to_world() {
        let preset = Preset::blinker();
        let world = preset.to_world();
        assert_eq!(world.active_cell_count(), 3);
    }

    #[test]
    fn test_preset_load_into() {
        let preset = Preset::block();
        let mut world = World::new();
        preset.load_into(&mut world, Coord::new(10, 10));

        assert_eq!(world.active_cell_count(), 4);
        assert!(world.get_cell(Coord::new(10, 10)));
        assert!(world.get_cell(Coord::new(11, 10)));
        assert!(world.get_cell(Coord::new(10, 11)));
        assert!(world.get_cell(Coord::new(11, 11)));
    }

    #[test]
    fn test_preset_load_into_clears_world() {
        let preset = Preset::blinker();
        let mut world = World::new();

        world.set_cell(Coord::new(100, 100), true);
        world.set_cell(Coord::new(200, 200), true);
        assert_eq!(world.active_cell_count(), 2);

        preset.load_into(&mut world, Coord::new(0, 0));
        assert_eq!(world.active_cell_count(), 3); 
    }

    #[test]
    fn test_empty_preset() {
        let preset = Preset::empty();
        let world = preset.to_world();
        assert_eq!(world.active_cell_count(), 0);
    }

    #[test]
    fn test_blinker_preset() {
        let preset = Preset::blinker();
        assert_eq!(preset.cells.len(), 3);
        assert_eq!(preset.name, "Blinker");
    }

    #[test]
    fn test_glider_preset() {
        let preset = Preset::glider();
        assert_eq!(preset.cells.len(), 5);
        let world = preset.to_world();
        assert_eq!(world.active_cell_count(), 5);
    }

    #[test]
    fn test_gosper_glider_gun_preset() {
        let preset = Preset::gosper_glider_gun();
        assert_eq!(preset.cells.len(), 36); 
        let world = preset.to_world();
        assert_eq!(world.active_cell_count(), 36);
    }

    #[test]
    fn test_pulsar_preset() {
        let preset = Preset::pulsar();
        assert_eq!(preset.cells.len(), 48); 
    }

    #[test]
    fn test_all_presets() {
        let presets = Preset::all();
        assert!(presets.len() >= 19); 

        for preset in &presets {
            assert!(!preset.name.is_empty());
            assert!(!preset.description.is_empty());
        }
    }

    #[test]
    fn test_find_preset() {
        
        assert!(Preset::find("blinker").is_some());
        assert!(Preset::find("BLINKER").is_some());
        assert!(Preset::find("Blinker").is_some());

        assert!(Preset::find("Glider").is_some());
        assert!(Preset::find("Empty").is_some());
        assert!(Preset::find("Gosper Glider Gun").is_some());
        
        assert!(Preset::find("NonExistent").is_none());
    }

    #[test]
    fn test_preset_offset() {
        let preset = Preset::block();
        let mut world = World::new();

        preset.load_into(&mut world, Coord::new(-10, -10));

        assert!(world.get_cell(Coord::new(-10, -10)));
        assert!(world.get_cell(Coord::new(-9, -10)));
        assert!(world.get_cell(Coord::new(-10, -9)));
        assert!(world.get_cell(Coord::new(-9, -9)));
        assert!(!world.get_cell(Coord::new(0, 0)));
    }

    #[test]
    fn test_random_presets_are_different() {
        let small = Preset::random_small();
        let medium = Preset::random_medium();

        assert_ne!(small.cells.len(), medium.cells.len());
        assert!(medium.cells.len() > small.cells.len());
    }

    #[test]
    fn test_stable_structures() {
        
        let preset = Preset::block();
        assert_eq!(preset.cells.len(), 4);

        
        let preset = Preset::beehive();
        assert_eq!(preset.cells.len(), 6);

        
        let preset = Preset::loaf();
        assert_eq!(preset.cells.len(), 7);
    }

    #[test]
    fn test_oscillators() {
        
        let preset = Preset::toad();
        assert_eq!(preset.cells.len(), 6);
        
        let preset = Preset::beacon();
        assert_eq!(preset.cells.len(), 8);
       
        let preset = Preset::pentadecathlon();
        assert_eq!(preset.cells.len(), 10);
    }

    #[test]
    fn test_spaceships() {
        
        let preset = Preset::lightweight_spaceship();
        assert_eq!(preset.cells.len(), 9);
        
        let preset = Preset::mwss();
        assert_eq!(preset.cells.len(), 11);

        let preset = Preset::hwss();
        assert_eq!(preset.cells.len(), 13);
    }

    #[test]
    fn test_methuselahs() {
        let preset = Preset::r_pentomino();
        assert_eq!(preset.cells.len(), 5);

        let preset = Preset::diehard();
        assert_eq!(preset.cells.len(), 7);

        let preset = Preset::acorn();
        assert_eq!(preset.cells.len(), 7);
    }
}
