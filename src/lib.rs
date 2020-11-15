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

    //pub fn field(&self) -> Vec<Block> {
    //    self.game_state.world.field
    //}

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
            "/" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "/");
                    self.buttons.button.push("/");
                } else {
                    self.buttons.button.retain(|&x| x != "/");
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
            "t" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "t");
                    self.buttons.button.push("t");
                } else {
                    self.buttons.button.retain(|&x| x != "t");
                }
            },
            "g" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "g");
                    self.buttons.button.push("g");
                } else {
                    self.buttons.button.retain(|&x| x != "g");
                }
            },
            "f" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "f");
                    self.buttons.button.push("f");
                } else {
                    self.buttons.button.retain(|&x| x != "f");
                }
            },
            "h" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "h");
                    self.buttons.button.push("h");
                } else {
                    self.buttons.button.retain(|&x| x != "h");
                }
            },
            "b" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "b");
                    self.buttons.button.push("b");
                } else {
                    self.buttons.button.retain(|&x| x != "b");
                }
            },
            "i" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "i");
                    self.buttons.button.push("i");
                } else {
                    self.buttons.button.retain(|&x| x != "i");
                }
            },
            "k" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "k");
                    self.buttons.button.push("k");
                } else {
                    self.buttons.button.retain(|&x| x != "k");
                }
            },
            "j" => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != "j");
                    self.buttons.button.push("j");
                } else {
                    self.buttons.button.retain(|&x| x != "j");
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
            "," => {
                if flag == 1 {
                    self.buttons.button.retain(|&x| x != ",");
                    self.buttons.button.push(",");
                } else {
                    self.buttons.button.retain(|&x| x != ",");
                }
            },
            _ => (),
        }
    }

    pub fn x(&mut self, num: usize, char_name: &str) -> f64 {
        match char_name {
            "player" => {
                *self.game_state.world.players[num].x()
            },
            "bomb" => {
                *self.game_state.world.bombs[num].x()
            },
            "fire" => {
                *self.game_state.world.fires[num].x()
            },
            "item" => {
                *self.game_state.world.items[num].x()
            }
            "block" => {
                *self.game_state.world.blocks[num].x()
            }
            "softblock" => {
                *self.game_state.world.softblocks[num].x()
            }
            _ => 0.0,
        }
    }

    pub fn y(&mut self, num: usize, char_name: &str) -> f64 {
        match char_name {
            "player" => {
                *self.game_state.world.players[num].y()
            },
            "bomb" => {
                *self.game_state.world.bombs[num].y()
            },
            "fire" => {
                *self.game_state.world.fires[num].y()
            },
            "item" => {
                *self.game_state.world.items[num].y()
            },
            "block" => {
                *self.game_state.world.blocks[num].y()
            },
            "softblock" => {
                *self.game_state.world.softblocks[num].y()
            },

            _ => 0.0,
        }
    }

    pub fn what_type(&mut self, num:usize, char_name: &str) -> usize {
      match char_name {
        "item" => {
          self.game_state.world.items[num].get_item_type()
        },
        "bomb" => {
          self.game_state.world.bombs[num].bomb_type as usize
        }
        _ => 0,
      }
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
//debug
    pub fn get_fire_item_num(&mut self) -> usize {
        self.game_state.world.players[0].items.fire_up as usize
    }
    pub fn get_bomb_item_num(&mut self) -> usize {
        self.game_state.world.players[0].items.bomb_up as usize
    }

    pub fn get_speed_item_num(&mut self) -> usize {
        self.game_state.world.players[0].items.speed_up as usize
    }
    pub fn get_kick_item_num(&mut self) -> usize {
        self.game_state.world.players[0].items.kick as usize
    }
    pub fn get_uni_item_num(&mut self) -> usize {
        self.game_state.world.players[0].items.bomb_type as usize
    }
//
    pub fn get_winner(&mut self) -> usize {
        self.game_state.world.get_winner()
    }

    pub fn get_item_num(&mut self) -> usize {
        self.game_state.world.get_item_num()
    }

    pub fn get_block_num(&mut self) -> usize {
        self.game_state.world.get_block_num()
    }

    pub fn get_softblock_num(&mut self) -> usize {
        self.game_state.world.get_softblock_num()
    }

    pub fn get_time(&mut self) -> f64 {
      self.game_state.world.life
    }

    pub fn is_end(&mut self) -> bool {
      self.game_state.world.end
    }
}
