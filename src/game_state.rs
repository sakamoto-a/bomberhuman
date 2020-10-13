use std::fmt;
use crate::models::World;
use crate::geometory::Size;
use crate::controller::Actions;

pub struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new(size: Size) -> GameState {
        GameState {
            world: World::new(size),
        }
    }

    pub fn update(&mut self, dt: f64, actions: &Actions) {
        self.world.update(dt, actions);
    }
}