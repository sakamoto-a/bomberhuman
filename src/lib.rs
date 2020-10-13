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
        let width = 100.0;
        let height = 100.0;
        let mut game_state = GameState::new(Size::new(width, height));
        let mut actions = Actions::new();
        GameData { game_state, actions }
    }

    pub fn update(mut self, dt: f64) {
        //let actions = self.actions.copy();
        self.game_state.update(dt, &self.actions);
    }

    pub fn width(&self) -> f64 {
        self.game_state.world.size.width
    }

    pub fn height(&self) -> f64 {
        self.game_state.world.size.height
    }

    pub fn actions_left(&self) -> bool {
        self.actions.move_left
    }

}
