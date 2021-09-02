const express = require('express');
const app = express();
const http = require('http');
const server = http.createServer(app);
const io = require('socket.io')(server);
const World = require('./world.js');
const Bomb = require('./bomb.js');
const Item = require('./item.js');
const Softblock = require('./softblock.js');
const Block = require('./block.js');
const Player = require('./player.js');
const Fire = require('./fire.js');
const { GameData } = require("bomberhuman")

var game_data;
var update_ok = 0;
var world = new World;
var timestamp = null;
var prevTimestamp = null;
const max_player = 4;
var ready_player_num = 0;
var id2player_num = [];

app.use("/public", express.static('public'));

app.get('/', function (req,res) {
  res.sendFile(__dirname + '/public/index.html');
});

io.on('connection', (socket) => {
  socket.on('my_name', () => {
    //socket.join(String(0));
    id2player_num[socket.id] = ready_player_num;
    ready_player_num += 1;
    if (ready_player_num == max_player) {
      game_start();
      ready_player_num = 0;
    }
  });

  socket.on('update', (operation) => {
    update_ok += 1;
    // WIP:update key 
    move_player(id2player_num[socket.id], operation);

    if (update_ok == max_player) {
      if (game_data.is_end()) {
        console.log('end')
        let winner = game_data.get_winner();
        io.emit('end', winner);
        return;
      } else {
        var date = new Date();
        if (prevTimestamp == null) {
          timestamp = date.getTime();
          prevTimestamp = timestamp;  
        }
        world.players.splice(0);
        world.blocks.splice(0);
        world.softblocks.splice(0);
        world.items.splice(0);
        world.bombs.splice(0);
        world.fires.splice(0);
        world.time = 0.0;
        timestamp = date.getTime();
        let progress = (timestamp - prevTimestamp) / 1000;

        game_data.update(progress);

        let bomb_num = game_data.get_bomb_num();
        for(let i=0; i<bomb_num; i++) {
          var x = game_data.x(i, "bomb");
          var y = game_data.y(i, "bomb");
          var type = game_data.what_type(i, "bomb");
          world.bombs.push(new Bomb(x, y, type))
        }

        let fire_num = game_data.get_fire_num();
        for(let i=0; i<fire_num; i++) {
          var x = game_data.x(i, "fire");
          var y = game_data.y(i, "fire");
          world.fires.push(new Fire(x, y))
        }

        let item_num = game_data.get_item_num();
        for(let i=0; i<item_num; i++) {
          var x = game_data.x(i, "item");
          var y = game_data.y(i, "item");
          var type = game_data.what_type(i, "item");
          world.items.push(new Item(x, y, type))
        }

        let softblock_num = game_data.get_softblock_num();
        for(let i=0; i<softblock_num; i++) {
          var x = game_data.x(i, "softblock");
          var y = game_data.y(i, "softblock");
          world.softblocks.push(new Softblock(x, y))
        }

        let player_num = game_data.get_player_num();
        for(let i=0; i<player_num; i++) {
          var x = game_data.x(i, "player");
          var y = game_data.y(i, "player");
          var angle = game_data.angle(i);
          world.players.push(new Player(x, y, angle))
        }

        let block_num = game_data.get_block_num();
        for(let i=0; i<block_num; i++) {
          var x = game_data.x(i, "block");
          var y = game_data.y(i, "block");
          world.blocks.push(new Block(x, y))
        }

        world.time = game_data.get_time();
        io.emit('update_game', world);
      }
      console.log(world.time);
      prevTimestamp = timestamp;
      update_ok = 0;
    }
  });
});

function game_start() {
  game_data = new GameData;
  io.emit('start');
}

function move_player(player_num, operation) {
  if (game_data == null) {
    return;
  }
  switch (player_num) {
  case 0:
    if (operation.up) {
      game_data.buttons("up", 1);
    } else {
      game_data.buttons("up", 0);
    }
    if (operation.down) {
      game_data.buttons("down", 1);
    } else {
      game_data.buttons("down", 0);
    }
    if (operation.left) {
      game_data.buttons("left", 1);
    } else {
      game_data.buttons("left", 0);
    }
    if (operation.right) {
      game_data.buttons("right", 1);
    } else {
      game_data.buttons("right", 0);
    }
    if (operation.putbomb) {
      game_data.buttons("space", 1);
    } else {
      game_data.buttons("space", 0);
    }
    if (operation.neutral) {
      game_data.buttons(".", 1);
    } else {
      game_data.buttons(".", 0);
    }
    break;
  case 1:
    if (operation.up) {
      game_data.buttons("w", 1);
    } else {
      game_data.buttons("w", 0);
    }
    if (operation.down) {
      game_data.buttons("s", 1);
    } else {
      game_data.buttons("s", 0);
    }
    if (operation.left) {
      game_data.buttons("a", 1);
    } else {
      game_data.buttons("a", 0);
    }
    if (operation.right) {
      game_data.buttons("d", 1);
    } else {
      game_data.buttons("d", 0);
    }
    if (operation.putbomb) {
      game_data.buttons("x", 1);
    } else {
      game_data.buttons("x", 0);
    }
    if (operation.neutral) {
      game_data.buttons("q", 1);
    } else {
      game_data.buttons("q", 0);
    }
    break;
  case 2:
    if (operation.up) {
      game_data.buttons("t", 1);
    } else {
      game_data.buttons("t", 0);
    }
    if (operation.down) {
      game_data.buttons("g", 1);
    } else {
      game_data.buttons("g", 0);
    }
    if (operation.left) {
      game_data.buttons("f", 1);
    } else {
      game_data.buttons("f", 0);
    }
    if (operation.right) {
      game_data.buttons("h", 1);
    } else {
      game_data.buttons("h", 0);
    }
    if (operation.putbomb) {
      game_data.buttons("b", 1);
    } else {
      game_data.buttons("b", 0);
    }
    if (operation.neutral) {
      game_data.buttons("r", 1);
    } else {
      game_data.buttons("r", 0);
    }
    break;
  case 3:
    if (operation.up) {
      game_data.buttons("i", 1);
    } else {
      game_data.buttons("i", 0);
    }
    if (operation.down) {
      game_data.buttons("k", 1);
    } else {
      game_data.buttons("k", 0);
    }
    if (operation.left) {
      game_data.buttons("j", 1);
    } else {
      game_data.buttons("j", 0);
    }
    if (operation.right) {
      game_data.buttons("l", 1);
    } else {
      game_data.buttons("l", 0);
    }
    if (operation.putbomb) {
      game_data.buttons(",", 1);
    } else {
      game_data.buttons(",", 0);
    }
    if (operation.neutral) {
      game_data.buttons("u", 1);
    } else {
      game_data.buttons("u", 0);
    }
    break;
  default: 
    break;
}

}

server.listen(process.env.PORT || 3000 , function(){
  console.log('listening on *:3000');
});
