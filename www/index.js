import { GameData } from "bomberhuman";

const canvas = document.getElementById("bomberhuman-canvas");
const game_data = GameData.new();
canvas.width = game_data.width();
canvas.height = game_data.height();

var ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
ctx.fillRect(0, 0, canvas.width, canvas.height);


const resources = () => {
  let res = {
    player0: document.createElement('img'),
    player1: document.createElement('img'),
    bomb: document.createElement('img'),
    fire: document.createElement('img'),
  }

  res.player0.width = 100;
  res.player0.height = 130;
  res.player0.src = "/image/0p.gif";
  res.player1.width = 100;
  res.player1.height = 130;
  res.player1.src = "/image/8s.gif";
  res.bomb.width = 100;
  res.bomb.height = 130;
  res.bomb.src = "/image/1p.gif";
  res.fire.width = 100;
  res.fire.height = 130;
  res.fire.src = "/image/0s.gif";

  return res;
}

let clear_screen = () => {
  ctx.fillStyle = "green";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
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

  // Update and draw
  let progress = (timestamp - prevTimestamp) / 1000;
  game_data.update(progress);
  clear_screen();
  let player_num = game_data.get_player_num();
  var p_x = [];
  var p_y = [];
  var angle = [];
  for(let i=0; i<player_num; i++) {
    p_x[i] = game_data.p_x(i);
    p_y[i] = game_data.p_y(i);
    angle[i] = game_data.angle(i);
    draw_player(p_x[i], p_y[i], angle[i], i);
  }

  let bomb_num = game_data.get_bomb_num();
  var b_x = [];
  var b_y = [];
  for(let i=0; i<bomb_num; i++) {
    b_x[i] = game_data.b_x(i);
    b_y[i] = game_data.b_y(i);
    draw_bomb(b_x[i], b_y[i]);
  }

  let fire_num = game_data.get_fire_num();
  var f_x = [];
  var f_y = [];
  for(let i=0; i<fire_num; i++) {
    f_x[i] = game_data.f_x(i);
    f_y[i] = game_data.f_y(i);
    draw_fire(f_x[i], f_y[i]);
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
    case "l":
      game_data.buttons("l", f);
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
  }
}

document.addEventListener('keydown', e => processKey(e.key, 1));
document.addEventListener('keyup', e => processKey(e.key, 0));

play();
