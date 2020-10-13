pub mod game_state;
pub mod models;
pub mod geometory;
pub mod controller;

use self::game_state::GameState;
use self::geometory::Size;
use self::controller::Actions;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct GameData {
    game_state: GameState,
    actions: Actions,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        let width = 1024.0;
        let height = 840.0;
        let mut game_state = GameState::new(Size::new(width, height));
        let mut actions = Actions::new();
        GameData { game_state, actions }
    }

    pub fn update(&mut self, dt: f64) {
        self.game_state.update(dt, &self.actions);
    }

    pub fn width(&self) -> f64 {
        self.game_state.world.size.width
    }

    pub fn height(&self) -> f64 {
        self.game_state.world.size.height
    }

    pub fn actions(&mut self, event: &str, flag: i32) {
        match event{
            "move_left" => self.actions.move_left = int_to_bool(flag),
            "move_right" => self.actions.move_right = int_to_bool(flag),
            "move_up" => self.actions.move_up = int_to_bool(flag),
            "move_down" => self.actions.move_down = int_to_bool(flag),
            _ => (),
        }
    }
    pub fn x(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].x()
    }

    pub fn y(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].y()
    }

    pub fn angle(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].angle()
    }
}

fn int_to_bool(i: i32) -> bool {
    i != 0
}
