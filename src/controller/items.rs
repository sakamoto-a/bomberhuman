pub struct Items {
    pub bomb_up: i8,
    pub fire_up: i8,
    pub speed_up: i8,
    pub kick: i8,
    pub bomb_type: i8,
}

impl Items {
    pub fn new() -> Items {
        Items {
          bomb_up: 0,
          fire_up: 0,
          speed_up: 0,
          kick: 0,
          bomb_type: 0,
        }
    }
}
