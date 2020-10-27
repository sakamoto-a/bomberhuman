use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Events;
//#define BOMB_LIFE 3

pub struct Bomb {
    pub position: Point,
    firepower: i8,
    pub life: f64,
    player_id: usize,
    pub size: Size,
}

impl Bomb {
    pub fn new(mut point: Point, firepower: i8, player_id: usize) -> Bomb {
        let size = Size::new(50.0, 50.0);
        point.set_field_point();
            Bomb {
                position: point,
                firepower: firepower,
                life: 3.0,
                player_id: player_id,
                size: size,
            }
    }

    pub fn update(&mut self, dt: &f64, events: &mut Vec<Events>) {
        self.life -= dt;
        if self.life < 0.0 {
            self.remove(events);
            events.push(Events::new("fn", self.position, 0, self.firepower, self.player_id));

        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn remove(&mut self, events: &mut Vec<Events>) {
        events.push(Events::new("br", self.position, 0, 0, self.player_id));
    }

    pub fn x(&mut self) -> &mut f64{
        &mut self.position.x
    }

    pub fn y(&mut self) -> &mut f64{
        &mut self.position.y
    }

}
