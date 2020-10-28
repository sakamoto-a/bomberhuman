use crate::models::Player;
use crate::models::Bomb;
use crate::models::Fire;
use crate::models::Block;
use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Actions;
use crate::controller::Buttons;
use crate::controller::Events;


pub struct World {
    pub blocks: Vec<Block>,
    pub players: Vec<Player>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub events: Vec<Events>,
    pub field: Vec<i32>,
    pub size: Size,
    pub end: bool,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut blocks: Vec<Block> = Vec::new();
        let field: Vec<i32> = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        for i in 0..13 {
            for j in 0..15 {
                if field[i*15+j] == 1 {
                    blocks.push(Block::new(Point::new((j*50) as f64, (i*50) as f64)))
                }
            }
        }
        let mut players: Vec<Player> = Vec::new();
        let bombs: Vec<Bomb> = Vec::new();
        let fires: Vec<Fire> = Vec::new();
        let events: Vec<Events> = Vec::new();
        for n in 0..2 {
            match n {
                0 => {
                    let point = Point::new(50.0, 50.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("up", "down", "left", "right", "l"), 0));
                },
                1 => {
                    let point = Point::new(550.0, 350.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("w", "s", "a", "d", "x"), 1));
                },
                _ => (),
            }
        }
        World {
            players: players,
            bombs: bombs,
            fires: fires,
            blocks: blocks,
            events: events,
            field: field,
            size: size,
            end: false,
        }
    }

    pub fn update(&mut self, dt: f64, buttons: &Buttons) {
        for player in &mut self.players {
            player.update(&dt, buttons, &mut self.events);
        }
        for bomb in &mut self.bombs {
            bomb.update(&dt, &mut self.events);
        }
        for fire in &mut self.fires {
            fire.update(&dt,&mut self.events);
        }
        for player in &mut self.players {
          if World::is_dead(player, &mut self.fires) {
            player.dead(&mut self.events);
          }
        }
        for event in &mut self.events {
            match event.event {
              "pm" => {
                if World::can_move_block(event.position, &mut self.blocks, self.players[event.player_id].size) {
                  if World::can_move_bomb(event.position, &mut self.bombs, self.players[event.player_id].size, event.player_id) {
                    self.players[event.player_id].position_move(event.position);
                  }
                }
                for bomb in &mut self.bombs {
                  if bomb.over_players.len() != 0 {
                    for player_id in 0..self.players.len() {
                      if World::is_over_bomb(self.players[player_id].position, bomb.position, bomb.size) == false {
                        bomb.remove_over_player(player_id);
//                        bomb.over_players.retain(|x| *x != *player_id);
                      }
                    }
                  }
                }

              },
              "bn" => {
                if World::can_put_bomb(&mut self.bombs, event.position) {
                  let mut over_players: Vec<usize> = Vec::new();
                  for player in &self.players {
                    if player.player_id == event.player_id {
                      over_players.push(event.player_id);
                    } else {
                      if World::is_over_bomb(player.position, event.position, Size::new(50.0,50.0)) {
                        over_players.push(player.player_id);
                      }
                    }
                  }
                  self.bombs.push(Bomb::new(event.position, event.firepower, event.player_id, over_players));
                  self.players[event.player_id].sub_bomb_num();
                }
              },
              "br" => {
                self.bombs.retain(|x| x.life > 0.0);
              },
              "fn" => {
                if World::can_fire(event.position, &mut self.blocks, Size::new(50.0,50.0)) {
                  self.fires.push(Fire::new(event.position, event.direction, event.firepower, event.player_id));
                }
              },
              "fr" => {
                self.fires.remove(0);
                if event.direction == 0 {
                  self.players[event.player_id].add_bomb_num();
                }
              },
              _ => (),
            }
        }
        for bomb in &mut self.bombs {
          if World::is_explosion(bomb, &mut self.fires) {
            bomb.life = 0.0;
          }
        }

        let mut count: usize = 0;
        for player in &mut self.players {
          if player.dead == false {
            count += 1;
          }
        }
        if count <= 1 {
          self.end = true;
        }

        self.events.clear();
    }

    pub fn get_player_num(&mut self) -> usize {
        self.players.len()
    }

    pub fn get_bomb_num(&mut self) -> usize {
        self.bombs.len()
    }

    pub fn get_fire_num(&mut self) -> usize {
        self.fires.len()
    }

    pub fn get_block_num(&mut self) -> usize {
        self.blocks.len()
    }

    pub fn push_bomb(&mut self, bomb: Bomb) {
        self.bombs.push(bomb);
    }

    pub fn push_fire(&mut self, fire: Fire) {
        self.fires.push(fire);
    }
