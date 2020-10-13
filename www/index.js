import { GameData } from "bomberhuman";
import { memory } from "bomberhuman/bomberhuman_bg";

const canvas = document.getElementById("bomberhuman-canvas");
const game_data = GameData.new();
canvas.width = game_data.width();
canvas.height = game_data.height();

var ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
ctx.fillRect(0, 0, canvas.width, canvas.height);


//game_data.game_state()
const resources = () => {
  let res = {
    player: document.createElement('canvas'),
    enemy: document.createElement('canvas'),
    bullet: document.createElement('canvas'),
    particle: document.createElement('canvas')
  }

  // Particle
  res.particle.width = 20;
  res.particle.height = 20;
  let pCtx = res.particle.getContext('2d');
  pCtx.fillStyle = "darkviolet";
  pCtx.beginPath();
  pCtx.arc(10, 10, 10, 0, 2 * Math.PI);
  pCtx.fill();

  // Bullet
  res.bullet.width = 6;
  res.bullet.height = 6;
  let bCtx = res.bullet.getContext('2d');
  bCtx.fillStyle = "blue";
  bCtx.beginPath();
  bCtx.arc(3, 3, 3, 0, 2 * Math.PI);
  bCtx.fill();

  // Enemy
  res.enemy.width = 20;
  res.enemy.height = 20;
  let eCtx = res.enemy.getContext('2d');
  eCtx.fillStyle = "yellow";
  eCtx.beginPath();
  eCtx.arc(10, 10, 10, 0, 2 * Math.PI);
  eCtx.fill();

  // Player
  res.player.width = 20;
  res.player.height = 16;
  let plCtx = res.player.getContext('2d');
  plCtx.fillStyle = "red";
  plCtx.beginPath();
  plCtx.lineTo(20, 8);
  plCtx.lineTo(0, 16);
  plCtx.lineTo(0, 0);
  plCtx.fill();

  return res;
}

function draw_player(x, y, angle) {
  ctx.translate(x, y);
  ctx.rotate(angle);
  ctx.translate(0, -8);
  ctx.drawImage(res.player, 0, 0);
  ctx.setTransform(1, 0, 0, 1, 0, 0);

  ctx.fillStyle = "black";
}

let res = resources()

draw_player(100, 100, 100)

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
  ///module.draw();
  draw_player(100, 100, 100)

  // Some bookkeeping
  prevTimestamp = timestamp;
  requestAnimationFrame(drawAndUpdate);
};

const play = () => {
  //resize();
  drawAndUpdate();
}

function Actions() {
  this.move_left  = false;
  this.move_right = false;
  this.move_up    = false;
  this.move_down  = false;
};

let actions = new Actions();

const processKey = (key, f) => {
  switch (key) {
    case "ArrowLeft":
      //console.log(game_data.actions);
      console.log(game_data.actions_left());
      game_data.actions_left();
      //console.log(game_data.actions(3));
      //game_data.actions.move_left = f;
      break;
    case "ArrowRight":
      game_data.actions.move_right = f;
      break;
    case "ArrowUp":
      game_data.actions.move_up = f;
      break;
    case "ArrowDown":
      game_data.actions.move_down = f;
      break;
  }
}

document.addEventListener('keydown', e => processKey(e.key, true));
document.addEventListener('keyup', e => processKey(e.key, false));

play();
