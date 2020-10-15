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
  }

  res.player0.src = "/image/1p.png";
  res.player1.src = "/image/1s.png";

  return res;
}

let clear_screen = () => {
  ctx.fillStyle = "green";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function draw_player(x, y, angle, player_id) {
  ctx.translate(x, y);
  ctx.translate(50,65);
  ctx.rotate(angle);
  ctx.translate(-50,-65);
  switch(player_id) {
    case 0:
      ctx.drawImage(res.player0, 0, 0, 100, 130);
      break;
    case 1:
      ctx.drawImage(res.player1, 0, 0, 100, 130);
      break;
    default:
      break;
  }
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
  var x = [];
  var y = [];
  var angle = [];
  for(let i=0; i<player_num; i++) {
    x[i] = game_data.x(i);
    y[i] = game_data.y(i);
    angle[i] = game_data.angle(i);
    draw_player(x[i], y[i], angle[i], i);
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
  }
}

document.addEventListener('keydown', e => processKey(e.key, 1));
document.addEventListener('keyup', e => processKey(e.key, 0));

play();
