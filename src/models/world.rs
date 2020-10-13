use rand::Rng;
use crate::models::Player;
use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Actions;

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
                    let mut point = Point::new(0.0, 0.0);
                    players.push(Player::new(point, 150.0, 0.0));
                },
                1 => {
                    let mut point = Point::new(500.0, 300.0);
                    players.push(Player::new(point, 150.0, 0.0));
                },
                _ => (),
            }
        }
        World {
            players: players,
            size: size,
        }
    }

    pub fn update(&mut self, dt: f64, actions: &Actions) {
        for player in &mut self.players {
            player.update(&dt, actions);
        }
    }

}