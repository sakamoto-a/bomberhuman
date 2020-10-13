use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Actions {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool
}

impl Actions {
    pub fn new() -> Actions {
        Actions {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
        }
    }
}