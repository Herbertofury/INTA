/* Defense Circuit: deterministic tabletop combat. No network or device operations. */
(() => {
  'use strict';
  const SAVE_KEY = 'inta-defense-circuit-v1';
  const MAX_FLOOR = 999;
  const threats = {
    bluekeep: {
      name: 'BlueKeep', kind: 'CVE-2019-0708', risk: 9.8, cvss: true, mark: 'signal',
      summary: 'A critical flaw in Remote Desktop Services on some older Windows versions.',
      lore: 'Disclosed in 2019. Defenders prioritized exposed, unpatched legacy servers because the flaw could have enabled machine-to-machine spread.',
      fix: ['Verify the Windows edition and match it to Microsoft’s guidance. Install the applicable security update or replace the legacy system.', 'Restrict Remote Desktop to approved access paths. If a vulnerable host cannot be updated, isolate it until a supported remediation is in place.'],
      source: ['CERT-EU · BlueKeep advisory', 'https://cert.europa.eu/publications/security-advisories/2019-013/'],
      style: 'surge'
    },
    wannacry: {
      name: 'WannaCry', kind: 'Ransomware worm', risk: 9.2, mark: 'lock',
      summary: 'A ransomware outbreak that encrypted files and spread between vulnerable Windows machines.',
      lore: 'The May 2017 outbreak behaved like a worm: once a vulnerable computer was affected, other vulnerable computers on connected networks were at risk. A ransom notice and inaccessible files were visible signs.',
      fix: ['Isolate suspected hosts and contact your incident response lead. Restore only from a known-clean backup after containment and remediation.', 'Verify the relevant Windows security updates, including the historical MS17-010 bulletin where applicable. Retire unsupported systems and remove SMBv1 where it is no longer required.'],
      source: ['Microsoft · WannaCrypt customer guidance', 'https://www.microsoft.com/en-us/msrc/blog/2017/05/customer-guidance-for-wannacrypt-attacks/'],
      style: 'surge'
    },
    notpetya: {
      name: 'NotPetya', kind: 'Destructive malware', risk: 9.8, mark: 'fracture',
      summary: 'Destructive malware that made affected computers and data inaccessible, with severe recovery consequences.',
      lore: 'The June 2017 outbreak appeared to demand payment, but organizations faced widespread destruction and often needed to rebuild systems to recover.',
      fix: ['Isolate affected systems and involve incident response. Rebuild compromised hosts from trusted media and validate offline backups before restoring.', 'Verify patch coverage on older Windows machines, remove unneeded SMBv1, and strengthen segmentation and privileged access controls.'],
      source: ['Microsoft · Petya mitigation and recovery lessons', 'https://www.microsoft.com/en-us/security/blog/2018/02/05/overview-of-petya-a-rapid-cyberattack/'],
      style: 'armor'
    },
    citrix: {
      name: 'Citrix Bleed', kind: 'CVE-2023-4966', risk: 9.4, cvss: true, mark: 'gateway',
      summary: 'A sensitive-data exposure in affected NetScaler configurations that could put authenticated sessions at risk.',
      lore: 'Disclosed in 2023. Administrators could see suspicious session use rather than a visible ransom note, and updating alone did not end surviving sessions.',
      fix: ['Have the appliance owner confirm the affected configuration and apply the vendor’s fixed build.', 'After updating, end active and persistent sessions as the vendor directs. Review possible compromise through your incident response process before closing the case.'],
      source: ['NetScaler · Critical security update', 'https://www.netscaler.com/blog/news/cve-2023-4966-critical-security-update-now-available-for-netscaler-adc-and-netscaler-gateway/'],
      style: 'drain'
    },
    moveit: {
      name: 'MOVEit Transfer', kind: 'CVE-2023-34362', risk: 9.8, cvss: true, mark: 'gateway',
      summary: 'A critical MOVEit Transfer vulnerability that put managed file-transfer data at risk.',
      lore: 'Reported in 2023. The consequences were primarily data exposure from affected file-transfer services, not an inevitable ransomware screen on every workstation.',
      fix: ['Restrict web access to affected services as Progress directs. Apply the appropriate supported update and verify the installed build.', 'Preserve evidence, review for unauthorized access with the service owner, and follow the vendor’s restoration and incident response guidance before reopening access.'],
      source: ['Progress · MOVEit Transfer security advisory', 'https://community.progress.com/s/article/MOVEit-Transfer-Critical-Vulnerability-31May2023'],
      style: 'armor'
    },
    akira: {
      name: 'Akira', kind: 'Ransomware', risk: 9.1, mark: 'lock',
      summary: 'Ransomware linked to disruption and extortion across Windows and Linux environments.',
      lore: 'CISA and its partners documented this ransomware against both Windows and Linux environments. Encrypted services and extortion were separate pressures on victims.',
      fix: ['Activate your incident response plan and contain affected systems. Verify the scope before restoring from segregated, known-clean backups.', 'Prioritize fixes for exposed, vulnerable systems, require strong multifactor authentication for remote access, and keep backup recovery exercises current.'],
      source: ['CISA and partners · Akira mitigations', 'https://www.cisa.gov/sites/default/files/2024-04/aa24-109a-stopransomware-akira-ransomware_2.pdf#page=11'],
      style: 'surge'
    },
    lockbit: {
      name: 'LockBit 3.0', kind: 'Ransomware', risk: 9.6, mark: 'fracture',
      summary: 'A ransomware operation associated with extortion and disruption across multiple sectors.',
      lore: 'CISA and its partners described this as an affiliate-run ransomware operation. Different incidents could look different, from service outages to extortion demands.',
      fix: ['Isolate affected systems and coordinate with your incident response team. Preserve evidence and confirm backups are clean before restoring.', 'Prioritize fixes for known exploited vulnerabilities, enforce phishing-resistant multifactor authentication, and test offline recovery.'],
      source: ['CISA and partners · LockBit mitigations', 'https://www.cisa.gov/sites/default/files/2023-03/aa23-075a-stop-ransomware-lockbit.pdf'],
      style: 'drain'
    },
    log4shell: {
      name: 'Log4Shell', kind: 'CVE-2021-44228', risk: 10, cvss: true, mark: 'eye',
      summary: 'A critical Apache Log4j Core vulnerability that put an unusually broad range of applications at risk.',
      lore: 'Disclosed in December 2021. Vulnerable logging components could be buried in otherwise unrelated applications, widening the damage possible across a software estate.',
      fix: ['Inventory direct and bundled Log4j Core components. Upgrade affected products to vendor-supported fixed versions; confirm the fixed component is the one actually deployed.', 'Follow incident response procedures for potentially exposed systems and verify remediation across the estate, including bundled dependencies.'],
      source: ['Apache · Log4j security advisories', 'https://logging.apache.org/security.html'],
      style: 'surge'
    }
  };
  const zones = [
    { name: 'Endpoint ward', icon: '01', clue: 'Protect the workstations; restore trust at the edge.', scouts: ['bluekeep', 'wannacry', 'akira'], boss: 'notpetya' },
    { name: 'Gateway district', icon: '02', clue: 'Cover the access paths; confirm the service owner.', scouts: ['citrix', 'moveit', 'bluekeep'], boss: 'lockbit' },
    { name: 'Dependency core', icon: '03', clue: 'Trace what ships inside the stack, not just what appears on its label.', scouts: ['moveit', 'citrix', 'akira'], boss: 'log4shell' }
  ];
  const upgrades = {
    cache: { name: 'Patch cache', effect: 'Patch hits 3 harder.' },
    segment: { name: 'Microsegment', effect: 'Isolate provides 4 more shield.' },
    sensor: { name: 'Signal amplifier', effect: 'Trace marks one extra target and does 1 more damage.' },
    toolkit: { name: 'Extra action rack', effect: 'Start each turn with one additional action point.' },
    stock: { name: 'Cold storage', effect: 'Built-in repair heals 5 more integrity and immediately grants one repair charge.' },
    plating: { name: 'Redundant core', effect: 'Maximum integrity +10. Restore 10 immediately.' }
  };
  const kits = {
    analyst: { name: 'Analyst', detail: 'Starts with Signal amplifier. Trace into a Patch combo.', relic: 'sensor', hp: 36, charges: 2 },
    responder: { name: 'Responder', detail: 'Starts with Microsegment. Brace for heavy telegraphed attacks.', relic: 'segment', hp: 40, charges: 2 },
    engineer: { name: 'Engineer', detail: 'Starts with Patch cache. Beat the clock with stronger attacks.', relic: 'cache', hp: 34, charges: 3 }
  };
  const own = (object, key) => Object.hasOwn(object, key);
  const clamp = (n, min, max, fallback = min) => Number.isInteger(n) ? Math.min(max, Math.max(min, n)) : fallback;
  const hash = value => {
    let state = 2166136261;
    for (const c of String(value)) state = Math.imul(state ^ c.charCodeAt(0), 16777619);
    return state >>> 0;
  };
  const zoneFor = floor => zones[Math.floor((floor - 1) / 3) % zones.length];
  const isBoss = floor => floor % 3 === 0;
  const freshProfile = () => ({ archive: [], high: 0, circuits: 0 });
  const pick = (list, seed, field) => list[hash(seed + ':' + field) % list.length];
  const active = (run, id) => run.upgrades.includes(id);
  const capacity = run => 3 + Number(active(run, 'toolkit'));
  const keyList = (xs, object) => Array.isArray(xs) ? [...new Set(xs.filter(x => typeof x === 'string' && own(object, x)))] : [];
  const displayRisk = t => t.cvss ? 'CVSS 3.x · ' + t.risk.toFixed(1) : 'High impact · game rating ' + t.risk.toFixed(1);

  function newRun(seed, kit = 'responder') {
    const preset = kits[kit];
    return { seed, kit, phase: 'route', floor: 1, score: 0, hp: preset.hp, max: preset.hp, charges: preset.charges,
      upgrades: [preset.relic], field: null, last: null, note: 'Choose your first room.', choices: [] };
  }

  function route(run) {
    const z = zoneFor(run.floor);
    if (isBoss(run.floor)) return [{ id: 'boss', title: 'Breach chamber', summary: 'Face the sector boss and unlock its real-world recovery card.', enemy: z.boss }];
    const first = pick(z.scouts, run.seed, 'floor:' + run.floor);
    const second = pick(z.scouts.filter(candidate => candidate !== first), run.seed, 'branch:' + run.floor);
    return [
      { id: 'respond', title: 'Respond to incident', summary: 'Standard threat. Restore 4 integrity after containment.', enemy: first },
      { id: 'priority', title: 'Take the critical route', summary: 'Reinforced threat. Higher score and an extra repair charge.', enemy: second },
      { id: 'service', title: 'Service alcove', summary: 'Restore 11 integrity and add 1 repair charge. No encounter or score.', enemy: null }
    ];
  }

  function stats(id, floor, lane) {
    const hazard = threats[id].risk;
    const tier = Math.max(0, Math.round((hazard - 9) * 6));
    const hp = 15 + tier + Math.min(12, floor - 1) + (lane === 'boss' ? 13 : lane === 'priority' ? 6 : 0);
    return { hp, power: 4 + Math.max(0, Math.ceil(hazard - 9.2)) + Math.min(12, Math.floor((floor - 1) / 3)) + (lane === 'boss' ? 2 : lane === 'priority' ? 1 : 0) };
  }

  function intent(run) {
    const field = run.field;
    const base = stats(field.enemy, run.floor, field.lane).power;
    const specialty = threats[field.enemy].style;
    const boss = field.lane === 'boss';
    const lastStand = boss && field.hp * 2 <= field.max;
    if (field.turn % 3 !== 2) return { name: field.turn % 3 === 0 ? 'Counter pulse' : 'Pressure', hit: base + Number(lastStand), add: 0, drain: 0 };
    if (specialty === 'armor') return { name: 'Reinforce', hit: base - 2, add: 4 + Number(lastStand), drain: 0 };
    if (specialty === 'drain') return { name: 'Blackout', hit: base, add: 0, drain: 1 };
    return { name: 'Surge', hit: base + (lastStand ? 5 : 3), add: 0, drain: 0 };
  }

  function actions(run) {
    return [
      { id: 'trace', name: 'Trace', icon: '◎', cost: 1, text: 'Deal ' + (active(run, 'sensor') ? 3 : 2) + ' and mark the target. Marks power Patch.' },
      { id: 'patch', name: 'Patch', icon: '◇', cost: 2, text: 'Deal ' + (active(run, 'cache') ? 10 : 7) + ', plus 3 per mark. Spend all marks.' },
      { id: 'isolate', name: 'Isolate', icon: '⛨', cost: 1, text: 'Block ' + (active(run, 'segment') ? 9 : 5) + ' incoming damage and prevent this turn’s enemy side effect.' },
      { id: 'repair', name: 'Restore backup', icon: '+', cost: 1, text: 'Repair ' + (active(run, 'stock') ? 13 : 8) + ' integrity. Uses a repair charge.' }
    ];
  }

  function advance(run) {
    if (run.floor >= MAX_FLOOR) {
      run.phase = 'final';
      run.field = null; run.choices = [];
      run.note = 'Maximum endless depth cleared.';
      return;
    }
    run.floor++;
    run.field = null;
    run.phase = 'route';
    run.choices = [];
    run.note = 'Choose your next room.';
  }

  function clear(run, profile) {
    const f = run.field;
    const boss = f.lane === 'boss';
    run.score += Math.round(threats[f.enemy].risk * 2) + (boss ? 18 : f.lane === 'priority' ? 7 : 0);
    profile.high = Math.max(profile.high, run.score);
    run.last = f.enemy;
    run.note = f.lane === 'respond' ? 'Standard response complete. +4 integrity.' : f.lane === 'priority' ? 'Critical response complete. +1 repair charge.' : 'Sector boss contained. Field report unlocked.';
    if (f.lane === 'respond') run.hp = Math.min(run.max, run.hp + 4);
    if (f.lane === 'priority') run.charges = Math.min(5, run.charges + 1);
    if (!profile.archive.includes(f.enemy)) profile.archive.push(f.enemy);
    if (boss) {
      run.hp = Math.min(run.max, run.hp + 8);
      run.charges = Math.min(5, run.charges + 1);
      const available = Object.keys(upgrades).filter(id => !active(run, id))
        .sort((a, b) => hash(run.seed + ':' + run.floor + ':' + a) - hash(run.seed + ':' + run.floor + ':' + b));
      run.choices = available.slice(0, 3);
      if (run.floor === 9) profile.circuits = Math.min(1000000, profile.circuits + 1);
    }
    run.phase = 'won';
  }

  function transition(state, type, value) {
    const { run, profile } = state;
    if (!run || !['route', 'fight', 'won'].includes(run.phase)) return false;
    if (run.phase === 'route' && type === 'route') {
      const room = route(run).find(x => x.id === value);
      if (!room) return false;
      if (!room.enemy) {
        run.hp = Math.min(run.max, run.hp + 11);
        run.charges = Math.min(5, run.charges + 1);
        advance(run);
        run.note = 'Serviced. +11 integrity and +1 charge, up to capacity.';
      } else {
        const { hp } = stats(room.enemy, run.floor, room.id);
        run.field = { enemy: room.enemy, lane: room.id, hp, max: hp, shield: 0, turn: 1, ap: capacity(run), marks: 0, armored: 0, isolated: false };
        run.phase = 'fight';
        run.note = 'Encounter started. Check the next response before spending actions.';
      }
      return true;
    }
    if (run.phase === 'fight') {
      const f = run.field;
      if (type === 'act') {
        const action = actions(run).find(x => x.id === value);
        if (!action || f.ap < action.cost || (value === 'repair' && (run.charges === 0 || run.hp === run.max))) return false;
        f.ap -= action.cost;
        let damage = 0;
        if (value === 'trace') {
          damage = active(run, 'sensor') ? 3 : 2;
          f.marks = Math.min(3, f.marks + (active(run, 'sensor') ? 2 : 1));
        }
        if (value === 'patch') {
          damage = (active(run, 'cache') ? 10 : 7) + f.marks * 3;
          f.marks = 0;
        }
        if (damage) {
          const absorbed = Math.min(damage, f.armored);
          f.armored -= absorbed;
          f.hp = Math.max(0, f.hp - (damage - absorbed));
          run.note = action.name + ': ' + (damage - absorbed) + ' damage' + (absorbed ? ', ' + absorbed + ' absorbed by armor.' : '.');
        }
        if (value === 'isolate') {
          f.isolated = true;
          f.shield += active(run, 'segment') ? 9 : 5;
          run.note = 'Isolated. Incoming shield: ' + f.shield + '. Side effect blocked.';
        }
        if (value === 'repair') {
          run.charges--;
          const healed = Math.min(run.max - run.hp, active(run, 'stock') ? 13 : 8);
          run.hp += healed;
          run.note = 'Restored ' + healed + ' integrity.';
        }
        if (!f.hp) clear(run, profile);
        return true;
      }
      if (type === 'end') {
        const next = intent(run);
        const injury = Math.max(0, next.hit - f.shield);
        run.hp = Math.max(0, run.hp - injury);
        if (next.add && !f.isolated) f.armored = Math.min(12, f.armored + next.add);
        f.ap = capacity(run) - (f.isolated ? 0 : next.drain);
        f.shield = 0;
        f.isolated = false;
        f.turn++;
        run.note = next.name + ': ' + injury + ' integrity lost. New turn.';
        if (!run.hp) {
          run.phase = 'defeat';
          run.field = null;
          run.note = 'Run ended. Reports you earned are still in your archive.';
        }
        return true;
      }
    }
    if (run.phase === 'won' && type === 'next') {
      if (!run.choices.length && value !== 'continue') return false;
      if (run.choices.length && !run.choices.includes(value)) return false;
      if (run.choices.length) {
        run.upgrades.push(value);
        if (value === 'plating') { run.max += 10; run.hp += 10; }
        if (value === 'stock') run.charges = Math.min(5, run.charges + 1);
      }
      advance(run);
      return true;
    }
    if (run.phase === 'won' && type === 'finish' && run.floor >= 9 && isBoss(run.floor)) {
      run.phase = 'final';
      run.field = null; run.choices = [];
      run.note = 'Circuit secured. Your field reports and record score are saved.';
      return true;
    }
    return false;
  }

  function reload(raw) {
    if (!raw || typeof raw !== 'object') return { profile: freshProfile(), run: null };
    const p = raw.profile || {};
    const profile = { high: clamp(p.high, 0, 1000000), circuits: clamp(p.circuits, 0, 1000000), archive: keyList(p.archive, threats) };
    const r = raw.run;
    if (!r || typeof r.seed !== 'string' || !/^[A-Za-z0-9-]{1,32}$/.test(r.seed) || !own(kits, r.kit)) return { profile, run: null };
    const phase = ['route', 'fight', 'won', 'defeat', 'final'].includes(r.phase) ? r.phase : 'route';
    const level = clamp(r.floor, 1, MAX_FLOOR);
    const run = { ...newRun(r.seed, r.kit), phase, floor: level, score: clamp(r.score, 0, 1000000), upgrades: keyList(r.upgrades, upgrades),
      last: own(threats, r.last) ? r.last : null, note: typeof r.note === 'string' ? r.note.slice(0, 180) : 'Run restored.' };
    if (!run.upgrades.includes(kits[run.kit].relic)) run.upgrades.push(kits[run.kit].relic);
    run.max = kits[run.kit].hp + (active(run, 'plating') ? 10 : 0);
    run.hp = clamp(r.hp, phase === 'defeat' ? 0 : 1, run.max, run.max);
    run.charges = clamp(r.charges, 0, 5);
    run.choices = keyList(r.choices, upgrades).filter(id => !active(run, id)).slice(0, 3);
    if (phase === 'fight' || phase === 'won') {
      const f = r.field;
      const room = f && route(run).find(x => x.id === f.lane && x.enemy && x.enemy === f.enemy);
      if (!room) return { profile, run: null };
      const { hp } = stats(room.enemy, level, room.id);
      run.field = { enemy: room.enemy, lane: room.id, hp: clamp(f.hp, phase === 'fight' ? 1 : 0, hp, hp), max: hp,
        turn: clamp(f.turn, 1, 9999), ap: clamp(f.ap, 0, capacity(run), capacity(run)), shield: clamp(f.shield, 0, capacity(run) * 9),
        marks: clamp(f.marks, 0, 3), armored: clamp(f.armored, 0, 12), isolated: f.isolated === true };
      if (phase === 'won') run.last = room.enemy;
    }
    return { profile, run };
  }

  const esc = value => String(value).replace(/[&<>"']/g, c => ({ '&':'&amp;', '<':'&lt;', '>':'&gt;', '"':'&quot;', "'":'&#39;' })[c]);
  const weaponArt = {
    signal: '<path d="M17 29h13l6-14 8 35 7-19h17"/><circle cx="65" cy="31" r="6"/>',
    lock: '<path d="M27 31h30v23H27z M32 31v-8a10 10 0 0 1 20 0v8 M42 39v7"/><circle cx="42" cy="39" r="2"/>',
    fracture: '<path d="M26 12l19 16-9 12 20 16 M55 13l-6 10 M25 50l8-14 M44 44l18-13 M44 23l-7-12"/>',
    gateway: '<path d="M27 57V17h30v40z M35 49V25h14v24z M42 29v8 M39 34l3 3 3-3"/>',
    eye: '<path d="M18 34 Q42 9 66 34 Q42 59 18 34Z M15 34h-5 M69 34h5"/><circle cx="42" cy="34" r="9"/>'
  };

  function threatArt(kind) {
    return '<svg class="dc-threat-art" viewBox="0 0 84 68" fill="none" aria-hidden="true"><path class="dc-rings" d="M42 1l37 16v34L42 67 5 51V17z M42 7l31 14v26L42 61 11 47V21z"/><g>' + weaponArt[kind] + '</g></svg>';
  }
  function report(key) {
    const t = threats[key];
    if (!t) return '';
    return '<article class="dc-report" data-dc-report="' + key + '"><div class="dc-report-visual">' + threatArt(t.mark) + '</div><div><p class="dc-eyebrow">FIELD REPORT · ' + esc(t.kind) + '</p><h3>' + esc(t.name) + ': what to do on a real system</h3><p>' + esc(t.summary) + '</p><p class="dc-lore"><strong>Incident lore:</strong> ' + esc(t.lore) + '</p><p><small>' + esc(displayRisk(t)) + '</small></p><ol>' + t.fix.map(item => '<li>' + esc(item) + '</li>').join('') + '</ol><a href="' + esc(t.source[1]) + '" target="_blank" rel="noopener noreferrer">' + esc(t.source[0]) + ' ↗</a></div></article>';
  }
  const meter = (label, value, max, kind) => '<div class="dc-meter ' + kind + '"><span><b>' + esc(label) + '</b><span>' + value + ' / ' + max + '</span></span><meter min="0" max="' + max + '" value="' + value + '" aria-label="' + esc(label) + '"></meter></div>';
  const button = (type, id, label, detail, classes = '', disable = false) => '<button type="button" class="dc-button ' + classes + '" data-dc-action="' + type + '" data-dc-id="' + esc(id) + '"' + (disable ? ' disabled' : '') + '><strong>' + esc(label) + '</strong>' + (detail ? '<small>' + esc(detail) + '</small>' : '') + '</button>';
  const beginMarkup = () => '<div class="dc-start"><p class="dc-eyebrow">SELECT LOADOUT</p><h2>A different circuit every time.</h2><p>Three sectors, three bosses. Choose rooms, read the next move, and carry upgrades into a deeper cyclic run. Your archive and record are saved here.</p><div class="dc-grid">' +
    Object.keys(kits).map(id => button('start', id, kits[id].name, kits[id].detail)).join('') + '</div><details><summary>Daily circuit</summary><p>These loadouts use today’s UTC seed. Same date and loadout, same rooms.</p><div class="dc-grid">' + Object.keys(kits).map(id => button('daily', id, kits[id].name + ' · daily', kits[id].detail)).join('') + '</div></details></div>';

  function body(run) {
    if (!run) return beginMarkup();
    if (run.phase === 'defeat' || run.phase === 'final') return '<div class="dc-final"><p class="dc-eyebrow">' + (run.phase === 'final' ? '◆ CIRCUIT SECURED' : '▲ RUN ABORTED') + '</p><h2>' + run.score + ' points</h2><p>' + esc(run.note) + '</p>' + (run.last ? report(run.last) : '') + button('restart', 'ready', 'Choose another loadout', 'Start a different circuit') + '</div>';
    const z = zoneFor(run.floor);
    const chips = '<div class="dc-chip-row"><span>◆ ' + esc(kits[run.kit].name) + '</span><span>✚ ' + run.charges + ' / 5 repair charges</span><span>✦ ' + run.score + ' score</span><span title="Code for this run’s generated sequence">Seed ' + esc(run.seed) + '</span></div>';
    const head = '<div class="dc-run-head"><div><p class="dc-eyebrow">SECTOR ' + z.icon + ' · DEPTH ' + run.floor + ' · CYCLE ' + (Math.floor((run.floor - 1) / 9) + 1) + '</p><h2>' + esc(z.name) + '</h2><p>' + esc(z.clue) + '</p>' + chips + '</div>' + meter('Integrity', run.hp, run.max, 'is-integrity') + '</div>';
    const routeTrace = '<ol class="dc-route-trace" aria-label="Rooms in this sector">' + [1, 2, 3].map((room, index) => '<li class="' + (index < ((run.floor - 1) % 3) ? 'past' : index === ((run.floor - 1) % 3) ? 'current' : 'future') + '"' + (index === ((run.floor - 1) % 3) ? ' aria-current="step"' : '') + '>' + (room === 3 ? '◆ Boss' : '◈ Room ' + room) + '</li>').join('') + '</ol>';
    if (run.phase === 'route') return head + routeTrace + '<div class="dc-grid">' + route(run).map(room => button('route', room.id, room.title, (room.enemy ? threats[room.enemy].name + ' · ' : '') + room.summary, room.id === 'boss' ? 'is-boss' : '')).join('') + '</div>';
    if (run.phase === 'won') {
      const boss = run.field.lane === 'boss';
      return head + '<div class="dc-victory" data-dc-victory><p class="dc-eyebrow">✓ CONTAINED</p><h3>' + esc(threats[run.last].name) + '</h3><p>' + esc(run.note) + '</p>' + report(run.last) +
        (run.choices.length ? '<h4>Take one permanent upgrade for this run</h4><div class="dc-grid">' + run.choices.map(id => button('next', id, upgrades[id].name, upgrades[id].effect)).join('') + '</div>' : button('next', 'continue', isBoss(run.floor) && run.floor >= 9 ? 'Dive deeper' : 'Continue circuit', 'Open the next room')) +
        (boss && run.floor >= 9 ? '<div class="dc-finale">' + button('finish', 'finish', 'Bank the win', 'End the run; keep your field reports and score') + '</div>' : '') + '</div>';
    }
    const f = run.field; const t = threats[f.enemy]; const next = intent(run);
    return head + routeTrace + '<div class="dc-encounter">' + threatArt(t.mark) + '<div><p class="dc-eyebrow">' + (f.lane === 'boss' ? 'SECTOR BOSS' : 'ACTIVE THREAT') + ' · ' + esc(t.kind) + '</p><h3>' + esc(t.name) + '</h3><p>' + esc(displayRisk(t)) + ' · ' + esc(t.summary) + '</p>' + meter('Threat integrity', f.hp, f.max, 'is-enemy') + '</div></div>' +
      '<div class="dc-readout"><div><p class="dc-eyebrow">ENEMY INTENT · TURN ' + f.turn + '</p><strong>' + esc(next.name) + ' · ' + next.hit + ' damage' + (next.add ? ' + ' + next.add + ' armor' : next.drain ? ' + next-turn action loss' : '') + '</strong></div><div><p class="dc-eyebrow">YOUR TURN</p><strong>' + f.ap + ' / ' + capacity(run) + ' actions · ' + f.shield + ' shield · ' + f.marks + ' marks' + (f.armored ? ' · threat armor ' + f.armored : '') + '</strong></div></div>' +
      '<div class="dc-grid dc-actions">' + actions(run).map(a => button('act', a.id, a.icon + ' ' + a.name + ' · ' + a.cost + ' AP', a.text, '', f.ap < a.cost || (a.id === 'repair' && (!run.charges || run.hp === run.max)))).join('') + '</div><div class="dc-end">' + button('end', 'end', 'End turn →', 'Resolve the shown intent', 'is-primary') + '</div>';
  }

  function mount(host) {
    if (!host) return;
    let state;
    try { state = reload(JSON.parse(globalThis.localStorage.getItem(SAVE_KEY))); }
    catch (_) { state = reload(null); }
    const shell = host.ownerDocument.createElement('section');
    shell.className = 'defense-circuit';
    shell.setAttribute('aria-label', 'Defense Circuit');
    host.replaceChildren(shell);
    let archiveOpen = false, memoryOnly = false;
    function save() {
      try { globalThis.localStorage.setItem(SAVE_KEY, JSON.stringify(state)); }
      catch (_) { memoryOnly = true; }
    }
    function render(focus) {
      const run = state.run;
      const p = state.profile;
      const oldArchive = shell.querySelector('.dc-archive');
      if (oldArchive) archiveOpen = oldArchive.open;
      shell.innerHTML = '<header class="dc-head"><div><p class="dc-eyebrow">INTA TRAINING · SIMULATION</p><h1>Defense Circuit</h1><p>Secure the circuit. Learn the fix.</p></div><div class="dc-record"><span>PERSONAL RECORD</span><b>' + p.high + '</b><small>' + p.circuits + ' clears · ' + p.archive.length + ' / ' + Object.keys(threats).length + ' reports</small></div></header>' +
        '<p class="dc-boundary">Fictional combat only. The game never scans, connects to, patches or otherwise touches a system. Vulnerability CVSS and malware impact ratings are different; displayed difficulty is an adaptation, not a risk assessment.' + (memoryOnly ? ' Saving is unavailable in this browser session.' : '') + '</p>' +
        '<div class="dc-stage" tabindex="-1">' + body(run) + '</div><p class="dc-status" role="status" aria-live="polite">' + (run ? esc(run.note) : '') + '</p>' +
        '<details class="dc-archive"' + (archiveOpen ? ' open' : '') + '><summary>Field archive · ' + p.archive.length + ' unlocked</summary><p>Victories unlock each threat report here. All advisory links are available in the source register.</p>' +
        p.archive.map(report).join('') + '<details><summary>Source register · ' + Object.keys(threats).length + ' threats</summary>' + Object.keys(threats).map(key => report(key)).join('') + '</details></details>' +
        (run && ['route','fight','won'].includes(run.phase) ? '<details class="dc-exit"><summary>End this run early</summary>' + button('restart', 'leave', 'Abandon this run', 'Keep earned reports and record') + '</details>' : '');
      if (focus) {
        const candidates = [...shell.querySelectorAll('[data-dc-action]:not(:disabled)')];
        const same = candidates.find(x => x.dataset.dcAction === focus[0] && x.dataset.dcId === focus[1]);
        if (same && focus[0] === 'act' && run.phase === 'fight') same.focus({ preventScroll: true });
        else if (focus[0] === 'act' && run.phase === 'fight') shell.querySelector('[data-dc-action=end]').focus({ preventScroll: true });
        else shell.querySelector('.dc-stage').focus();
      }
    }
    shell.addEventListener('click', event => {
      const control = event.target.closest('button[data-dc-action]');
      if (!control || !shell.contains(control) || control.disabled) return;
      const { dcAction: type, dcId: id } = control.dataset;
      const previousPhase = state.run?.phase;
      if (type === 'start' || type === 'daily') {
        if (!own(kits, id)) return;
        const seed = type === 'daily' ? 'daily-' + new Date().toISOString().slice(0, 10) :
          (globalThis.crypto?.getRandomValues ? globalThis.crypto.getRandomValues(new Uint32Array(1))[0] : Math.floor(Math.random() * 4294967296)).toString(36).toUpperCase();
        state.run = newRun(seed, id);
      } else if (type === 'restart') {
        state.run = null;
      } else if (!transition(state, type, id)) return;
      save();
      const phase = state.run?.phase;
      if (phase !== previousPhase) {
        const cue = phase === 'won' ? 'report' : phase === 'final' ? 'win' : phase === 'defeat' ? 'loss' : null;
        if (cue) globalThis.INTATrainingAudio?.play(cue,event);
      }
      render([type, id]);
    });
    render();
  }

  globalThis.INTARogue = Object.freeze({ mount });
  if (typeof module === 'object' && module.exports) module.exports = { newRun, route, transition, freshProfile, reload, intent, stats, threats, report, actions };
})();
