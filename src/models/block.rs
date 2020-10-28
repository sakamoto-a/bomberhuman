use crate::geometory::Point;
use crate::geometory::Size;

pub struct Block {
    pub position: Point,
    pub size: Size,
}

impl Block {
    pub fn new(point: Point) -> Block {
      let size = Size::new(50.0, 50.0);
        Block {
            position: point,
            size: size,
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
}
