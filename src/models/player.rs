use crate::geometory::Point;
use crate::controller::Actions;
use std::f64::consts::PI;

pub struct Player {
    position: Point,
    speed: f64,
    direction: f64,
}

impl Player {
    pub fn new(point: Point, speed: f64, direction: f64) -> Player {
        Player {
            position: point,
            speed: speed,
            direction: direction,
        }
    }

    pub fn update(&mut self, dt: &f64, actions: &Actions) {
        if actions.move_up {
            *self.y() -= dt * self.speed;
            *self.angle() = PI * 3.0 / 2.0;
        }
        if actions.move_down {
            *self.y() += dt * self.speed;
            *self.angle() = PI / 2.0;
        }
        if actions.move_left {
            *self.x() -= dt * self.speed;
            *self.angle() = PI;
        }
        if actions.move_right {
            *self.x() += dt * self.speed;
            *self.angle() = 0.0;
        }
    }

    pub fn x(&mut self) -> &mut f64{
        &mut self.position.x
    }

    pub fn y(&mut self) -> &mut f64{
        &mut self.position.y
    }

    pub fn angle(&mut self) -> &mut f64{
        &mut self.direction
    }
}
