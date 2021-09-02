const socket = io();

const canvas = document.getElementById("bomberhuman-canvas");
canvas.width = 1000; //game_data.width();
canvas.height = 1000; //game_data.height();

let ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
ctx.fillRect(0, 0, canvas.width, canvas.height);

class button {
  constructor(x, y, kind) {
    this.x = x;
    this.y = y;
    this.w = 100;
    this.h = 40;
    this.kind = kind;
  }

  draw() {
    ctx.drawImage(res.start, this.x, this.y, res.start.width, res.start.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  }

  testHit(point) {
    return (this.x <= point.x && point.x <= this.x + this.w) &&
           (this.y <= point.y && point.y <= this.y + this.h);
  }
}

class operation {
  constructor() {
    this.up = false;
    this.down = false;
    this.right = false;
    this.left = false;
    this.putbomb = false;
    this.neutral = false;
  }
}

const resources = () => {
  let res = {
    player0: document.createElement('img'),
    player1: document.createElement('img'),
    player2: document.createElement('img'),
    player3: document.createElement('img'),
    bomb: document.createElement('img'),
    uni_bomb: document.createElement('img'),
    gomu_bomb: document.createElement('img'),
    fire: document.createElement('img'),
    bomb_item: document.createElement('img'),
    fire_item: document.createElement('img'),
    speed_item: document.createElement('img'),
    kick_item: document.createElement('img'),
    dokuro_item: document.createElement('img'),
    bomb_type_item: document.createElement('img'),
    block: document.createElement('img'),
    softblock: document.createElement('img'),
    item_frame: document.createElement('img'),
    zero: document.createElement('img'),
    one: document.createElement('img'),
    two: document.createElement('img'),
    three: document.createElement('img'),
    four: document.createElement('img'),
    five: document.createElement('img'),
    six: document.createElement('img'),
    seven: document.createElement('img'),
    eight: document.createElement('img'),
    nine: document.createElement('img'),
    start: document.createElement('img'),
  }

  res.player0.width = 50;
  res.player0.height = 50;
  res.player0.src = "./public/image/player1.png";
  res.player1.width = 50;
  res.player1.height = 50;
  res.player1.src = "./public/image/player2.png";
  res.player2.width = 50;
  res.player2.height = 50;
  res.player2.src = "./public/image/player3.png";
  res.player3.width = 50;
  res.player3.height = 50;
  res.player3.src = "./public/image/player4.png";
  res.bomb.width = 50;
  res.bomb.height = 50;
  res.bomb.src = "./public/image/bomb.png";
  res.uni_bomb.width = 50;
  res.uni_bomb.height = 50;
  res.uni_bomb.src = "./public/image/uni_bomb.png";
  res.gomu_bomb.width = 50;
  res.gomu_bomb.height = 50;
  res.gomu_bomb.src = "./public/image/gomu_bomb.png";
  res.fire.width = 50;
  res.fire.height = 50;
  res.fire.src = "./public/image/fire.png";
  res.bomb_item.width = 50;
  res.bomb_item.height = 50;
  res.bomb_item.src = "./public/image/bomb_item.png";
  res.fire_item.width = 50;
  res.fire_item.height = 50;
  res.fire_item.src = "./public/image/fire_item.png";
  res.speed_item.width = 50;
  res.speed_item.height = 50;
  res.speed_item.src = "./public/image/speed_item.png";
  res.kick_item.width = 50;
  res.kick_item.height = 50;
  res.kick_item.src = "./public/image/kick_item.png";
  res.dokuro_item.width = 50;
  res.dokuro_item.height = 50;
  res.dokuro_item.src = "./public/image/dokuro_item.png";
  res.bomb_type_item.width = 50;
  res.bomb_type_item.height = 50;
  res.bomb_type_item.src = "./public/image/bomb_type_item.png";
  res.block.width = 50;
  res.block.height = 50;
  res.block.src = "./public/image/block.jpg";
  res.softblock.width = 50;
  res.softblock.height = 50;
  res.softblock.src = "./public/image/soft_block.png";
  res.item_frame.width = 50;
  res.item_frame.height = 50;
  res.item_frame.src = "./public/image/item_frame.jpg";
  
  res.one.width = 40;
  res.one.height = 50;
  res.one.src = "./public/image/p1.gif";
  res.two.width = 40;
  res.two.height = 50;
  res.two.src = "./public/image/p2.gif";
  res.three.width = 40;
  res.three.height = 50;
  res.three.src = "./public/image/p3.gif";
  res.four.width = 40;
  res.four.height = 50;
  res.four.src = "./public/image/p4.gif";
  res.five.width = 40;
  res.five.height = 50;
  res.five.src = "./public/image/p5.gif";
  res.six.width = 40;
  res.six.height = 50;
  res.six.src = "./public/image/p6.gif";
  res.seven.width = 40;
  res.seven.height = 50;
  res.seven.src = "./public/image/p7.gif";
  res.eight.width = 40;
  res.eight.height = 50;
  res.eight.src = "./public/image/p8.gif";
  res.nine.width = 40;
  res.nine.height = 50;
  res.nine.src = "./public/image/p9.gif";
  res.zero.width = 40;
  res.zero.height = 50;
  res.zero.src = "./public/image/p0.gif";
  res.start.width = 100;
  res.start.height = 40;
  res.start.src = "./public/image/start.png";

  return res;
}

var myaudio = new Audio('./public/audio/bgm.mp3');

function play_bgm() {
  myaudio.autoplay = true;
  myaudio.play();
}

function stop_bgm() {
  myaudio.pause();
}

let clear_screen = () => {
  ctx.fillStyle = "green";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}


let result_menu_screen = (winner) => {
  ctx.lineWidth = 2;
  ctx.fillStyle = "black";
  ctx.font = "100px cursive";
  ctx.textBaseline = 'center';
	ctx.textAlign = 'center';
  let x = canvas.width / 2
  let y = canvas.height / 4
  ctx.fillText("END", x, y);
  if (winner == 0) {
    ctx.fillText("Draw", x, y+100);
  } else {
    ctx.fillText(String(winner)+"Pwin", x, y+100);
  }
}

function draw_player(x, y, angle, player_id) {
  ctx.translate(x, y);
  switch(player_id) {
    case 0:
      ctx.translate(res.player0.width/2,res.player0.height/2);
      ctx.rotate(angle);
      ctx.translate(-res.player0.width/2,-res.player0.height/2);
      ctx.drawImage(res.player0, 0, 0, res.player0.width, res.player0.height);
      break;
    case 1:
      ctx.translate(res.player1.width/2,res.player1.height/2);
      ctx.rotate(angle);
      ctx.translate(-res.player1.width/2,-res.player1.height/2);
      ctx.drawImage(res.player1, 0, 0, res.player1.width, res.player1.height);
      break;
    case 2:
      ctx.translate(res.player2.width/2,res.player2.height/2);
      ctx.rotate(angle);
      ctx.translate(-res.player2.width/2,-res.player1.height/2);
      ctx.drawImage(res.player2, 0, 0, res.player2.width, res.player2.height);
      break;
    case 3:
      ctx.translate(res.player3.width/2,res.player3.height/2);
      ctx.rotate(angle);
      ctx.translate(-res.player3.width/2,-res.player3.height/2);
      ctx.drawImage(res.player3, 0, 0, res.player1.width, res.player3.height);
      break;
    default:
      break;
  }
  ctx.setTransform(1, 0, 0, 1, 0, 0);

  ctx.fillStyle = "black";
}

function draw_bomb(x, y, type) {
  if (type == 0) {
    ctx.drawImage(res.bomb, x, y, res.bomb.width, res.bomb.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  } else if (type == 1) {
    ctx.drawImage(res.uni_bomb, x, y, res.uni_bomb.width, res.uni_bomb.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  } else if (type == 2) {
    ctx.drawImage(res.gomu_bomb, x, y, res.gomu_bomb.width, res.gomu_bomb.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  }
}

function draw_fire(x, y) {
  ctx.drawImage(res.fire, x, y, res.fire.width, res.fire.height);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = "black";
}

function draw_item(x, y, type) {
    ctx.drawImage(res.item_frame, x, y, res.item_frame.width, res.item_frame.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
    switch (type) {
      case 1:
        ctx.drawImage(res.fire_item, x, y, res.fire_item.width, res.fire_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 2:
        ctx.drawImage(res.bomb_item, x, y, res.bomb_item.width, res.bomb_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 3:
        ctx.drawImage(res.speed_item, x, y, res.speed_item.width, res.speed_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 4:
        ctx.drawImage(res.kick_item, x, y, res.kick_item.width, res.kick_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 5:
        ctx.drawImage(res.bomb_type_item, x, y, res.bomb_type_item.width, res.bomb_type_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 6:
        ctx.drawImage(res.gomu_bomb, x, y, res.gomu_bomb.width, res.gomu_bomb.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
      case 7:
        ctx.drawImage(res.dokuro_item, x, y, res.dokuro_item.width, res.dokuro_item.height);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
        break;
     default :
        break;
  }
}

function draw_block(x, y) {
  ctx.drawImage(res.block, x, y, res.block.width, res.block.height);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = "black";
}

function draw_softblock(x, y) {
  ctx.drawImage(res.softblock, x, y, res.softblock.width, res.softblock.height);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = "black";
}

function draw_number(x, y, number) {
  switch (number) {
    case 0:
      ctx.drawImage(res.zero, x, y, res.zero.width, res.zero.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 1:
      ctx.drawImage(res.one, x, y, res.one.width, res.one.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 2:
      ctx.drawImage(res.two, x, y, res.two.width, res.two.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 3:
      ctx.drawImage(res.three, x, y, res.three.width, res.three.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 4:
      ctx.drawImage(res.four, x, y, res.four.width, res.four.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 5:
      ctx.drawImage(res.five, x, y, res.five.width, res.five.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 6:
      ctx.drawImage(res.six, x, y, res.six.width, res.six.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 7:
      ctx.drawImage(res.seven, x, y, res.seven.width, res.seven.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 8:
      ctx.drawImage(res.eight, x, y, res.eight.width, res.eight.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    case 9:
      ctx.drawImage(res.nine, x, y, res.nine.width, res.nine.height);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.fillStyle = "black";
      break;
    default:
      break;
   }
}

function draw_hurry_up() {
  ctx.lineWidth = 2;
  ctx.fillStyle = "red";
  ctx.font = "100px cursive";
  ctx.textBaseline = 'center';
	ctx.textAlign = 'center';
  let x = 700 / 2
  let y = 600 / 2
  ctx.fillText("HURRY UP", x, y);
}

function game_pad_update() {
  if(!(window.Gamepad)) return;
 	if(!(navigator.getGamepads)) return;
  var gamepad_list = navigator.getGamepads();
  var num = gamepad_list.length;
  for(let i=0;i < num;i++){
    let gamepad = gamepad_list[i];
    if (!gamepad) continue;
//    if (gamepad.mapping == "standard") {
      switch (i) {
        case 0:
          if ((gamepad.axes[1] < -0.5)) {
            game_data.buttons("up", 1);
          } else {
            game_data.buttons("up", 0);
          }
          if ((gamepad.axes[1] > 0.5)) {
            game_data.buttons("down", 1);
          } else {
            game_data.buttons("down", 0);
          }
          if ((gamepad.axes[0] < -0.5)) {
            game_data.buttons("left", 1);
          } else {
            game_data.buttons("left", 0);
          }
          if ((gamepad.axes[0] > 0.5)) {
            game_data.buttons("right", 1);
          } else {
            game_data.buttons("right", 0);
          }
          if (gamepad.buttons[0].pressed) {
            game_data.buttons("space", 1);
          } else {
            game_data.buttons("space", 0);
          }
          if (gamepad.buttons[5].pressed) {
            game_data.buttons(".", 1);
          } else {
            game_data.buttons(".", 0);
          }
          break;
        case 1:
          if ((gamepad.axes[1] < -0.5)) {
            game_data.buttons("w", 1);
          } else {
            game_data.buttons("w", 0);
          }
          if ((gamepad.axes[1] > 0.5)) {
            game_data.buttons("s", 1);
          } else {
            game_data.buttons("s", 0);
          }
          if ((gamepad.axes[0] < -0.5)) {
            game_data.buttons("a", 1);
          } else {
            game_data.buttons("a", 0);
          }
          if ((gamepad.axes[0] > 0.5)) {
            game_data.buttons("d", 1);
          } else {
            game_data.buttons("d", 0);
          }
          if (gamepad.buttons[0].pressed) {
            game_data.buttons("x", 1);
          } else {
            game_data.buttons("x", 0);
          }
          if (gamepad.buttons[5].pressed) {
            game_data.buttons("q", 1);
          } else {
            game_data.buttons("q", 0);
          }
          break;
        case 2:
          if ((gamepad.axes[1] < -0.5)) {
            game_data.buttons("t", 1);
          } else {
            game_data.buttons("t", 0);
          }
          if ((gamepad.axes[1] > 0.5)) {
            game_data.buttons("g", 1);
          } else {
            game_data.buttons("g", 0);
          }
          if ((gamepad.axes[0] < -0.5)) {
            game_data.buttons("f", 1);
          } else {
            game_data.buttons("f", 0);
          }
          if ((gamepad.axes[0] > 0.5)) {
            game_data.buttons("h", 1);
          } else {
            game_data.buttons("h", 0);
          }
          if (gamepad.buttons[0].pressed) {
            game_data.buttons("b", 1);
          } else {
            game_data.buttons("b", 0);
          }
          if (gamepad.buttons[5].pressed) {
            game_data.buttons("r", 1);
          } else {
            game_data.buttons("r", 0);
          }
          break;
        case 3:
          if ((gamepad.axes[1] < -0.5)) {
            game_data.buttons("i", 1);
          } else {
            game_data.buttons("i", 0);
          }
          if ((gamepad.axes[1] > 0.5)) {
            game_data.buttons("k", 1);
          } else {
            game_data.buttons("k", 0);
          }
          if ((gamepad.axes[0] < -0.5)) {
            game_data.buttons("j", 1);
          } else {
            game_data.buttons("j", 0);
          }
          if ((gamepad.axes[0] > 0.5)) {
            game_data.buttons("l", 1);
          } else {
            game_data.buttons("l", 0);
          }
          if (gamepad.buttons[0].pressed) {
            game_data.buttons(",", 1);
          } else {
            game_data.buttons(",", 0);
          }
          if (gamepad.buttons[5].pressed) {
            game_data.buttons("u", 1);
          } else {
            game_data.buttons("u", 0);
          }
          break;
        default: 
          break;
      }
    //}
  }
}

let res = resources();

let animationId = null;
let game_data = null;
let winner = 0;
let end_flag = false;
let start_flag = false;
var prevTimestamp=0;
var start_button = null;
var ope = new operation;

socket.on('update_game', (world) => {
  game_data = world;
});

socket.on('end', (win_player) => {
  end_flag = true;
  winner = win_player
});

socket.on('start', () => {
  start_flag = true;
  socket.emit('update', ope);
});

// Game loop
let drawAndUpdate = (timestamp) => {
    clear_screen();
  console.log(game_data);
    let progress = (timestamp - prevTimestamp) / 1000;
  if (!start_flag) { 
    if (start_button) {
      console.log('start');
      start_button.draw();
    }
  } else {
    if (end_flag) {
      stop_bgm();
      result_menu_screen(winner);
      //alert("Game End");
      return;
    }
    if (game_data != null) {

    // Update and draw

    for(let i=0; i<game_data.bombs.length; i++) {
      draw_bomb(game_data.bombs[i].x, game_data.bombs[i].y, game_data.bombs[i].type);
    }

    for(let i=0; i<game_data.fires.length; i++) {
      draw_fire(game_data.fires[i].x, game_data.fires[i].y);
    }

    for(let i=0; i<game_data.items.length; i++) {
      console.log("draw_item");
      console.log(game_data.items[i].x)
      draw_item(game_data.items[i].x, game_data.items[i].y, game_data.items[i].type);
    }

    for(let i=0; i<game_data.softblocks.length; i++) {
      draw_softblock(game_data.softblocks[i].x, game_data.softblocks[i].y);
    }

    for(let i=0; i<game_data.players.length; i++) {
      if (game_data.players[i].x >= 0) {
        draw_player(game_data.players[i].x, game_data.players[i].y, game_data.players[i].angle, i);
      }
    }

    for(let i=0; i<game_data.blocks.length; i++) {
      draw_block(game_data.blocks[i].x, game_data.blocks[i].y);
    }

    if ((game_data.time < 31.75 && game_data.time > 31.5) || (game_data.time < 31.25 && game_data.time > 31) || (game_data.time < 30.75 && game_data.time > 30.5)){
      draw_hurry_up();
    }
    var time = Math.floor(game_data.time);
    draw_number(800, 0, Math.floor(time/60));
    draw_number(850, 0, Math.floor((time-Math.floor(time/60)*60)/10));
    draw_number(890, 0, Math.floor((time-Math.floor(time/60)*60)%10));
    game_pad_update();
    socket.emit('update', ope);
  }
}
  requestAnimationFrame(drawAndUpdate);
};

const play = () => {
  start_button = new button(100, 50, 'start');
  drawAndUpdate();
}


var bgm_flag = false;
const processKey = (key, f) => {
  if (!bgm_flag) {
    //play_bgm();
    bgm_flag=true;
  }
  switch (key) {
    case "ArrowLeft":
      ope.left = f;
      break;
    case "ArrowRight":
      ope.right = f;
      break;
    case "ArrowUp":
      ope.up = f;
      break;
    case "ArrowDown":
      ope.down = f;
      break;
    case " ":
      ope.putbomb = f;
      break;
    case ".":
      ope.neutral = f;
      break;
  }
}

canvas.addEventListener("click", e => {
  // マウスの座標をCanvas内の座標とあわせるため
  const rect = canvas.getBoundingClientRect();
  const point = {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top
  };
  console.log(point);
  if (start_button != null && start_button.testHit(point)) {
    //console.log("clicked");
    socket.emit('my_name');
    start_button = null;
  }
});

document.addEventListener('keydown', e => processKey(e.key, 1));
document.addEventListener('keyup', e => processKey(e.key, 0));

play();

