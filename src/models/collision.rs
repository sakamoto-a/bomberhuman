use crate::models::Player;
use crate::models::Bomb;
use crate::models::Fire;
use crate::models::Item;
use crate::models::Block;
use crate::models::Softblock;
use crate::geometory::Point;
use crate::geometory::Size;

pub struct Collision {
}

impl Collision {
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
      let offset: f64 = 10.0;
      for fire in fires {
        if player.position.x - fire.position.x < (player.size.width + fire.size.width - offset)/2.0 && fire.position.x - player.position.x < (player.size.width + fire.size.width - offset)/2.0 && player.position.y - fire.position.y < (player.size.height + fire.size.height - offset)/2.0 && fire.position.y - player.position.y < (player.size.height + fire.size.height - offset)/2.0 {
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

    pub fn dokuro_player(position: Point, players: &mut Vec<Player>, size: Size) -> usize{
      let offset: f64 = 10.0;
      for player in players {
        if position.x - player.position.x < (size.width + player.size.width-offset)/2.0 && player.position.x - position.x < (size.width + player.size.width-offset)/2.0 && position.y - player.position.y < (size.height + player.size.height-offset)/2.0 && player.position.y - position.y < (size.height + player.size.height-offset)/2.0 {
          return player.player_id;
        }
      }
      return 0;
    }

    pub fn can_move_softblock(position: Point, softblocks: &mut Vec<Softblock>, size: Size) -> bool{
      let offset: f64 = 10.0;
      for softblock in softblocks {
        if position.x - softblock.position.x < (size.width + softblock.size.width-offset)/2.0 && softblock.position.x - position.x < (size.width + softblock.size.width-offset)/2.0 && position.y - softblock.position.y < (size.height + softblock.size.height-offset)/2.0 && softblock.position.y - position.y < (size.height + softblock.size.height-offset)/2.0 {
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

    pub fn can_fire_softblock(position: Point, softblocks: &mut Vec<Softblock>, size: Size) -> bool {
      let offset: f64 = 10.0;
      for softblock in softblocks {
        if position.x - softblock.position.x < (size.width + softblock.size.width-offset)/2.0 && softblock.position.x - position.x < (size.width + softblock.size.width-offset)/2.0 && position.y - softblock.position.y < (size.height + softblock.size.height-offset)/2.0 && softblock.position.y - position.y < (size.height + softblock.size.height-offset)/2.0 {
          softblock.remove();
          return false;
        }
      }
      return true;
    }

    pub fn can_fire_item(position: Point, items: &mut Vec<Item>, size: Size) -> bool {
      let offset: f64 = 10.0;
      for item in items {
        if position.x - item.position.x < (size.width + item.size.width-offset)/2.0 && item.position.x - position.x < (size.width + item.size.width-offset)/2.0 && position.y - item.position.y < (size.height + item.size.height-offset)/2.0 && item.position.y - position.y < (size.height + item.size.height-offset)/2.0 {
          item.remove();
          return false;
        }
      }
      return true;
    }

    pub fn get_item(position: Point, items: &mut Vec<Item>, size: Size) -> usize {
      let offset: f64 = 10.0;
      for item in items {
        if position.x - item.position.x < (size.width + item.size.width-offset)/2.0 && item.position.x - position.x < (size.width + item.size.width-offset)/2.0 && position.y - item.position.y < (size.height + item.size.height-offset)/2.0 && item.position.y - position.y < (size.height + item.size.height-offset)/2.0 {
          let item_type = item.item_type;
          item.remove();
          return item_type;
        }
      }
      return 0;
    }

    pub fn can_move_bomb(position: Point, bombs: &mut Vec<Bomb>, size: Size, player_id: usize, direction: i8, can_kick: bool) -> bool{
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
          if can_kick {
            bomb.move_start(direction);
          }
          return false;
          }
        }
      }
      return true;
    }

    pub fn can_move_bomb_bomb(position: Point, bombs: &mut Vec<Bomb>, size: Size, bomb_id: usize) -> bool{
      let offset: f64 = 10.0;
      for bomb in bombs {
        if bomb.bomb_id == bomb_id {
          continue;
        }
        if position.x - bomb.position.x < (size.width + bomb.size.width-offset)/2.0 && bomb.position.x - position.x < (size.width + bomb.size.width-offset)/2.0 && position.y - bomb.position.y < (size.height + bomb.size.height-offset)/2.0 && bomb.position.y - position.y < (size.height + bomb.size.height-offset)/2.0 {
        return false;
        }
      }
      return true;
    }

    pub fn can_move_bomb_player(position: Point, players: &mut Vec<Player>, size: Size) -> bool{
      let offset: f64 = 10.0;
      for player in players {
        if position.x - player.position.x < (size.width + player.size.width-offset)/2.0 && player.position.x - position.x < (size.width + player.size.width-offset)/2.0 && position.y - player.position.y < (size.height + player.size.height-offset)/2.0 && player.position.y - position.y < (size.height + player.size.height-offset)/2.0 {
        return false;
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

    pub fn is_over_block(position: Point, block_position: Point, size:Size) -> bool {
      let offset: f64 = 10.0;
      if position.x - block_position.x < (size.width + size.width-offset)/2.0 && block_position.x - position.x < (size.width + size.width-offset)/2.0 && position.y - block_position.y < (size.height + size.height-offset)/2.0 && block_position.y - position.y < (size.height + size.height-offset)/2.0 {
        return true;
      }
      return false;
    }
}
