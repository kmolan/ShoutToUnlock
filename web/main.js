import init, { ShoutLock, levelDbfs } from "./pkg/shout_to_unlock.js";

const LEVEL_FLOOR_DBFS = -60;

const element = (id) => document.getElementById(id);
const minimumLevel = element("minimum-level");
const minimumSeconds = element("minimum-seconds");
const startButton = element("start");
const card = document.querySelector("main");

// squish [LEVEL_FLOOR_DBFS, 0] dBFS into [0, 100] % for the bars.
const levelPercent = (dbfs) => Math.max(0, Math.min(100, (1 - dbfs / LEVEL_FLOOR_DBFS) * 100));

function showLimits() {
  element("minimum-level-text").textContent = `${minimumLevel.value} dBFS`;
  element("minimum-seconds-text").textContent = `${Number(minimumSeconds.value).toFixed(1)} s`;
  element("threshold-mark").style.left = `${levelPercent(Number(minimumLevel.value))}%`;
}

function show(lock) {
  const level = lock.lastLevelDbfs;
  element("level-bar").style.width = `${levelPercent(level)}%`;
  element("level-bar").classList.toggle("loud", level >= lock.minimumLevelDbfs);
  element("level-text").textContent = Number.isFinite(level) ? `${level.toFixed(1)} dBFS` : "–∞ dBFS";
  element("progress-bar").style.width = `${lock.progress * 100}%`;
  element("progress-text").textContent = `${lock.loudSeconds.toFixed(1)} / ${lock.minimumSeconds.toFixed(1)} s`;
}

function setListening(listening) {
  startButton.disabled = minimumLevel.disabled = minimumSeconds.disabled = listening;
}

function setOpen(open) {
  card.classList.toggle("open", open);
  element("padlock").textContent = open ? "🔓" : "🔒";
}

async function listen() {
  setListening(true);
  setOpen(false);
  element("status").textContent = "SCREAM.";

  // automatic gain control makes a scream and a whisper sound the same. I do not want that. I want CHAOS.
  const stream = await navigator.mediaDevices.getUserMedia({
    audio: { autoGainControl: false, noiseSuppression: false, echoCancellation: false },
  });
  const context = new AudioContext();
  const analyser = context.createAnalyser();
  analyser.fftSize = 2048;
  context.createMediaStreamSource(stream).connect(analyser);

  const samples = new Float32Array(analyser.fftSize);
  const windowSeconds = analyser.fftSize / context.sampleRate;
  const lock = new ShoutLock(Number(minimumLevel.value), Number(minimumSeconds.value));
  let previousTime = context.currentTime;

  // analyser windows overlap, so each frame only counts the elapsed time, max one window. no double-dipping ur screams.
  function frame() {
    analyser.getFloatTimeDomainData(samples);
    const now = context.currentTime;
    lock.processLevel(levelDbfs(samples), Math.min(now - previousTime, windowSeconds));
    previousTime = now;
    show(lock);
    if (!lock.isUnlocked) {
      requestAnimationFrame(frame);
      return;
    }
    stream.getTracks().forEach((track) => track.stop());
    context.close();
    setOpen(true);
    element("status").textContent = `ur in. peak ${lock.peakLevelDbfs.toFixed(1)} dBFS. the neighbours have been notified.`;
    lock.free();
    startButton.textContent = "again!!";
    setListening(false);
  }
  requestAnimationFrame(frame);
}

await init();
minimumLevel.addEventListener("input", showLimits);
minimumSeconds.addEventListener("input", showLimits);
showLimits();
startButton.addEventListener("click", () =>
  listen().catch((error) => {
    element("status").textContent = `the mic said no: ${error.message}`;
    setListening(false);
  }),
);
