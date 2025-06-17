"use strict";

const AudioContext = window.AudioContext || window.webkitAudioContext;
let audioContext;

var rawTime = 0;

let init_audio_output = function (sampleRate, channelCount, channelSampleCount) {
  console.log("Initialising audio");
  try {
    audioContext = new AudioContext({
      sampleRate: sampleRate
    });
  } catch (e) {
    return -1;
  }

  let audioBuffer;
  try {
    audioBuffer = audioContext.createBuffer(
      channelCount,
      channelSampleCount,
      sampleRate,
    );
  } catch (e) {
    return -2
  }

  let bufferDurationSecs = channelSampleCount / sampleRate;
  let timeStepMs = bufferDurationSecs * 1000;
  let offsetMs = 0;

  for (let _i = 0; _i < 2; _i++) {
    let closure = () => {
      let currentTime = audioContext.currentTime;
      let startTime = rawTime >= currentTime ? rawTime : currentTime;

      let ptr = wasm_exports.write_samples();
      let samples = new Float32Array(wasm_memory.buffer, ptr, channelSampleCount * channelCount);

      for (let channel = 0; channel < channelCount; channel++) {
        let tempSamples = []
        for (let i = channel; i < samples.length; i += channelCount) {
          tempSamples.push(samples[i]);
        }
        audioBuffer.copyToChannel(Float32Array.from(tempSamples), channel);
      }

      const source = audioContext.createBufferSource();
      source.buffer = audioBuffer;
      source.connect(audioContext.destination);
      source.onended = closure;
      source.start(startTime);

      rawTime = startTime + bufferDurationSecs;
    };

    setTimeout(closure, offsetMs);

    offsetMs += timeStepMs;
  }

  audioContext.resume();

  return 0;
}

let close_audio_output = () => {
  audioContext.close()
}

miniquad_add_plugin({
  register_plugin: function (importObject) {
    importObject.env.init_audio_output = init_audio_output;
    importObject.env.close_audio_output = close_audio_output;
  }, version: 1, name: "whinyaudio"
});