//colision
    pub fn can_put_bomb(bombs: &mut Vec<Bomb>,position: Point) -> bool{
      for bomb in bombs {
        if bomb.position.x - position.x < bomb.size.width && position.x - bomb.position.x < bomb.size.width && bomb.position.y - position.y < bomb.size.height && position.y - bomb.position.y < bomb.size.height {
          return false;
        }
      }
      return true;
    }

    pub fn is_explosion(bomb: &mut Bomb, fires: &mut Vec<Fire>) -> bool{
      for fire in fires {
        if bomb.position.x - fire.position.x < (bomb.size.width + fire.size.width)/2.0 && fire.position.x - bomb.position.x < (bomb.size.width + fire.size.width)/2.0 && bomb.position.y - fire.position.y < (bomb.size.height + fire.size.height)/2.0 && fire.position.y - bomb.position.y < (bomb.size.height + fire.size.height)/2.0 {
          return true;
        }
      }
      return false;
    }

    pub fn is_dead(player: &mut Player, fires: &mut Vec<Fire>) -> bool{
      for fire in fires {
        if player.position.x - fire.position.x < (player.size.width + fire.size.width)/2.0 && fire.position.x - player.position.x < (player.size.width + fire.size.width)/2.0 && player.position.y - fire.position.y < (player.size.height + fire.size.height)/2.0 && fire.position.y - player.position.y < (player.size.height + fire.size.height)/2.0 {
          return true;
        }
      }
      return false;
    }

    pub fn can_move_block(position: Point, blocks: &mut Vec<Block>, size: Size) -> bool{
      let offset: f64 = 10.0;
      for block in blocks {
        if position.x - block.position.x < (size.width + block.size.width-offset)/2.0 && block.position.x - position.x < (size.width + block.size.width-offset)/2.0 && position.y - block.position.y < (size.height + block.size.height-offset)/2.0 && block.position.y - position.y < (size.height + block.size.height-offset)/2.0 {
          return false;
        }
      }
      return true;
    }

    pub fn can_fire(position: Point, blocks: &mut Vec<Block>, size: Size) -> bool{
      let offset: f64 = 10.0;
      for block in blocks {
        if position.x - block.position.x < (size.width + block.size.width-offset)/2.0 && block.position.x - position.x < (size.width + block.size.width-offset)/2.0 && position.y - block.position.y < (size.height + block.size.height-offset)/2.0 && block.position.y - position.y < (size.height + block.size.height-offset)/2.0 {
          return false;
        }
      }
      return true;
    }

    pub fn can_move_bomb(position: Point, bombs: &mut Vec<Bomb>, size: Size, player_id: usize) -> bool{
      let offset: f64 = 10.0;
      let mut flag: bool;
      for bomb in bombs {
        flag = true;
        for over_player in &bomb.over_players {
          if *over_player == player_id {
            flag = false;
          }
        }
        if flag {
          if position.x - bomb.position.x < (size.width + bomb.size.width-offset)/2.0 && bomb.position.x - position.x < (size.width + bomb.size.width-offset)/2.0 && position.y - bomb.position.y < (size.height + bomb.size.height-offset)/2.0 && bomb.position.y - position.y < (size.height + bomb.size.height-offset)/2.0 {
          return false;
          }
        }
      }
      return true;
    }

    pub fn is_over_bomb(position: Point, bomb_position: Point, size:Size) -> bool {
      let offset: f64 = 10.0;
      if position.x - bomb_position.x < (size.width + size.width-offset)/2.0 && bomb_position.x - position.x < (size.width + size.width-offset)/2.0 && position.y - bomb_position.y < (size.height + size.height-offset)/2.0 && bomb_position.y - position.y < (size.height + size.height-offset)/2.0 {
        return true;
      }
      return false;
    }
}
