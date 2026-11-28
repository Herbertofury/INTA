/* Optional, local sound for verified actions. No assets, autoplay, or polling. */
(() => {
  'use strict';
  const root = globalThis;
  const KEY = 'inta:training-audio:v1';
  const PATTERNS = Object.freeze({
    check: [[660,0,.10],[880,.052,.13]],
    stage: [[523,0,.15],[659,.08,.16],[784,.16,.20]],
    handoff: [[392,0,.17],[523,.09,.18],[659,.18,.20],[1047,.28,.31]],
    card: [[277,0,.09],[415,.035,.10]],
    report: [[494,0,.16],[622,.095,.18],[988,.205,.25]],
    win: [[440,0,.16],[554,.09,.17],[659,.18,.19],[880,.26,.26]],
    loss: [[392,0,.15],[311,.105,.15],[247,.21,.24]],
  });
  let setting = { enabled:false, volume:45 };
  try {
    const saved = JSON.parse(root.localStorage.getItem(KEY));
    if (saved && typeof saved.enabled === 'boolean' && Number.isInteger(saved.volume) &&
        saved.volume >= 0 && saved.volume <= 100) setting = saved;
  } catch (_) { /* Keep the in-memory preference if storage is unavailable. */ }
  let context, master, voices = [], sequence = 0;
  function silence() {
    sequence += 1;
    for (const entry of voices) {
      try { entry.osc.stop(); entry.osc.disconnect(); entry.gain.disconnect(); } catch (_) { /* Ended. */ }
    }
    voices = [];
  }
  function update() {
    try { root.localStorage.setItem(KEY, JSON.stringify(setting)); } catch (_) { /* This visit only. */ }
    if (master) master.gain.setValueAtTime(setting.volume / 100 * .32, context.currentTime);
    if (!setting.enabled || !setting.volume) silence();
  }
  root.document?.addEventListener('visibilitychange', () => { if (root.document.hidden) silence(); });
  async function play(cue, event) {
    if (!event?.isTrusted || !setting.enabled || !setting.volume || root.document.hidden || !Object.hasOwn(PATTERNS,cue)) return false;
    const C = root.AudioContext || root.webkitAudioContext;
    if (!C) return false;
    silence();
    const token = sequence;
    try {
      if (!context) {
        context = new C();
        master = context.createGain();
        master.connect(context.destination);
      }
      if (context.state !== 'running') await context.resume();
      if (token !== sequence || context.state !== 'running' || !setting.enabled || !setting.volume || root.document.hidden) return false;
      const origin = context.currentTime + .008;
      master.gain.setValueAtTime(setting.volume / 100 * .32, origin);
      for (const [hz, offset, length] of PATTERNS[cue]) {
        const at = origin + offset;
        const oscillator = context.createOscillator(), gain = context.createGain();
        const entry = {osc:oscillator,gain};
        oscillator.type = cue === 'card' || cue === 'loss' ? 'triangle' : 'sine';
        oscillator.frequency.setValueAtTime(hz, at);
        gain.gain.setValueAtTime(.0001, at);
        gain.gain.exponentialRampToValueAtTime(.21, at+.016);
        gain.gain.exponentialRampToValueAtTime(.0001, at+length);
        oscillator.connect(gain);
        gain.connect(master);
        oscillator.onended = () => { oscillator.disconnect(); gain.disconnect(); voices = voices.filter(item => item !== entry); };
        voices.push(entry);
        oscillator.start(at);
        oscillator.stop(at+length+.018);
      }
      return true;
    } catch (_) { if (token === sequence) silence(); return false; }
  }
  function mount(host) {
    const template = host.ownerDocument.createElement('details');
    template.className = 'training-audio';
    template.innerHTML = '<summary>Sound feedback <small></small></summary><div><label><input type="checkbox"> Enable sounds</label><label>Volume <input type="range" min="0" max="100" step="5"></label><button type="button">Preview</button><span role="status"></span></div>';
    host.prepend(template);
    const enable = template.querySelector('input[type=checkbox]'), slider = template.querySelector('input[type=range]'),
      preview = template.querySelector('button'), label = template.querySelector('small'), status = template.querySelector('[role=status]');
    const render = () => {
      enable.checked = setting.enabled;
      slider.value = String(setting.volume);
      preview.disabled = !setting.enabled || !setting.volume;
      label.textContent = setting.enabled ? 'On' : 'Off';
    };
    enable.addEventListener('change', () => { setting.enabled = enable.checked; update(); render(); status.textContent = ''; });
    slider.addEventListener('input', () => { setting.volume = Number(slider.value); update(); render(); status.textContent = ''; });
    preview.addEventListener('click', async event => {
      const started = await play('stage',event);
      if (status.isConnected) status.textContent = started ? 'Preview played.' : 'Sound is unavailable or was muted.';
    });
    render();
  }
  root.INTATrainingAudio = Object.freeze({play,mount,stop:silence});
})();
