pub struct Items {
    pub bomb_up: i8,
    pub fire_up: i8,
    pub speed_up: i8,
    pub kick: i8,
    pub uni_bomb: i8,
    pub gomu_bomb: i8,
}

impl Items {
    pub fn new() -> Items {
        Items {
          bomb_up: 0,
          fire_up: 0,
          speed_up: 0,
          kick: 0,
          uni_bomb: 0,
          gomu_bomb: 0,
        }
    }
}
