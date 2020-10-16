use crate::geometory::Point;

pub struct Events {
  pub event: &'static str,
  pub position: Point,
  pub direction: i8,
  pub firepower: i8,
  pub player_id: usize,
}

impl Events {
  pub fn new(event: &'static str, position: Point, direction: i8, firepower: i8, player_id: usize) -> Events {
        Events {
      event: event,
      position: position,
      direction: direction,
      firepower: firepower,
      player_id: player_id,
    }
  }
}
