use crate::models::Player;
use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Actions;
use crate::controller::Buttons;

pub struct World {
    pub players: Vec<Player>,
    pub size: Size,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut players: Vec<Player> = Vec::new();
        for n in 0..2 {
            match n {
                0 => {
                    let point = Point::new(0.0, 0.0);
                    let mut actions = Actions::new("up", "down", "left", "right");
                    players.push(Player::new(point, 150.0, 0.0, actions));
                },
                1 => {
                    let point = Point::new(500.0, 300.0);
                    let mut actions = Actions::new("w", "s", "a", "d");
                    players.push(Player::new(point, 150.0, 0.0, actions));
                },
                _ => (),
            }
        }
        World {
            players: players,
            size: size,
        }
    }

    pub fn update(&mut self, dt: f64, buttons: &Buttons) {
        for player in &mut self.players {
            player.update(&dt, buttons);
        }
    }

    pub fn get_player_num(&mut self) -> usize {
        self.players.len()
    }
}
