pub mod game_state;
pub mod models;
pub mod geometory;
pub mod controller;

use self::game_state::GameState;
use self::geometory::Size;
use self::controller::Buttons;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameData {
    game_state: GameState,
    buttons: Buttons,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        let width = 1024.0;
        let height = 840.0;
        let game_state = GameState::new(Size::new(width, height));
        let buttons = Buttons::new();
        GameData { game_state, buttons }
    }

    pub fn update(&mut self, dt: f64) {
        self.game_state.update(dt, &self.buttons);
    }

    pub fn width(&self) -> f64 {
        self.game_state.world.size.width
    }

    pub fn height(&self) -> f64 {
        self.game_state.world.size.height
    }

    pub fn buttons(&mut self, event: &str, flag: i32) {
        match event{
            "up" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "up");
                self.buttons.button.push("up");
              } else {
                self.buttons.button.retain(|&x| x != "up");
              }
            },
            "down" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "down");
                self.buttons.button.push("down");
              } else {
                self.buttons.button.retain(|&x| x != "down");
              }
            },
            "left" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "left");
                self.buttons.button.push("left");
              } else {
                self.buttons.button.retain(|&x| x != "left");
              }
            },
            "right" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "right");
                self.buttons.button.push("right");
              } else {
                self.buttons.button.retain(|&x| x != "right");
              }
            },
            "l" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "l");
                self.buttons.button.push("l");
              } else {
                self.buttons.button.retain(|&x| x != "l");
              }
            },
            "w" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "w");
                self.buttons.button.push("w");
              } else {
                self.buttons.button.retain(|&x| x != "w");
              }
            },
            "s" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "s");
                self.buttons.button.push("s");
              } else {
                self.buttons.button.retain(|&x| x != "s");
              }
            },
            "a" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "a");
                self.buttons.button.push("a");
              } else {
                self.buttons.button.retain(|&x| x != "a");
              }
            },
            "d" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "d");
                self.buttons.button.push("d");
              } else {
                self.buttons.button.retain(|&x| x != "d");
              }
            },
            "x" => {
              if flag == 1 {
                self.buttons.button.retain(|&x| x != "x");
                self.buttons.button.push("x");
              } else {
                self.buttons.button.retain(|&x| x != "x");
              }
            },
            _ => (),
        }
    }
    pub fn p_x(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].x()
    }

    pub fn p_y(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].y()
    }
    
    pub fn b_x(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.bombs[p_num].x()
    }

    pub fn b_y(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.bombs[p_num].y()
    }
    
    pub fn f_x(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.fires[p_num].x()
    }

    pub fn f_y(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.fires[p_num].y()
    }

    pub fn angle(&mut self, p_num: usize) -> f64 {
        *self.game_state.world.players[p_num].angle()
    }

    pub fn get_player_num(&mut self) -> usize {
      self.game_state.world.get_player_num()
    }
    pub fn get_bomb_num(&mut self) -> usize {
      self.game_state.world.get_bomb_num()
    }
    pub fn get_fire_num(&mut self) -> usize {
      self.game_state.world.get_fire_num()
    }
}
