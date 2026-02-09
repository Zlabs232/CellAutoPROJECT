use crate::automaton::Rule;
use crate::world::{Coord, World};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationCommand {
    Start,
    Stop,
    Pause,
    Resume,
    Step,
    SetSpeed(u32),
    Shutdown,
}

#[derive(Clone)]
pub struct Simulation {
    world: Arc<Mutex<World>>,

    state: Arc<Mutex<SimulationState>>,

    tick_count: Arc<Mutex<u64>>,

    tps: Arc<Mutex<u32>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(World::new())),
            state: Arc::new(Mutex::new(SimulationState::Stopped)),
            tick_count: Arc::new(Mutex::new(0)),
            tps: Arc::new(Mutex::new(10)),
        }
    }

    pub fn get_world(&self) -> World {
        self.world.lock().unwrap().clone()
    }

    pub fn set_world(&self, world: World) {
        let mut w = self.world.lock().unwrap();
        *w = world;
    }

    pub fn get_state(&self) -> SimulationState {
        *self.state.lock().unwrap()
    }

    pub fn get_tick_count(&self) -> u64 {
        *self.tick_count.lock().unwrap()
    }

    pub fn get_tps(&self) -> u32 {
        *self.tps.lock().unwrap()
    }

    pub fn set_tps(&self, new_tps: u32) {
        let tps = new_tps.clamp(1, 1000);
        let mut current_tps = self.tps.lock().unwrap();
        *current_tps = tps;
    }

    pub fn step<R: Rule>(&self, rule: &R) {
        let mut world = self.world.lock().unwrap();
        let next_world = rule.apply(&*world);
        *world = next_world;

        let mut tick = self.tick_count.lock().unwrap();
        *tick += 1;
    }

    pub fn reset_tick_count(&self) {
        let mut tick = self.tick_count.lock().unwrap();
        *tick = 0;
    }

    pub fn run<R: Rule + 'static>(
        &self,
        rule: Arc<R>,
        command_rx: std::sync::mpsc::Receiver<SimulationCommand>,
    ) -> thread::JoinHandle<()> {
        let sim = self.clone();

        thread::spawn(move || {
            loop {
                if let Ok(cmd) = command_rx.try_recv() {
                    match cmd {
                        SimulationCommand::Start => {
                            let mut state = sim.state.lock().unwrap();
                            *state = SimulationState::Running;
                        }
                        SimulationCommand::Stop => {
                            let mut state = sim.state.lock().unwrap();
                            *state = SimulationState::Stopped;
                            sim.reset_tick_count();
                        }
                        SimulationCommand::Pause => {
                            let mut state = sim.state.lock().unwrap();
                            *state = SimulationState::Paused;
                        }
                        SimulationCommand::Resume => {
                            let mut state = sim.state.lock().unwrap();
                            if *state == SimulationState::Paused {
                                *state = SimulationState::Running;
                            }
                        }
                        SimulationCommand::Step => {
                            sim.step(rule.as_ref());
                        }
                        SimulationCommand::SetSpeed(tps) => {
                            sim.set_tps(tps);
                        }
                        SimulationCommand::Shutdown => {
                            break;
                        }
                    }
                }

                let state = sim.get_state();
                if state == SimulationState::Running {
                    let start = Instant::now();

                    sim.step(rule.as_ref());

                    let tps = sim.get_tps();
                    let tick_duration = Duration::from_millis(1000 / tps as u64);
                    let elapsed = start.elapsed();

                    if elapsed < tick_duration {
                        thread::sleep(tick_duration - elapsed);
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        })
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automaton::GameOfLife;

    #[test]
    fn test_simulation_new() {
        let sim = Simulation::new();
        assert_eq!(sim.get_state(), SimulationState::Stopped);
        assert_eq!(sim.get_tick_count(), 0);
        assert_eq!(sim.get_tps(), 10);
    }

    #[test]
    fn test_set_get_world() {
        let sim = Simulation::new();
        let mut world = World::new();
        world.set_cell(Coord::new(5, 5), true);

        sim.set_world(world.clone());
        let retrieved = sim.get_world();

        assert_eq!(retrieved.active_cell_count(), 1);
        assert!(retrieved.get_cell(Coord::new(5, 5)));
    }

    #[test]
    fn test_set_tps() {
        let sim = Simulation::new();

        sim.set_tps(50);
        assert_eq!(sim.get_tps(), 50);

        sim.set_tps(0);
        assert_eq!(sim.get_tps(), 1); 

        sim.set_tps(2000);
        assert_eq!(sim.get_tps(), 1000); 
    }

    #[test]
    fn test_step() {
        let sim = Simulation::new();
        let rule = GameOfLife::new();

        let mut world = World::new();
        world.set_cell(Coord::new(0, -1), true);
        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(0, 1), true);
        sim.set_world(world);

        assert_eq!(sim.get_tick_count(), 0);

        sim.step(&rule);

        assert_eq!(sim.get_tick_count(), 1);
        let world_after = sim.get_world();
        assert_eq!(world_after.active_cell_count(), 3);
    }

    #[test]
    fn test_reset_tick_count() {
        let sim = Simulation::new();
        let rule = GameOfLife::new();

        let mut world = World::new();
        world.set_cell(Coord::new(0, 0), true);
        sim.set_world(world);

        sim.step(&rule);
        sim.step(&rule);
        assert_eq!(sim.get_tick_count(), 2);

        sim.reset_tick_count();
        assert_eq!(sim.get_tick_count(), 0);
    }

    #[test]
    fn test_simulation_commands() {
        let sim = Simulation::new();
        let rule = Arc::new(GameOfLife::new());

        let (tx, rx) = std::sync::mpsc::channel();

        let handle = sim.run(rule, rx);

        assert_eq!(sim.get_state(), SimulationState::Stopped);

        let mut world = World::new();
        world.set_cell(Coord::new(0, 0), true);
        world.set_cell(Coord::new(1, 0), true);
        world.set_cell(Coord::new(0, 1), true);
        world.set_cell(Coord::new(1, 1), true);
        sim.set_world(world);

        tx.send(SimulationCommand::Start).unwrap();
        thread::sleep(Duration::from_millis(150));
        assert_eq!(sim.get_state(), SimulationState::Running);

        thread::sleep(Duration::from_millis(300));
        assert!(sim.get_tick_count() > 0);

        tx.send(SimulationCommand::Pause).unwrap();
        thread::sleep(Duration::from_millis(150));
        assert_eq!(sim.get_state(), SimulationState::Paused);

        let ticks_paused = sim.get_tick_count();
        thread::sleep(Duration::from_millis(200));
        assert_eq!(sim.get_tick_count(), ticks_paused);

        tx.send(SimulationCommand::Resume).unwrap();
        thread::sleep(Duration::from_millis(150));
        assert_eq!(sim.get_state(), SimulationState::Running);

        tx.send(SimulationCommand::Stop).unwrap();
        thread::sleep(Duration::from_millis(150));
        assert_eq!(sim.get_state(), SimulationState::Stopped);
        assert_eq!(sim.get_tick_count(), 0); 

        tx.send(SimulationCommand::Shutdown).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_step_command() {
        let sim = Simulation::new();
        let rule = Arc::new(GameOfLife::new());

        let (tx, rx) = std::sync::mpsc::channel();

        let mut world = World::new();
        world.set_cell(Coord::new(0, 0), true);
        sim.set_world(world);

        let handle = sim.run(rule, rx);

        tx.send(SimulationCommand::Step).unwrap();
        thread::sleep(Duration::from_millis(50));

        assert_eq!(sim.get_tick_count(), 1);
        assert_eq!(sim.get_state(), SimulationState::Stopped);

        tx.send(SimulationCommand::Shutdown).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_set_speed_command() {
        let sim = Simulation::new();
        let rule = Arc::new(GameOfLife::new());

        let (tx, rx) = std::sync::mpsc::channel();
        let handle = sim.run(rule, rx);

        assert_eq!(sim.get_tps(), 10);

        tx.send(SimulationCommand::SetSpeed(100)).unwrap();
        thread::sleep(Duration::from_millis(50));

        assert_eq!(sim.get_tps(), 100);

        tx.send(SimulationCommand::Shutdown).unwrap();
        handle.join().unwrap();
    }
}
