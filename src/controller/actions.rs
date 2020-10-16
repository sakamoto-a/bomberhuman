pub struct Actions {
    pub move_up: &'static str,
    pub move_down: &'static str,
    pub move_left: &'static str,
    pub move_right: &'static str,
    pub put_bomb: &'static str,
}

impl Actions {
    pub fn new(up: &'static str, down: &'static str, left: &'static str, right: &'static str, bomb: &'static str) -> Actions {
        Actions {
            move_up: up,
            move_down: down,
            move_left: left,
            move_right: right,
            put_bomb: bomb,
        }
    }
}
