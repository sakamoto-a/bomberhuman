use crate::geometory::Point;
use crate::controller::Buttons;
use crate::controller::Actions;
use crate::controller::Events;
use std::f64::consts::PI;

pub struct Player {
    position: Point,
    speed: f64,
    direction: f64,
    actions: Actions,
    bomb_num: i8,
    firepower: i8,
    player_id: usize,
    }

impl Player {
    pub fn new(point: Point, speed: f64, direction: f64, actions: Actions, player_id: usize) -> Player {
        Player {
            position: point,
            speed: speed,
            direction: direction,
            actions: actions,
            bomb_num: 4,
            firepower: 3,
            player_id: player_id,
        }
    }

    pub fn update(&mut self, dt: &f64, buttons: &Buttons, events: &mut Vec<Events>) {
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
          if button == &self.actions.put_bomb {
            if self.bomb_num > 0 {
              events.push(Events::new("bn", self.position, 0, self.firepower, self.player_id));
            }
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

    pub fn position(&mut self) -> Point{
        self.position
    }

    pub fn add_bomb_num(&mut self) {
      self.bomb_num += 1;
    }
    pub fn sub_bomb_num(&mut self) {
      self.bomb_num -= 1;
    }
}
