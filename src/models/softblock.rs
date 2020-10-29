use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Events;

pub struct Softblock {
    pub position: Point,
    pub size: Size,
    pub dead: bool
}

impl Softblock {
    pub fn new(point: Point) -> Softblock {
      let size = Size::new(50.0, 50.0);
        Softblock {
            position: point,
            size: size,
            dead: false,
        }
    }

    pub fn update(&mut self, events: &mut Vec<Events>) {
      if self.dead {
        events.push(Events::new("sbr", self.position, 0, 0, 0));
      }
    }


    pub fn position(&self) -> Point {
        self.position
    }

    pub fn x(&mut self) -> &mut f64{
        &mut self.position.x
    }

    pub fn y(&mut self) -> &mut f64{
        &mut self.position.y
    }

    pub fn remove(&mut self) {
      self.dead = true;
    }
}
