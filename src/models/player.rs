use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Buttons;
use crate::controller::Actions;
use crate::controller::Events;
use crate::controller::Items;
use std::f64::consts::PI;

pub struct Player {
    pub position: Point,
    pub speed: f64,
    direction: f64,
    actions: Actions,
    pub bomb_num: i8,
    pub firepower: i8,
    pub kick: bool,
    pub bomb_type: i8,
    pub player_id: usize,
    pub size: Size,
    pub dead: bool,
    pub items: Items, 
    }

impl Player {
    pub fn new(point: Point, speed: f64, direction: f64, actions: Actions, player_id: usize) -> Player {
      let size = Size::new(50.0,50.0);
      let items = Items::new();
        Player {
            position: point,
            speed: speed,
            direction: direction,
            actions: actions,
            bomb_num: 1,
            firepower: 1,
            kick: false,
            bomb_type: 0,
            player_id: player_id,
            size: size,
            dead: false,
            items: items,
        }
    }

    pub fn update(&mut self, dt: &f64, buttons: &Buttons, events: &mut Vec<Events>) {
      if self.dead {
        self.position.x = -1.0;
        self.position.y = -1.0;
        return
      } else {
        for button in &buttons.button {
          if button == &self.actions.move_up {
//              *self.y() -= dt * self.speed;
              *self.angle() = PI * 3.0 / 2.0;
              let new_position = Point::new(self.position.x, self.position.y - dt * self.speed);
              events.push(Events::new("pm", new_position, 1, 0, self.player_id, 0));
            
          }
          if button == &self.actions.move_down {
            //*self.y() += dt * self.speed;
            *self.angle() = PI / 2.0;
              let new_position = Point::new(self.position.x, self.position.y + dt * self.speed);
              events.push(Events::new("pm", new_position, 2, 0, self.player_id, 0));
          }
          if button == &self.actions.move_left {
            //*self.x() -= dt * self.speed;
            *self.angle() = PI;
              let new_position = Point::new(self.position.x - dt * self.speed, self.position.y);
              events.push(Events::new("pm", new_position, 3, 0, self.player_id, 0));
          }
          if button == &self.actions.move_right {
            //*self.x() += dt * self.speed;
            *self.angle() = 0.0;
              let new_position = Point::new(self.position.x + dt * self.speed, self.position.y);
              events.push(Events::new("pm", new_position, 4, 0, self.player_id, 0));
          }
          if button == &self.actions.put_bomb {
            if self.bomb_num > 0 {
              let mut new_position = self.position;
              new_position.set_field_point();
              //events.push(Events::new("pm", new_position, 0, 0, self.player_id, 0));
              events.push(Events::new("bn", self.position, 0, self.firepower, self.player_id, self.bomb_type));
            }
          }
        }
      }
    }

    //pub fn get_field(&mut self) -> Vec<i32> {
    //    &mut self.field
    //}

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

    pub fn position_move(&mut self, new_position: Point) {
      self.position = new_position;
    }

    pub fn dead(&mut self) {
      self.dead = true;
    }
}
