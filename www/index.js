import { GameData } from "bomberhuman";

const canvas = document.getElementById("bomberhuman-canvas");
const game_data = GameData.new();
canvas.width = 800; //game_data.width();
canvas.height = 700; //game_data.height();

let ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
ctx.fillRect(0, 0, canvas.width, canvas.height);


const resources = () => {
  let res = {
    player0: document.createElement('img'),
    player1: document.createElement('img'),
    player2: document.createElement('img'),
    player3: document.createElement('img'),
    bomb: document.createElement('img'),
    fire: document.createElement('img'),
    bomb_item: document.createElement('img'),
    fire_item: document.createElement('img'),
    speed_item: document.createElement('img'),
    block: document.createElement('img'),
    softblock: document.createElement('img'),
  }

  res.player0.width = 50;
  res.player0.height = 50;
  res.player0.src = "image/human.png";
  res.player1.width = 50;
  res.player1.height = 50;
  res.player1.src = "image/human.png";
  res.player2.width = 50;
  res.player2.height = 50;
  res.player2.src = "image/human.png";
  res.player3.width = 50;
  res.player3.height = 50;
  res.player3.src = "image/human.png";
  res.bomb.width = 50;
  res.bomb.height = 50;
  res.bomb.src = "image/1p.png";
  res.fire.width = 50;
  res.fire.height = 50;
  res.fire.src = "image/fire.png";
  res.bomb_item.width = 50;
  res.bomb_item.height = 50;
  res.bomb_item.src = "image/bomb_item.png";
  res.fire_item.width = 50;
  res.fire_item.height = 50;
  res.fire_item.src = "image/fire_item.png";
  res.speed_item.width = 50;
  res.speed_item.height = 50;
  res.speed_item.src = "image/speed_item.png";
  res.block.width = 50;
  res.block.height = 50;
  res.block.src = "image/block.jpg";
  res.softblock.width = 50;
  res.softblock.height = 50;
  res.softblock.src = "image/soft_block.png";

  return res;
}

let clear_screen = () => {
  ctx.fillStyle = "green";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}


let result_menu_screen = () => {
  ctx.lineWidth = 2;
  ctx.fillStyle = "black";
  ctx.font = "100px cursive";
  ctx.textBaseline = 'center';
	ctx.textAlign = 'center';
  let x = canvas.width / 2
  let y = canvas.height / 4
  ctx.fillText("END", x, y);
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


function draw_bomb(x, y) {
  ctx.drawImage(res.bomb, x, y, res.bomb.width, res.bomb.height);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = "black";
}

function draw_fire(x, y) {
  ctx.drawImage(res.fire, x, y, res.fire.width, res.fire.height);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = "black";
}

function draw_item(x, y, type) {
  if (type == 1) {
    ctx.drawImage(res.fire_item, x, y, res.fire_item.width, res.fire_item.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  } else if (type == 2) {
    ctx.drawImage(res.bomb_item, x, y, res.bomb_item.width, res.bomb_item.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
  } else if (type == 3) {
    ctx.drawImage(res.speed_item, x, y, res.speed_item.width, res.speed_item.height);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = "black";
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

let res = resources();

let animationId = null;

// Game loop
let start = null;
let prevTimestamp = null;
let drawAndUpdate = (timestamp) => {
  // Initialization
  if (!prevTimestamp) {
    start = timestamp;
    prevTimestamp = timestamp;
    requestAnimationFrame(drawAndUpdate);
    return;
  }

  if (game_data.is_end()) {
    result_menu_screen();
    //alert("Game End");
    return;
  }

  // Update and draw
  let progress = (timestamp - prevTimestamp) / 1000;
  game_data.update(progress);
  clear_screen();

  let bomb_num = game_data.get_bomb_num();
  var b_x = [];
  var b_y = [];
  for(let i=0; i<bomb_num; i++) {
    b_x[i] = game_data.x(i, "bomb");
    b_y[i] = game_data.y(i, "bomb");
    draw_bomb(b_x[i], b_y[i]);
  }

  let fire_num = game_data.get_fire_num();
  var f_x = [];
  var f_y = [];
  for(let i=0; i<fire_num; i++) {
    f_x[i] = game_data.x(i, "fire");
    f_y[i] = game_data.y(i, "fire");
    draw_fire(f_x[i], f_y[i]);
  }

  let item_num = game_data.get_item_num();
  var i_x = [];
  var i_y = [];
  var i_type = [];
  for(let i=0; i<item_num; i++) {
    i_x[i] = game_data.x(i, "item");
    i_y[i] = game_data.y(i, "item");
    i_type[i] = game_data.item_type(i, "item");
    draw_item(i_x[i], i_y[i], i_type[i]);
  }

  let block_num = game_data.get_block_num();
  var bl_x = [];
  var bl_y = [];
  for(let i=0; i<block_num; i++) {
    bl_x[i] = game_data.x(i, "block");
    bl_y[i] = game_data.y(i, "block");
    draw_block(bl_x[i], bl_y[i]);
  }

  let softblock_num = game_data.get_softblock_num();
  var sbl_x = [];
  var sbl_y = [];
  for(let i=0; i<softblock_num; i++) {
    sbl_x[i] = game_data.x(i, "softblock");
    sbl_y[i] = game_data.y(i, "softblock");
    draw_softblock(sbl_x[i], sbl_y[i]);
  }

  let player_num = game_data.get_player_num();
  var p_x = [];
  var p_y = [];
  var angle = [];
  for(let i=0; i<player_num; i++) {
    p_x[i] = game_data.x(i, "player");
    p_y[i] = game_data.y(i, "player");
    angle[i] = game_data.angle(i);
    if (p_x[i] >= 0) {
      draw_player(p_x[i], p_y[i], angle[i], i);
    }
  }

  prevTimestamp = timestamp;
  requestAnimationFrame(drawAndUpdate);
};

const play = () => {
  drawAndUpdate();
}

const processKey = (key, f) => {
  switch (key) {
    case "ArrowLeft":
      game_data.buttons("left", f);
      break;
    case "ArrowRight":
      game_data.buttons("right", f);
      break;
    case "ArrowUp":
      game_data.buttons("up", f);
      break;
    case "ArrowDown":
      game_data.buttons("down", f);
      break;
    case " ":
      game_data.buttons("space", f);
      break;
    case "w":
      game_data.buttons("w", f);
      break;
    case "s":
      game_data.buttons("s", f);
      break;
    case "a":
      game_data.buttons("a", f);
      break;
    case "d":
      game_data.buttons("d", f);
      break;
    case "x":
      game_data.buttons("x", f);
      break;
    case "t":
      game_data.buttons("t", f);
      break;
    case "g":
      game_data.buttons("g", f);
      break;
    case "f":
      game_data.buttons("f", f);
      break;
    case "h":
      game_data.buttons("h", f);
      break;
    case "b":
      game_data.buttons("b", f);
      break;
    case "i":
      game_data.buttons("i", f);
      break;
    case "k":
      game_data.buttons("k", f);
      break;
    case "j":
      game_data.buttons("j", f);
      break;
    case "l":
      game_data.buttons("l", f);
      break;
    case ",":
      game_data.buttons(",", f);
      break;
  }
}


document.addEventListener('keydown', e => processKey(e.key, 1));
document.addEventListener('keyup', e => processKey(e.key, 0));

play();

