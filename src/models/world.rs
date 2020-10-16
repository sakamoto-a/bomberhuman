use crate::models::Player;
use crate::models::Bomb;
use crate::models::Fire;
use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Actions;
use crate::controller::Buttons;
use crate::controller::Events;

pub struct World {
    pub players: Vec<Player>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub events: Vec<Events>,
    pub size: Size,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut players: Vec<Player> = Vec::new();
        let bombs: Vec<Bomb> = Vec::new();
        let fires: Vec<Fire> = Vec::new();
        let events: Vec<Events> = Vec::new();
        for n in 0..2 {
            match n {
                0 => {
                    let point = Point::new(0.0, 0.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("up", "down", "left", "right", "l"), 0));
                },
                1 => {
                    let point = Point::new(500.0, 300.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("w", "s", "a", "d", "x"), 1));
                },
                _ => (),
            }
        }
        World {
            players: players,
            bombs: bombs,
            fires: fires,
            events: events,
            size: size,
        }
    }

    pub fn update(&mut self, dt: f64, buttons: &Buttons) {
        for player in &mut self.players {
            player.update(&dt, buttons, &mut self.events);
        }
        for bomb in &mut self.bombs {
            bomb.update(&dt, &mut self.events);
        }
        for fire in &mut self.fires {
            fire.update(&dt,&mut self.events);
        }
        for event in &mut self.events {
            match event.event {
              "bn" => {
                if World::can_put_bomb(&mut self.bombs, event.position) {
                  self.bombs.push(Bomb::new(event.position, event.firepower, event.player_id));
                  self.players[event.player_id].sub_bomb_num();
                }
              },
              "br" => {
                self.bombs.retain(|x| x.life > 0.0);
              },
              "fn" => {
                self.fires.push(Fire::new(event.position, event.direction, event.firepower, event.player_id));
              },
              "fr" => {
                self.fires.remove(0);
                if event.direction == 0 {
                  self.players[event.player_id].add_bomb_num();
                }
              },
              _ => (),
            }
        }
        for bomb in &mut self.bombs {
          if World::is_explosion(bomb, &mut self.fires) {
            bomb.life = 0.0;
          }
        }
        self.events.clear();
    }

    pub fn get_player_num(&mut self) -> usize {
        self.players.len()
    }

    pub fn get_bomb_num(&mut self) -> usize {
        self.bombs.len()
    }
    
    pub fn get_fire_num(&mut self) -> usize {
        self.fires.len()
    }

    pub fn push_bomb(&mut self, bomb: Bomb) {
        self.bombs.push(bomb);
    }

    pub fn push_fire(&mut self, fire: Fire) {
        self.fires.push(fire);
    }

    pub fn can_put_bomb(bombs: &mut Vec<Bomb>,position: Point) -> bool{
      for bomb in bombs {
        if bomb.position.x - position.x < bomb.size.width && position.x - bomb.position.x < bomb.size.width && bomb.position.y - position.y < bomb.size.height && position.y - bomb.position.y < bomb.size.height {
          return false;
        }
      }
      return true;
    }

    pub fn is_explosion(bomb: &mut Bomb, fires: &mut Vec<Fire>) -> bool{
      for fire in fires {
        if bomb.position.x - fire.position.x < (bomb.size.width + fire.size.width)/2.0 && fire.position.x - bomb.position.x < (bomb.size.width + fire.size.width)/2.0 && bomb.position.y - fire.position.y < (bomb.size.height + fire.size.height)/2.0 && fire.position.y - bomb.position.y < (bomb.size.height + fire.size.height)/2.0 {
          return true;
        }
      }
      return false;
    }
}
