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
    player: document.createElement('img'),
  }

  res.player.src = "/image/1p.png";

  return res;
}

let clear_screen = () => {
  ctx.fillStyle = "green";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function draw_player(x, y, angle) {
  ctx.translate(x, y);
  //ctx.rotate(angle);

  ctx.drawImage(res.player, 0, 0, 100, 130);
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
  let x_0 = game_data.x(0);
  let y_0 = game_data.y(0);
  let angle_0 = game_data.angle(0);
  draw_player(x_0, y_0, angle_0);

  let x_1 = game_data.x(1);
  let y_1 = game_data.y(1);
  let angle_1 = game_data.angle(1);
  draw_player(x_1, y_1, angle_1);

  prevTimestamp = timestamp;
  requestAnimationFrame(drawAndUpdate);
};

const play = () => {
  //resize();
  drawAndUpdate();
}

const processKey = (key, f) => {
  switch (key) {
    case "ArrowLeft":
      game_data.actions("move_left", f);
      break;
    case "ArrowRight":
      game_data.actions("move_right", f);
      break;
    case "ArrowUp":
      game_data.actions("move_up", f);
      break;
    case "ArrowDown":
      game_data.actions("move_down", f);
      break;
  }
}

document.addEventListener('keydown', e => processKey(e.key, 1));
document.addEventListener('keyup', e => processKey(e.key, 0));

play();
