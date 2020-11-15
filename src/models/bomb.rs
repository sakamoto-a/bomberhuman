use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Events;
//#define BOMB_LIFE 3

pub struct Bomb {
    pub bomb_id: usize,
    pub position: Point,
    firepower: i8,
    pub life: f64,
    player_id: usize,
    pub size: Size,
    pub over_players: Vec<usize>,
    pub speed: f64,
    pub direction: i8,
    pub bomb_type: i8,
}

impl Bomb {
    pub fn new(bomb_id: usize, mut point: Point, firepower: i8, player_id: usize, over_players: Vec<usize>, bomb_type: i8) -> Bomb {
        let size = Size::new(50.0, 50.0);
        point.set_field_point();
            Bomb {
                bomb_id: bomb_id,
                position: point,
                firepower: firepower,
                life: 3.0,
                player_id: player_id,
                size: size,
                over_players: over_players,
                speed: 0.0,
                direction: 0,
                bomb_type: bomb_type,
            }
    }

    pub fn update(&mut self, dt: &f64, events: &mut Vec<Events>) {
        self.life -= dt;
        match self.direction{
//              *self.y() -= dt * self.speed;
            1 => {
              let next_position = Point::new(self.position.x, self.position.y - dt * self.speed);
              events.push(Events::new("bm", next_position, 1, 0, self.bomb_id, self.bomb_type));
            },
            2 => {
              let next_position = Point::new(self.position.x, self.position.y + dt * self.speed);
              events.push(Events::new("bm", next_position, 2, 0, self.bomb_id, self.bomb_type));
            },
            3 => {
              let next_position = Point::new(self.position.x - dt * self.speed, self.position.y);
              events.push(Events::new("bm", next_position, 3, 0, self.bomb_id, self.bomb_type));
          },
            4 => {
              let next_position = Point::new(self.position.x + dt * self.speed, self.position.y);
              events.push(Events::new("bm", next_position, 4, 0, self.bomb_id, self.bomb_type));
          },
          _ => (),
        }
        if self.life < 0.0 {
            self.position.set_field_point(); 
            self.remove(events);
            events.push(Events::new("fn", self.position, 0, self.firepower, self.player_id, self.bomb_type));
        }

    }

    pub fn move_start(&mut self, direction: i8) {
      self.speed = 250.0;
      self.direction = direction;
    }
    
    pub fn move_stop(&mut self) {
      self.speed = 0.0;
      self.direction = 0;
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn remove(&mut self, events: &mut Vec<Events>) {
        events.push(Events::new("br", self.position, 0, 0, self.player_id, self.bomb_type));
    }

    pub fn x(&mut self) -> &mut f64{
        &mut self.position.x
    }

    pub fn y(&mut self) -> &mut f64{
        &mut self.position.y
    }
    pub fn remove_over_player(&mut self, player_id: usize) {
      self.over_players.retain(|x| *x != player_id);
    }

    pub fn position_move(&mut self, next_position: Point) {
      self.position = next_position;
    }
}
