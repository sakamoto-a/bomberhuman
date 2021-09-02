use crate::models::Player;
use crate::models::Bomb;
use crate::models::Fire;
use crate::models::Item;
use crate::models::Block;
use crate::models::Softblock;
use crate::models::Collision;
use crate::geometory::Point;
use crate::geometory::Size;
use crate::controller::Actions;
use crate::controller::Buttons;
use crate::controller::Events;
use rand::seq::SliceRandom;
use rand::Rng;
//use wasm_bindgen::prelude::*;

pub struct World {
    pub blocks: Vec<Block>,
    pub softblocks: Vec<Softblock>,
    pub players: Vec<Player>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub items: Vec<Item>,
    pub events: Vec<Events>,
    pub field: Vec<i32>,
    pub size: Size,
    pub end: bool,
    pub life: f64,
    pub next_bomb_time: f64,
    pub hurry_up: Vec<i32>,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut blocks: Vec<Block> = Vec::new();
        let mut softblocks: Vec<Softblock> = Vec::new();
        let mut items: Vec<Item> = Vec::new();
        let mut field: Vec<i32> = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 2, 1,
            1, -1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, -1, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, -1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, -1, 1,
            1, 2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 2, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let mut item_set = vec![0, 0, 0, 0, 0, 0, 0,
                              0, 0, 0, 0, 0, 0,
                              3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                              3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                              3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                              3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                              7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                              11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
                              15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                              19, 19, 19, 23, 23, 23, 27, 27, 27, 31, 31,
                              ];
        let mut n:usize = 0;
        let mut rng = rand::thread_rng();
        item_set.shuffle(&mut rng);
        for i in 0..field.len() {
          if field[i] == 0 {
             field[i] = item_set[n];
            n += 1;
          }
        }
        for i in 0..13 {
            for j in 0..15 {
                if field[i*15+j] == 1 {
                    blocks.push(Block::new(Point::new((j*50) as f64, (i*50) as f64)))
                }
                if field[i*15+j]%4 == 3 {
                    softblocks.push(Softblock::new(Point::new((j*50) as f64, (i*50) as f64)));
                    if field[i*15+j]/4 > 0 {
                     items.push(Item::new(Point::new((j*50) as f64, (i*50) as f64), (field[i*15+j]/4) as usize))
                    }
                }
            }
        }
        let mut players: Vec<Player> = Vec::new();
        let bombs: Vec<Bomb> = Vec::new();
        let fires: Vec<Fire> = Vec::new();
        let events: Vec<Events> = Vec::new();
        for n in 0..4 {
            match n {
                0 => {
                    let point = Point::new(50.0, 50.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("up", "down", "left", "right", "space", "."), 0));
                },
                1 => {
                    let point = Point::new(650.0, 550.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("w", "s", "a", "d", "x", "q"), 1));
                },
                2 => {
                    let point = Point::new(50.0, 550.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("t", "g", "f", "h", "b", "r"), 2));
                },
                3 => {
                    let point = Point::new(650.0, 50.0);
                    players.push(Player::new(point, 150.0, 0.0, Actions::new("i", "k", "j", "l", ",", "u"), 3));
                },
                _ => (),
            }
        }
        let hurry_up = vec![
             97,  95, 110, 111, 112, 113, 114,  99,  84,  83,  82,
             81,  80,  79, 109, 125, 127, 129, 115,  85,  69,  67,
             65,  63,  78,  93, 108, 123, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 131, 116, 101,  86,  71,  56,  55,
             54,  53,  52,  51,  50,  49,  48,  47,  77, 107, 137,
            153, 155, 157, 159, 161, 147, 117,  87,  57,  41,  39,
             37,  35,  33,  31,  46,  61,  76,  91, 106, 121, 136,
            151, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
            176, 177, 178, 163, 148, 133, 118, 103,  88,  73,  58,
             43,  28,  27,  26,  25,  24,  23,  22,  21,  20,  19,
             18,  17,  16,
             ];
        World {
            players: players,
            bombs: bombs,
            fires: fires,
            items: items,
            blocks: blocks,
            softblocks: softblocks,
            events: events,
            field: field,
            size: size,
            end: false,
            life: 180.0,
            next_bomb_time: 0.0,
            hurry_up: hurry_up,
        }
    }

    pub fn update(&mut self, dt: f64, buttons: &Buttons) {
      self.life -= dt;
      if self.life < 30.0 {
        self.next_bomb_time -= dt;
  /*      if self.next_bomb_time < 0.0 {
          let mut field = World::mapping(&mut self.players, &mut self.fires, &mut self.bombs, &mut self.items, &mut self.softblocks);
          let mut n = 0;
          for i in 0..field.len() {
            if field[i] == 0 {
              n += 1;
            }
          }
          let bomb_id;
          if self.bombs.len() == 0 {
            bomb_id = 0;
          } else {
            bomb_id = self.bombs[self.bombs.len()-1].bomb_id + 1;
          }
          if n > 0 {
            let mut bomb_set = vec![0; n];
            bomb_set[0] = -10;
            let mut rng = rand::thread_rng();
            bomb_set.shuffle(&mut rng);
            n = 0; 
            for i in 0..field.len() {
              if field[i] == 0 {
                 field[i] = bomb_set[n];
                n += 1;
              }
            }
            for i in 0..13 {
                for j in 0..15 {
                  if field[i*15+j] == -10 {
                      self.bombs.push(Bomb::new(bomb_id, Point::new((j*50) as f64, (i*50) as f64),4, 20, Vec::new(),1));
                    }
                  }
               }
              }
              if self.life > 15.0 {
                self.next_bomb_time = 0.5;
              } else {
                self.next_bomb_time = 0.25;
              }
        }
*/        if self.next_bomb_time < 0.0 {
          match self.hurry_up.pop() {
            Some(next) => {
              let next_x = next % 15;
              let next_y = next / 15;
              let block_position = Point::new((next_x*50) as f64, (next_y*50) as f64);
              for player in &mut self.players {
                if Collision::is_over_block(player.position, block_position, player.size) {
                  player.dead();
                }
              }
              self.blocks.push(Block::new(block_position));
            },
            None => (),
          }
          self.next_bomb_time = 30.0/113.0;
        }

      }
        
        for item in &mut self.items {
            item.update(&mut self.events);
        }
        for player in &mut self.players {
            player.update(&dt, buttons, &mut self.events);
        }
        for bomb in &mut self.bombs {
            bomb.update(&dt, &mut self.events);
        }
        for fire in &mut self.fires {
            fire.update(&dt,&mut self.events);
        }
        for softblock in &mut self.softblocks {
            softblock.update(&mut self.events);
        }
        for player in &mut self.players {
          if Collision::is_dead(player, &mut self.fires) {
            player.dead();
            self.events.push(Events::new("pd", player.position, 0, 0, player.player_id, 0));
          }
        }

        for event in &mut self.events {
            match event.event {
              "pm" => {
                if Collision::can_move_softblock(event.position, &mut self.softblocks, self.players[event.player_id].size) {
                  if Collision::can_move_block(event.position, &mut self.blocks, self.players[event.player_id].size) {
                    if Collision::can_move_bomb(event.position, &mut self.bombs, self.players[event.player_id].size, event.player_id, event.direction, self.players[event.player_id].kick) {
                      self.players[event.player_id].position_move(event.position);
                      let item_type = Collision::get_item(event.position, &mut self.items, self.players[event.player_id].size);
                      match item_type {
                        1 => {
                         //sound("get_item");
                         if self.players[event.player_id].items.fire_up < 8 {
                           self.players[event.player_id].firepower += 1;
                         }
                         self.players[event.player_id].items.fire_up += 1;
                        },
                        2 => {
                         //sound("get_item");
                         if self.players[event.player_id].items.bomb_up < 8 {
                           self.players[event.player_id].bomb_num += 1;
                         }
                         self.players[event.player_id].items.bomb_up += 1;
                        },
                        3 => {
                         //sound("get_item");
                         if self.players[event.player_id].items.speed_up < 8{
                           self.players[event.player_id].speed += 25.0;
                         }
                         self.players[event.player_id].items.speed_up += 1;
                        },
                        4 => {
                         //sound("get_item");
                         self.players[event.player_id].kick = true;
                         self.players[event.player_id].items.kick += 1;
                        },
                        5 => {
                         //sound("get_item");
                         self.players[event.player_id].bomb_type = 1;
                         self.players[event.player_id].items.uni_bomb += 1;
                        },
                        6 => {
                         //sound("get_item");
                         self.players[event.player_id].bomb_type = 2;
                         self.players[event.player_id].items.gomu_bomb += 1;
                        },
                        7 => {
                         //sound("get_dokuro");
                          let mut rng = rand::thread_rng();
                          let i: u8 = rng.gen();
                          self.players[event.player_id].status = (i % 3 + 1) as i8;
                          self.players[event.player_id].status_life = 10.0;
                        },
                        _ => (),
                      }
                      
                    } 
                  }
                  for bomb in &mut self.bombs {
                    if bomb.over_players.len() != 0 {
                      for player_id in 0..self.players.len() {
                        if Collision::is_over_bomb(self.players[player_id].position, bomb.position, bomb.size) == false {
                          bomb.remove_over_player(player_id);
  //                        bomb.over_players.retain(|x| *x != *player_id);
                        }
                      }
                    }
                  }
                }
              },
              "pd" => {
                let mut field = World::mapping(&mut self.players, &mut self.fires, &mut self.bombs, &mut self.items, &mut self.softblocks);
                let mut n = 0;
                for i in 0..field.len() {
                  if field[i] == 0 {
                    n += 1;
                  }
                }
                let mut item_set = vec![0; n];
                let mut fire_item_num = self.players[event.player_id].items.fire_up;
                let mut bomb_item_num = self.players[event.player_id].items.bomb_up;
                let mut speed_item_num = self.players[event.player_id].items.speed_up;
                let mut kick_item_num = self.players[event.player_id].items.kick;
                let mut uni_bomb_item_num = self.players[event.player_id].items.uni_bomb;
                let mut gomu_bomb_item_num = self.players[event.player_id].items.gomu_bomb;
                for i in 0..item_set.len() {
                  if fire_item_num > 0 {
                    item_set[i] = 4;
                    fire_item_num -= 1;
                  } else if bomb_item_num > 0 {
                    item_set[i] = 8;
                    bomb_item_num -= 1;
                  } else if speed_item_num > 0 {
                    item_set[i] = 12;
                    speed_item_num -= 1;
                  } else if kick_item_num > 0 {
                    item_set[i] = 16;
                    kick_item_num -= 1;
                  } else if uni_bomb_item_num > 0 {
                    item_set[i] = 20;
                    uni_bomb_item_num -= 1;
                  } else if gomu_bomb_item_num > 0 {
                    item_set[i] = 24;
                    gomu_bomb_item_num -= 1;
                  }
                }
                n = 0;
                let mut rng = rand::thread_rng();
                item_set.shuffle(&mut rng);
                for i in 0..field.len() {
                  if field[i] == 0 {
                     field[i] = item_set[n];
                    n += 1;
                  }
                }
                for i in 0..13 {
                    for j in 0..15 {
                      if field[i*15+j] > 0 {
                        if field[i*15+j]%4 == 0 {
                            self.items.push(Item::new(Point::new((j*50) as f64, (i*50) as f64),(field[i*15+j]/4) as usize))
                        }
                      }
                    }
                }
              },
              "bn" => {
                if Collision::can_put_bomb(&mut self.bombs, event.position) {
                  let mut over_players: Vec<usize> = Vec::new();
                  for player in &self.players {
                    if player.player_id == event.player_id {
                      over_players.push(event.player_id);
                    } else {
                      if Collision::is_over_bomb(player.position, event.position, Size::new(50.0,50.0)) {
                        over_players.push(player.player_id);
                      }
                    }
                  }
                  let bomb_id;
                  if self.bombs.len() == 0 {
                    bomb_id = 0;
                  } else {
                    bomb_id = self.bombs[self.bombs.len()-1].bomb_id + 1;
                  }
                  //sound("put_bomb");
                  self.bombs.push(Bomb::new(bomb_id, event.position, event.firepower, event.player_id, over_players, event.bomb_type));
                  self.players[event.player_id].sub_bomb_num();
                }
              },
              "bm" => {
                if Collision::can_move_softblock(event.position, &mut self.softblocks, Size::new(50.0,50.0)) {
                  if Collision::can_move_block(event.position, &mut self.blocks,Size::new(50.0,50.0)) {
                    if Collision::can_move_bomb_bomb(event.position, &mut self.bombs,Size::new(50.0,50.0), event.player_id) {
                      if Collision::can_move_bomb_player(event.position, &mut self.players, Size::new(50.0,50.0)) {
                      let item_type = Collision::get_item(event.position, &mut self.items,Size::new(50.0,50.0) );
                      if item_type > 0 {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            if bomb.bomb_type == 2 {
                              bomb.move_reverse();
                            } else {
                              bomb.move_stop();
                            }
                            break;
                          }
                        }
                      } else {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            bomb.position_move(event.position);
                            break;
                          }
                        }
                      }
                      } else {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            if bomb.bomb_type == 2 {
                              bomb.move_reverse();
                            } else {
                              bomb.move_stop();
                            }
                            break;
                          }
                        }
                }
                    } else {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            if bomb.bomb_type == 2 {
                              bomb.move_reverse();
                            } else {
                              bomb.move_stop();
                            }
                            break;
                          }
                        }
                }
 
                  } else {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            if bomb.bomb_type == 2 {
                              bomb.move_reverse();
                            } else {
                              bomb.move_stop();
                            }
                            break;
                          }
                        }
                }

                } else {
                        for bomb in &mut self.bombs {
                          if bomb.bomb_id == event.player_id {
                            if bomb.bomb_type == 2 {
                              bomb.move_reverse();
                            } else {
                              bomb.move_stop();
                            }
                            break;
                          }
                        }
                }
              },
              "br" => {
                self.bombs.retain(|x| x.life > 0.0);
                //sound("explosion");
              },
              "fn" => {
                if Collision::can_fire(event.position, &mut self.blocks, Size::new(50.0,50.0)) {
                  if event.bomb_type == 1 {
                    if Collision::can_fire_softblock(event.position, &mut self.softblocks, Size::new(50.0,50.0)) {
                    Collision::can_fire_item(event.position, &mut self.items, Size::new(50.0,50.0));
                    self.fires.push(Fire::new(event.position, event.direction, event.firepower, event.player_id, event.bomb_type));
                    } else {
                      self.fires.push(Fire::new(event.position, event.direction, event.firepower, event.player_id, event.bomb_type));
                    }
                    } else {
                      if Collision::can_fire_softblock(event.position, &mut self.softblocks, Size::new(50.0,50.0)) {
                        if Collision::can_fire_item(event.position, &mut self.items, Size::new(50.0,50.0)) {
                          self.fires.push(Fire::new(event.position, event.direction, event.firepower, event.player_id, event.bomb_type));
                      }
                    }
                  }
                }
              },
              "fr" => {
                self.fires.remove(0);
                if event.direction == 0 {
                  if event.player_id < 10 {
                    self.players[event.player_id].add_bomb_num();
                  }
                }
              },
              "sbr" => {
                self.softblocks.retain(|x| x.dead == false);
              },
              "ir" => {
                self.items.retain(|x| x.dead == false);
              },
              _ => (),
            }
        }
        for bomb in &mut self.bombs {
          if Collision::is_explosion(bomb, &mut self.fires) {
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

        if self.life <= 0.0 {
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

    pub fn get_item_num(&mut self) -> usize {
        self.items.len()
    }

    pub fn get_block_num(&mut self) -> usize {
        self.blocks.len()
    }

    pub fn get_softblock_num(&mut self) -> usize {
        self.softblocks.len()
    }

    pub fn get_winner(&mut self) -> usize {
      let mut count = 0;
      let mut winner = 0;
        for player in &mut self.players {
          if player.dead == false {
            count += 1;
          }
        }
        if count > 1 {
          winner = 0;
        } else {
          for player in &mut self.players {
            if player.dead == false {
              winner = player.player_id + 1;
            }
          }
        }
        winner  
    }

    pub fn push_bomb(&mut self, bomb: Bomb) {
        self.bombs.push(bomb);
    }

    pub fn push_fire(&mut self, fire: Fire) {
        self.fires.push(fire);
    }

    pub fn mapping(players: &mut Vec<Player>, fires: &mut Vec<Fire>, bombs: &mut Vec<Bomb>, items: &mut Vec<Item>, softblocks: &mut Vec<Softblock>) -> Vec<i32> {
      let mut field: Vec<i32> = vec![
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
      for player in players {
        if !player.dead {
          let mut position: Point = player.position;
          position.set_field_point();
          field[(position.x/50.0+position.y/50.0*15.0) as usize] = 2;
          field[(position.x/50.0+position.y/50.0*15.0) as usize + 1] = -1;
          field[(position.x/50.0+position.y/50.0*15.0) as usize - 1] = -1;
          field[(position.x/50.0+position.y/50.0*15.0) as usize + 15] = -1;
          field[(position.x/50.0+position.y/50.0*15.0) as usize - 15] = -1;
        }
      }
      for bomb in bombs {
        let mut position: Point = bomb.position;
        position.set_field_point();
        if bomb.speed == 0.0 {
          field[(position.x/50.0+position.y/50.0*15.0) as usize] = 5;
        } else {
          match bomb.direction {
            1 => {
              field[(position.x/50.0+position.y/50.0*15.0) as usize + 15] = -5;
              field[(position.x/50.0+position.y/50.0*15.0) as usize - 15] = -5;
            },
            2 => {
              field[(position.x/50.0+position.y/50.0*15.0) as usize + 15] = -5;
              field[(position.x/50.0+position.y/50.0*15.0) as usize - 15] = -5;
            },
            3 => {
              field[(position.x/50.0+position.y/50.0*15.0) as usize + 1] = -5;
              field[(position.x/50.0+position.y/50.0*15.0) as usize - 1] = -5;
            },
            4 => {
              field[(position.x/50.0+position.y/50.0*15.0) as usize + 1] = -5;
              field[(position.x/50.0+position.y/50.0*15.0) as usize - 1] = -5;
            },
            _ => ()
          }
        }
      }
      for fire in fires {
        let position: Point = fire.position;
        field[(position.x/50.0+position.y/50.0*15.0) as usize] = 6;
      }
      for item in items {
        let position: Point = item.position;
        field[(position.x/50.0+position.y/50.0*15.0) as usize] = (item.item_type*4+10) as i32;
      }
      for softblock in softblocks {
        let position: Point = softblock.position;
        if field[(position.x/50.0+position.y/50.0*15.0) as usize] < 0 {
          field[(position.x/50.0+position.y/50.0*15.0) as usize] = 3;
        } else {
          field[(position.x/50.0+position.y/50.0*15.0) as usize] += 3;
        }
      }
      field
  }
}

//#[wasm_bindgen]
//pub fn sound (sound_type: &str) {
//  play_audio(sound_type);
//}
