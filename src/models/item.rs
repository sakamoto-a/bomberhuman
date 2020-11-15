use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Events;

pub struct Item {
    pub position: Point,
    pub size: Size,
    pub item_type: usize,
    pub dead: bool
}

impl Item {
    pub fn new(point: Point, item_type: usize) -> Item {
      let size = Size::new(50.0, 50.0);
        Item {
            position: point,
            size: size,
            item_type: item_type,
            dead: false,
        }
    }

    pub fn update(&mut self, events: &mut Vec<Events>) {
      if self.dead {
        events.push(Events::new("ir", self.position, 0, 0, 0, 0));
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

    pub fn get_item_type(&mut self) -> usize {
        self.item_type
    }

    pub fn remove(&mut self) {
      self.dead = true;
    }
}
