use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Events;

//#define BOMB_LIFE 3

pub struct Fire {
    pub position: Point,
    direction: i8,
    firepower: i8,
    bomb_type: i8,
    life: f64,
    pub size: Size,
    flag: bool,
    player_id: usize
}

impl Fire {
    pub fn new(point: Point, direction: i8, firepower:i8, player_id: usize, bomb_type: i8) -> Fire {
        let size = Size::new(50.0, 50.0);
        Fire {
            position: point,
            direction: direction,
            firepower: firepower,
            bomb_type: bomb_type,
            life: 0.3,
            size: size,
            flag: true,
            player_id: player_id,
        }
    }

    pub fn update(&mut self, dt: &f64, events: &mut Vec<Events>) {
      if self.flag && self.firepower > 0 {
        let mut new_position = Point::new(0.0, 0.0);
        match self.direction {
          0 => {
            new_position.x = self.position.x;
            new_position.y = self.position.y - self.size.height;
            events.push(Events::new("fn", new_position, 1, self.firepower-1, self.player_id, self.bomb_type));
            new_position.x = self.position.x;
            new_position.y = self.position.y + self.size.height;
            events.push(Events::new("fn", new_position, 2, self.firepower-1, self.player_id, self.bomb_type));
            new_position.x = self.position.x - self.size.width;
            new_position.y = self.position.y;
            events.push(Events::new("fn", new_position, 3, self.firepower-1, self.player_id, self.bomb_type));
            new_position.x = self.position.x + self.size.width;
            new_position.y = self.position.y;
            events.push(Events::new("fn", new_position, 4 , self.firepower-1, self.player_id, self.bomb_type));
          },
          1 => {
            new_position.x = self.position.x;
            new_position.y = self.position.y - self.size.height;
            events.push(Events::new("fn", new_position, self.direction , self.firepower-1, self.player_id, self.bomb_type));
          },
          2 => {
            new_position.x = self.position.x;
            new_position.y = self.position.y + self.size.height;
            events.push(Events::new("fn", new_position, self.direction , self.firepower-1, self.player_id, self.bomb_type));
          }
          3 => {
            new_position.x = self.position.x - self.size.width;
            new_position.y = self.position.y;
            events.push(Events::new("fn", new_position, self.direction , self.firepower-1, self.player_id, self.bomb_type));
          },
          4 => {
            new_position.x = self.position.x + self.size.width;
            new_position.y = self.position.y;
            events.push(Events::new("fn", new_position, self.direction , self.firepower-1, self.player_id, self.bomb_type));
          },
          _ => (),
        }

        self.flag = false;
      }
       self.life -= dt;
       if self.life < 0.0 {
         self.remove(events)
       }
    }

    fn remove(&mut self, events: &mut Vec<Events>) {
        events.push(Events::new("fr", self.position,self.direction ,0, self.player_id, self.bomb_type));
    }

    pub fn x(&mut self) -> &mut f64{
        &mut self.position.x
    }

    pub fn y(&mut self) -> &mut f64{
        &mut self.position.y
    }
}
