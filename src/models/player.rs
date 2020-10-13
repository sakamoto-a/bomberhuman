use crate::geometory::Point;
use crate::controller::Actions;

pub struct Player {
    position: Point,
    speed: i32,
    direction: f64,
}

impl Player {
    pub fn new(point: Point, speed: i32, direction: f64) -> Player {
        Player {
            position: point,
            speed: speed,
            direction: direction,
        }
    }

    pub fn update(&mut self, dt: &f64, actions: &Actions) {
        //if actions.move_up {

        //}

        //if actions.move_down {
        //}

        //if actions.move_right {
        //}

        //if actions.move_left {
        //}
    }

    pub fn x(&self) -> f64{
        self.position.x
    }

    pub fn y(&self) -> f64{
        self.position.y
    }
}


