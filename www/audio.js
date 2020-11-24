
export function play_audio(audio_file) {
  var myaudio = new Audio();
  myaudio.src = './audio/' + audio_file + '.wav';
  myaudio.play();
}

