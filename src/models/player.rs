use crate::geometory::Point;
use crate::controller::Buttons;
use crate::controller::Actions;
use std::f64::consts::PI;

pub struct Player {
    position: Point,
    speed: f64,
    direction: f64,
    actions: Actions
}

impl Player {
    pub fn new(point: Point, speed: f64, direction: f64, actions: Actions) -> Player {
        Player {
            position: point,
            speed: speed,
            direction: direction,
            actions: actions,
        }
    }

    pub fn update(&mut self, dt: &f64, buttons: &Buttons) {
        for button in &buttons.button {
          if button == &self.actions.move_up {
              *self.y() -= dt * self.speed;
              *self.angle() = PI * 3.0 / 2.0;
          }
          if button == &self.actions.move_down {
            *self.y() += dt * self.speed;
            *self.angle() = PI / 2.0;
          }
          if button == &self.actions.move_left {
            *self.x() -= dt * self.speed;
            *self.angle() = PI;
          }
          if button == &self.actions.move_right {
            *self.x() += dt * self.speed;
            *self.angle() = 0.0;
          }
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
