/* INTA Packet Duel. Original small offline implementation; its conventional
 * opening swap, energy curve, board zoning and simultaneous combat were
 * chosen after studying Athena's pinned GameSync card-game source snapshot
 * (bce6c1bcb31c63d31c64185fef143a53d9c4d35f). No upstream code/assets are embedded.
 */
(function (root) {
  "use strict";

  const LIFE = 24, CAP = 9, BOARD = 5, HAND = 8;
  const RECORD = "inta.packet-duel.record.v1";
  const FACTIONS = {
    defender: { title: "Repair Guild", label: "Defend the signal", power: "Field repair", help: "Restore 3 integrity to your core.", cue: "◈" },
    phantom: { title: "Phantom Circuit", label: "Haunt the simulation", power: "Static pulse", help: "Deal 1 damage to the opposing core.", cue: "◆" },
  };
  const DIFFICULTIES = {
    scout: { title: "Scout", summary: "Forgiving AI. Enemy core starts at 20.", health: 20 },
    adept: { title: "Adept", summary: "Even match. Enemy core starts at 24.", health: 24 },
    master: { title: "Master", summary: "Sharper targeting. Enemy core starts at 28.", health: 28 },
  };
  const KEYWORDS = {
    guard: ["Guard", "Enemies must attack a Guard unit before other targets."],
    rush: ["Rush", "This unit can attack an opposing unit immediately, but not the core."],
    shield: ["Shield", "Ignore the first nonzero hit this unit takes."],
  };
  // INTA source IDs (not user input), role-balanced numbers, and abstract game effects.
  const GUILD = [
    ["autoruns", 1, 1, 2, "", "draw", 1],
    ["process-explorer", 1, 2, 1, "rush"],
    ["crystaldiskinfo", 1, 1, 2, "guard"],
    ["osquery", 2, 2, 3],
    ["sysmon", 2, 1, 4, "guard"],
    ["wireshark", 2, 3, 2, "rush"],
    ["testdisk", 2, 1, 3, "", "repair", 2],
    ["system-informer", 2, 2, 2, "shield"],
    ["clamav", 3, 3, 3, "", "sweep", 1],
    ["ventoy", 3, 2, 4, "guard"],
    ["7zip", 3, 4, 2, "shield"],
    ["restic", 4, 3, 4, "", "repair", 4],
    ["gparted", 4, 3, 5, "shield"],
    ["clonezilla", 5, 4, 5, "guard"],
    ["defender-offline", 5, 5, 4, "rush"],
    ["adwcleaner", 2, 0, 0, "", "hit", 3],
    ["malwarebytes", 4, 0, 0, "", "sweep", 3],
    ["rclone", 3, 0, 0, "", "draw", 2],
    ["winmerge", 2, 0, 0, "", "repair", 5],
    ["winscp", 4, 0, 0, "", "hit", 5],
  ];
  // The Phantom Circuit is deliberately fictional: no exploit, code or real-world action.
  const PHANTOMS = [
    ["lint-sprite", "Lint Sprite", "⟡", 1, 1, 2, "", "draw", 1],
    ["pixel-mite", "Pixel Mite", "✶", 1, 2, 1, "rush"],
    ["spectral-decoy", "Spectral Decoy", "◬", 1, 1, 2, "guard"],
    ["neon-mimic", "Neon Mimic", "◇", 2, 2, 3],
    ["velvet-sentinel", "Velvet Sentinel", "♢", 2, 1, 4, "guard"],
    ["flicker-wisp", "Flicker Wisp", "✧", 2, 3, 2, "rush"],
    ["echo-moth", "Echo Moth", "≋", 2, 1, 3, "", "repair", 2],
    ["glass-gremlin", "Glass Gremlin", "◈", 2, 2, 2, "shield"],
    ["phase-loom", "Phase Loom", "⎔", 3, 3, 3, "", "sweep", 1],
    ["moonglass-golem", "Moonglass Golem", "⬡", 3, 2, 4, "guard"],
    ["chrome-phantom", "Chrome Phantom", "⬢", 3, 4, 2, "shield"],
    ["mirage-vessel", "Mirage Vessel", "∿", 4, 3, 4, "", "repair", 4],
    ["opal-chimera", "Opal Chimera", "◈", 4, 3, 5, "shield"],
    ["eclipse-behemoth", "Eclipse Behemoth", "▰", 5, 4, 5, "guard"],
    ["violet-comet", "Violet Comet", "⋇", 5, 5, 4, "rush"],
    ["prism-jolt", "Prism Jolt", "ϟ", 2, 0, 0, "", "hit", 3],
    ["shadow-burst", "Shadow Burst", "✸", 4, 0, 0, "", "sweep", 3],
    ["dream-cache", "Dream Cache", "◇", 3, 0, 0, "", "draw", 2],
    ["echo-rewind", "Echo Rewind", "↺", 2, 0, 0, "", "repair", 5],
    ["gravity-wave", "Gravity Wave", "≈", 4, 0, 0, "", "hit", 5],
  ];
  // Factual case notes never change the rules of a fictional move.
  const CASES = {
    "prism-jolt": { name: "WannaCry", type: "Ransomware worm", impact: "Affected Windows fleets in 2017. It spread between susceptible computers and encrypted files, disrupting access to systems and files.", defense: "Install vendor security updates or retire unsupported systems. Isolate affected devices, notify incident response, and restore from verified clean backups.", source: "CISA WannaCry fact sheet", url: "https://www.cisa.gov/sites/default/files/FactSheets/NCCIC%20ICS_FactSheet_WannaCry_Ransomware_S508C.pdf" },
    "echo-moth": { name: "Emotet", type: "Malware campaign", impact: "Malicious email often appeared to come from familiar senders. An infection could introduce additional malware and lead to broader compromise.", defense: "Verify unexpected email through a separate channel, use protected document settings and email filtering, and isolate suspected endpoints for investigation.", source: "Canadian Centre for Cyber Security", url: "https://www.cyber.gc.ca/en/alerts-advisories/active-spam-campaigns-leveraging-emotet-malware" },
    "phase-loom": { name: "Log4Shell", type: "Apache Log4j vulnerability", impact: "An application may contain an affected logging component even if it is not listed as a standalone product. The flaw could allow compromise of an affected service.", defense: "Inventory affected dependencies, apply the vendor-supported update, verify the fix, and investigate signs of compromise separately.", source: "Joint CISA Log4j guidance", url: "https://www.cisa.gov/sites/default/files/publications/AA21-356A_Joint_CSA_Mitigating_Log4Shell_and_Other_Log4j-Related_Vulnerabilities.pdf" },
    "violet-comet": { name: "LockBit", type: "Ransomware operation", impact: "Intrusions could include both data theft and encryption, interrupting services and increasing pressure on affected organizations.", defense: "Require multifactor authentication for remote access, maintain isolated backups, and use the organization’s incident response and reporting process.", source: "Joint CISA LockBit advisory", url: "https://www.cisa.gov/sites/default/files/2023-06/aa23-165a_understanding_TA_LockBit.pdf" },
    "eclipse-behemoth": { name: "NotPetya", type: "Destructive malware", impact: "The 2017 incident appeared to be ransomware but caused severe disruption and lasting data loss. Organizations faced disruption across multiple systems.", defense: "Keep systems supported and patched, segment networks, and practice recovery from isolated known-good backups. Treat outbreak signs as an incident, not a payment decision.", source: "Microsoft security response", url: "https://www.microsoft.com/en-us/msrc/blog/2017/06/update-on-petya-malware-attacks/" },
    "neon-mimic": { name: "Mirai", type: "IoT botnet malware", impact: "Compromised connected devices joined botnets that could disrupt online services. The first visible sign for a device owner might be unusual traffic or poor performance.", defense: "Update device firmware, use unique credentials, remove unnecessary remote administration, and isolate suspected devices.", source: "Nigeria NCC-CSIRT security advisory", url: "https://csirt.ncc.gov.ng/index.php/resources/security-advisories/297-mirai-malware-threat-active-in-nigeria-cyberspace" },
  };
  const TARGET_PROMPT = "Choose a highlighted target.";
  const esc = value => String(value ?? "").replace(/[&<>"']/g, ch => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[ch]);
  const list = value => Array.isArray(value) ? value : [];
  const otherFaction = faction => faction === "defender" ? "phantom" : "defender";
  const swapSide = side => 1 - side;
  const legalHttps = value => {
    try {
      const url = new URL(value);
      return url.protocol === "https:" && !url.username && !url.password ? url.href : "";
    } catch (_) { return ""; }
  };
  const dataArt = asset => {
    const value = typeof asset === "string" ? asset : asset?.dataUri;
    return typeof value === "string" && /^data:image\/(?:png|svg\+xml|webp|jpeg|gif|x-icon|vnd\.microsoft\.icon);base64,[\w+/]+=*$/i.test(value) ? value : "";
  };

  function observedStat(tool, health) {
    const sf = tool?.reputation?.sourceforge;
    const rating = sf?.rating;
    const isReal = sf?.status === "observed" && Number.isFinite(rating?.value) &&
      Number.isFinite(rating?.scale) && rating.scale > 0 && rating.value >= 0 &&
      rating.value <= rating.scale && Number.isInteger(rating.reviews) && rating.reviews > 0;
    const url = isReal ? legalHttps(sf.url) : "";
    if (!url) return { bonus: 0, info: "Neutral balance · no observed user rating", url: "" };
    const bonus = health > 0 && rating.value / rating.scale >= .9 ? 1 : 0;
    return { bonus, info: `${rating.value}/${rating.scale} SourceForge · ${rating.reviews} reviews${bonus ? " · +1 health" : ""}`, url };
  }
  function prepare(data = {}) {
    const tools = new Map(list(data.tools).map(tool => [tool.id, tool]));
    const logos = new Map();
    for (const brand of list(data.brands?.brands)) {
      const art = dataArt(data.brands?.assets?.[brand.assetId]);
      if (brand.toolId && art && !logos.has(brand.toolId)) logos.set(brand.toolId, art);
    }
    const cards = Object.create(null);
    for (const [id, cost, attack, health, keyword, effect, value] of GUILD) {
      const tool = tools.get(id);
      if (!tool) continue; // No invented publisher name, artwork or rating.
      const rating = observedStat(tool, health);
      cards[`g:${id}`] = { id: `g:${id}`, tool: id, faction: "defender", name: tool.name, art: logos.get(id) || "",
        glyph: "◈", cost, attack, health: health + rating.bonus, keyword, effect, value, rating,
        category: tool.category };
    }
    for (const [id, name, glyph, cost, attack, health, keyword, effect, value] of PHANTOMS) {
      cards[`p:${id}`] = { id: `p:${id}`, faction: "phantom", name, glyph, cost, attack, health, keyword, effect, value, lore: CASES[id]?.name };
    }
    return cards;
  }

  // Inline deterministic generator; no network, timers, or global random state in the rules.
  function drawRandom(holder) {
    let n = holder.rng || 0x4ab57431;
    n ^= n << 13; n ^= n >>> 17; n ^= n << 5;
    holder.rng = n >>> 0;
    return holder.rng / 0x100000000;
  }
  function shuffled(values, holder) {
    const result = values.slice();
    for (let i = result.length - 1; i > 0; i--) {
      const j = Math.floor(drawRandom(holder) * (i + 1));
      [result[i], result[j]] = [result[j], result[i]];
    }
    return result;
  }
  function buildDeck(cards, faction, holder) {
    const ids = Object.values(cards).filter(card => card.faction === faction).map(card => card.id);
    if (ids.length < 8) return [];
    const stack = shuffled(ids, holder);
    // Both sides start with a playable early unit, regardless of the shuffle.
    const first = stack.findIndex(id => cards[id].cost === 1 && cards[id].health);
    if (first > 2) [stack[1], stack[first]] = [stack[first], stack[1]];
    return stack;
  }
  function side(faction, deck, health) {
    return { faction, deck, hand: deck.splice(0, 3), board: [], health, maxHealth: health, energy: 0, cap: 0,
      fatigue: 0, power: true };
  }
  function createMatch(cards, faction = "defender", difficulty = "adept", seed = 1) {
    if (!Object.hasOwn(FACTIONS, faction) || !Object.hasOwn(DIFFICULTIES, difficulty)) return null;
    const match = { rng: seed >>> 0, round: 0, phase: "opening", winner: null,
      difficulty, log: [], serial: 0, sides: [] };
    const deck = buildDeck(cards, faction, match), rivals = buildDeck(cards, otherFaction(faction), match);
    if (!deck.length || !rivals.length) return null;
    match.sides = [side(faction, deck, LIFE), side(otherFaction(faction), rivals, DIFFICULTIES[difficulty].health)];
    return match;
  }
  function copy(match) {
    return { ...match, log: match.log.slice(), sides: match.sides.map(s =>
      ({ ...s, deck: s.deck.slice(), hand: s.hand.slice(), board: s.board.map(unit => ({ ...unit })) })) };
  }
  function note(match, line) {
    match.log.push(line);
    if (match.log.length > 6) match.log.shift();
  }
  function coreDamage(match, who, amount, label) {
    const core = match.sides[who];
    core.health = Math.max(0, core.health - amount);
    note(match, `${label} → ${FACTIONS[core.faction].title} −${amount}.`);
  }
  function hurt(unit, amount) {
    if (amount > 0 && unit.shield) { unit.shield = false; return; }
    unit.health -= amount;
  }
  function clean(match) {
    for (const owner of match.sides) owner.board = owner.board.filter(unit => unit.health > 0);
    if (match.sides.every(s => s.health > 0)) return false;
    match.phase = "ended";
    match.winner = match.sides[0].health && !match.sides[1].health ? 0 : !match.sides[0].health && match.sides[1].health ? 1 : -1;
    return true;
  }
  function drawCard(match, who) {
    const player = match.sides[who];
    const card = player.deck.shift();
    if (!card) coreDamage(match, who, ++player.fatigue, "Empty deck fatigue");
    else if (player.hand.length < HAND) player.hand.push(card);
    else note(match, `${who ? "Their" : "Your"} hand is full; one draw was skipped.`);
  }
  function startTurn(match, who) {
    const player = match.sides[who];
    player.cap = Math.min(CAP, player.cap + 1);
    player.energy = player.cap;
    player.power = true;
    for (const unit of player.board) unit.ready = "any";
    if (match.round > 1) drawCard(match, who);
    clean(match);
  }
  function chooseOpening(match, cards, swaps) {
    if (match.phase !== "opening" || !Array.isArray(swaps)) return null;
    if (swaps.some(i => !Number.isInteger(i) || i < 0 || i >= match.sides[0].hand.length)) return null;
    const next = copy(match);
    const you = next.sides[0];
    const replaced = [];
    for (const i of [...new Set(swaps)].sort()) {
      if (!you.deck.length) break;
      replaced.push(you.hand[i]);
      you.hand[i] = you.deck.shift();
    }
    you.deck = shuffled(you.deck.concat(replaced), next);
    const rival = next.sides[1];
    const higher = rival.hand.findIndex(id => cards[id].cost > 3);
    if (higher !== -1 && rival.deck.length) {
      const discarded = rival.hand[higher];
      rival.hand[higher] = rival.deck.shift();
      rival.deck = shuffled(rival.deck.concat(discarded), next);
    }
    next.round = 1;
    next.phase = "playing";
    startTurn(next, 0);
    note(next, "The signal is open. Deploy a unit or pass to build energy.");
    return next;
  }
  function unitFrom(match, cards, who, uid) {
    return match.sides[who].board.find(unit => unit.uid === uid && cards[unit.id]);
  }
  function legalStrike(match, cards, who, uid, target) {
    const source = unitFrom(match, cards, who, uid);
    if (!source || !source.ready || !cards[source.id].attack) return false;
    const rival = match.sides[swapSide(who)];
    const guards = rival.board.filter(unit => cards[unit.id].keyword === "guard");
    if (target === "core") return source.ready === "any" && guards.length === 0;
    const unit = unitFrom(match, cards, swapSide(who), target);
    return Boolean(unit && (!guards.length || guards.includes(unit)));
  }
  function legalPlay(match, cards, who, i, target) {
    const s = match.sides[who], card = cards[s.hand[i]];
    if (!card || card.cost > s.energy || (card.health && s.board.length >= BOARD)) return false;
    if (!card.health && card.effect === "hit") return target === "core" || Boolean(unitFrom(match, cards, swapSide(who), target));
    return target === "";
  }
  function applyEffect(match, cards, who, card, target) {
    const s = match.sides[who], enemy = swapSide(who);
    if (card.effect === "heal" || card.effect === "repair") {
      const gain = Math.min(s.maxHealth - s.health, card.value);
      s.health += gain;
      if (gain) note(match, `Core restored +${gain}.`);
    } else if (card.effect === "sweep") {
      for (const unit of match.sides[enemy].board) hurt(unit, card.value);
    } else if (card.effect === "draw") {
      for (let i = 0; i < card.value; i++) if (s.health > 0) drawCard(match, who);
    } else if (card.effect === "hit") {
      if (target === "core") coreDamage(match, enemy, card.value, card.name);
      else hurt(unitFrom(match, cards, enemy, target), card.value);
    }
  }
  function play(match, cards, who, i, target) {
    if (!legalPlay(match, cards, who, i, target)) return false;
    const s = match.sides[who], card = cards[s.hand.splice(i, 1)[0]];
    s.energy -= card.cost;
    if (card.health) s.board.push({ id: card.id, uid: `u${++match.serial}`, health: card.health,
      shield: card.keyword === "shield", ready: card.keyword === "rush" ? "unit" : "" });
    note(match, `${who ? "Rival played" : "You played"} ${card.name}.`);
    applyEffect(match, cards, who, card, target);
    clean(match);
    return true;
  }
  function attack(match, cards, who, uid, target) {
    if (!legalStrike(match, cards, who, uid, target)) return false;
    const source = unitFrom(match, cards, who, uid), card = cards[source.id];
    source.ready = "";
    if (target === "core") coreDamage(match, swapSide(who), card.attack, card.name);
    else {
      const unit = unitFrom(match, cards, swapSide(who), target);
      const rival = cards[unit.id];
      hurt(unit, card.attack);
      hurt(source, rival.attack);
      note(match, `${card.name} clashes with ${rival.name}.`);
    }
    clean(match);
    return true;
  }
  function heroPower(match, who) {
    const s = match.sides[who];
    if (!s.power || s.energy < 2 || (s.faction === "defender" && s.health === s.maxHealth)) return false;
    s.power = false;
    s.energy -= 2;
    if (s.faction === "defender") {
      const heal = Math.min(s.maxHealth - s.health, 3);
      s.health += heal;
      note(match, `${FACTIONS[s.faction].title} repairs +${heal}.`);
    } else coreDamage(match, swapSide(who), 1, "Static pulse");
    clean(match);
    return true;
  }
  function chooseRivalPlay(match, cards) {
    const rival = match.sides[1], you = match.sides[0];
    const hard = match.difficulty === "master", soft = match.difficulty === "scout";
    const options = [];
    rival.hand.forEach((id, i) => {
      const card = cards[id], targeted = !card.health && card.effect === "hit";
      const targets = targeted ? ["core", ...you.board.map(x => x.uid)] : [""];
      for (const target of targets) {
        if (!legalPlay(match, cards, 1, i, target)) continue;
        let score = card.health ? card.attack * .7 + card.health * .62 + (card.keyword === "guard" && rival.health < 12 ? 2 : 0) : 0;
        if (card.effect === "hit") {
          const unit = target === "core" ? null : unitFrom(match, cards, 0, target);
          score += unit ? unit.shield ? 1 : card.value >= unit.health ? 3 + cards[unit.id].attack * .8 : 1
            : card.value + (card.value >= you.health ? 100 : you.health < 9 ? 4 : 0);
        } else if (card.effect === "sweep") score += you.board.reduce((a, unit) => a + (unit.shield ? .3 : card.value >= unit.health ? 2.7 : .8), 0);
        else if (card.effect === "repair") score += Math.min(rival.maxHealth - rival.health, card.value) * (rival.health < 9 ? 1 : .6);
        else if (card.effect === "draw") score += rival.deck.length >= card.value && rival.hand.length <= HAND - card.value ? card.value * 1.2 : -2;
        const noise = soft ? drawRandom(match) * 3 : .12 * drawRandom(match);
        options.push({ i, target, score: score - card.cost * (hard ? .25 : .18) + noise });
      }
    });
    options.sort((a, b) => b.score - a.score || a.i - b.i);
    return options[0]?.score > 0 ? options[0] : null;
  }
  function rivalTurn(match, cards) {
    startTurn(match, 1);
    const you = match.sides[0], rival = match.sides[1];
    for (let i = 0; match.phase === "playing" && i < CAP; i++) {
      const move = chooseRivalPlay(match, cards);
      if (!move || !play(match, cards, 1, move.i, move.target)) break;
    }
    for (const uid of rival.board.map(unit => unit.uid)) {
      if (match.phase !== "playing") break;
      const unit = unitFrom(match, cards, 1, uid);
      if (!unit?.ready) continue;
      const base = cards[unit.id];
      const choices = ["core", ...you.board.map(x => x.uid)].filter(target => legalStrike(match, cards, 1, uid, target))
        .map(target => {
          if (target === "core") return { target, score: base.attack >= you.health ? 100 : 2 + base.attack * .8 };
          const aimed = unitFrom(match, cards, 0, target), aimCard = cards[aimed.id];
          const kill = !aimed.shield && base.attack >= aimed.health;
          const alive = unit.shield || unit.health > aimCard.attack;
          return { target, score: (kill ? 4 + aimCard.attack : aimed.shield ? 1 : 2)
            - (alive ? 0 : base.attack * .95) + (match.difficulty === "scout" ? drawRandom(match) * 1.8 : 0) };
        }).sort((a, b) => b.score - a.score);
      if (choices.length) attack(match, cards, 1, uid, choices[0].target);
    }
    if (match.phase === "playing") heroPower(match, 1);
  }
  function transition(match, cards, action = {}) {
    if (!match) return null;
    if (action.type === "opening") return chooseOpening(match, cards, action.indices);
    if (match.phase !== "playing") return null;
    const next = copy(match);
    switch (action.type) {
      case "play": return Number.isInteger(action.i) && play(next, cards, 0, action.i, action.target || "") ? next : null;
      case "attack": return attack(next, cards, 0, action.uid, action.target) ? next : null;
      case "power": return heroPower(next, 0) ? next : null;
      case "concede": next.phase = "ended"; next.winner = 1; note(next, "Match conceded."); return next;
      case "pass":
        rivalTurn(next, cards);
        if (next.phase === "playing") { next.round++; startTurn(next, 0); }
        return next;
      default: return null;
    }
  }

  const effectText = card => ({
    repair: `Restore ${card.value} core integrity.`, hit: `Deal ${card.value} damage to a target.`,
    sweep: `Deal ${card.value} damage to every opposing unit.`, draw: `Draw ${card.value}.`,
  })[card.effect] || "";
  const tags = card => card.keyword ? KEYWORDS[card.keyword][0] : "";
  const typeName = card => card.health ? "Unit" : "Action";
  const cls = name => `inta-duel-${name}`;
  const effectDescription = card => [tags(card), effectText(card)].filter(Boolean).join(" · ");
  function cardArt(card) {
    return card.art ? `<img src="${esc(card.art)}" width="48" height="48" alt="" decoding="async">`
      : `<span class="${cls("glyph")}" aria-hidden="true">${esc(card.glyph)}</span>`;
  }
  function makeCard(card, {action = "", key = "", id = "", disabled = false, selected = false, unit = null, target = false, opening = false} = {}) {
    const hp = unit ? unit.health : card.health, attackValue = card.attack;
    const unitNote = unit ? unit.shield ? "Shield active" : unit.ready === "unit" ? "Rush ready" : unit.ready ? "Ready" : "Resting" : "";
    const desc = [card.name, unit ? `${attackValue} attack, ${hp} health` : `${card.cost} energy, ${typeName(card)}`, effectDescription(card), unitNote].filter(Boolean).join(". ");
    const extra = disabled ? " disabled" : "";
    const state = (opening || action === "select-attack") ? ` aria-pressed="${selected}"` : "";
    return `<button type="button" class="${cls("card")}" data-faction="${card.faction}" data-card-id="${esc(card.id)}" data-move="${action}" data-key="${esc(key)}" data-focus="${esc(id)}"${target ? " data-target" : ""}${extra}${state} aria-label="${esc(desc)}">
      <span class="${cls("card-art")}">${cardArt(card)}<span class="${cls("cost")}" aria-hidden="true">${unit ? (unit.shield ? "◉" : "") : card.cost}</span></span>
      <span class="${cls("card-body")}"><strong>${esc(card.name)}</strong><small>${esc(unitNote || effectDescription(card) || typeName(card))}</small>${!unit && card.lore ? `<small class="${cls("lore-tag")}">Case: ${esc(card.lore)}</small>` : ""}</span>
      ${card.health ? `<span class="${cls("stats")}" aria-hidden="true"><b>⚔ ${attackValue}</b><b>♥ ${hp}</b></span>` : ""}
    </button>`;
  }
  function localRead() {
    try {
      const stored = JSON.parse(root.localStorage?.getItem(RECORD) || "{}");
      const int = value => Number.isSafeInteger(value) && value >= 0 && value <= 99999 ? value : 0;
      return { wins: int(stored.wins), losses: int(stored.losses), draws: int(stored.draws), master: int(stored.master) };
    } catch (_) { return { wins: 0, losses: 0, draws: 0, master: 0 }; }
  }
  function localWrite(record) {
    try { root.localStorage?.setItem(RECORD, JSON.stringify(record)); } catch (_) { /* Private or blocked storage. */ }
  }
  const current = { match: null, swaps: new Set(), picked: null, faction: "defender", difficulty: "adept", seed: 0,
    record: null, counted: false, deckOpen: false, latestFocus: "" };
  const getRecord = () => current.record || (current.record = localRead());
  let pool, poolSource;
  function finish() {
    if (current.counted || current.match?.phase !== "ended") return;
    current.counted = true;
    const data = getRecord();
    const key = current.match.winner === -1 ? "draws" : current.match.winner === 0 ? "wins" : "losses";
    data[key] = Math.min(99999, data[key] + 1);
    if (current.match.winner === 0 && current.difficulty === "master") data.master = Math.min(99999, data.master + 1);
    localWrite(data);
  }
  function makeFresh(again) {
    const seed = again && current.seed ? current.seed : (Date.now() ^ Math.floor(Math.random() * 0x1fffffff)) >>> 0;
    current.seed = seed;
    current.match = createMatch(pool, current.faction, current.difficulty, seed);
    current.swaps = new Set();
    current.picked = null;
    current.counted = false;
    current.deckOpen = false;
  }
  function sectionTitle(extra) {
    return `<header class="${cls("header")}">
      <div><p class="${cls("kicker")}">INTA FIELD ARCADE <span>● OFFLINE SOLO</span></p>
      <h1 id="inta-duel-title">Packet Duel</h1><p>Build your board. Read the rival. Outlast the signal.</p></div>
      ${extra || `<a class="${cls("exit")}" href="#checklist">Field checklist ↗</a>`}
    </header>`;
  }
  function recordMarkup() {
    const record = getRecord();
    const buckets = [[record.wins,"Wins"], [record.losses,"Losses"], [record.draws,"Draws"],[record.master,"Master clears"]];
    return `<dl class="${cls("record")}" aria-label="Local arcade record">${buckets.map(([count, label]) =>
      `<div><dt>${label}</dt><dd>${count}</dd></div>`).join("")}</dl>`;
  }
  function renderRules(cards, lobby = false) {
    const deck = Object.values(cards).filter(card => card.faction === current.faction).sort((a, b) => a.cost - b.cost || a.name.localeCompare(b.name));
    return `<details class="${cls("rules")}"${current.deckOpen ? " open" : ""}>
      <summary>${lobby ? "How to play & card collection" : "Rules & your deck"} <span>${deck.length} cards</span></summary>
      <div><p>Start with three cards. Choose any to swap. Your energy refills and grows each round. A unit is usually ready the turn after deployment. Select a ready unit, then choose its target. Reduce the opposing core to zero to win. Taking an extra draw from an empty deck costs increasing integrity.</p>
      <ul>${Object.values(KEYWORDS).map(([name, description]) => `<li><strong>${name}</strong> — ${description}</li>`).join("")}<li><strong>Direct actions</strong> — Ignore Guard when targeting.</li></ul>
      <p>Deck tools and publisher marks come from INTA’s catalog. Only a catalog-observed SourceForge rating of at least 90% adds +1 base unit health. Unrated cards use neutral balance; stars and downloads are not user ratings. Stats are for the game, not product recommendations.</p>
      <ul class="${cls("collection")}">${deck.map(card => `<li>${cardArt(card)}<div><strong>${card.tool ? `<a href="#tool/${esc(card.tool)}" data-brand-decorated="true">${esc(card.name)}</a>` : esc(card.name)}</strong>
      <small>${card.cost} energy · ${card.health ? `${card.attack}/${card.health} · ` : ""}${esc(effectDescription(card) || typeName(card))}</small>
      ${card.rating ? `<small>${card.rating.url ? `<a href="${esc(card.rating.url)}" target="_blank" rel="noopener noreferrer" data-brand-decorated="true">${esc(card.rating.info)} ↗</a>` : esc(card.rating.info)}</small>` : ""}</div></li>`).join("")}</ul></div>
    </details>`;
  }
  function caseFiles() {
    return `<details class="${cls("rules")} ${cls("casebook")}">
      <summary>Real-world case files <span>${Object.keys(CASES).length} cases</span></summary>
      <div><p>These are factual defensive notes. Fictional moves and health numbers do not describe real techniques, reproduce incidents, or score security severity.</p>
      <div class="${cls("cases")}">${Object.entries(CASES).map(([id, fact]) => `<article>
        <p class="${cls("eyebrow")}">${esc(fact.type)}</p><h3>${esc(fact.name)}</h3>
        <small>Paired fictional card: ${esc(PHANTOMS.find(card => card[0] === id)[1])}</small>
        <p>${esc(fact.impact)}</p><p><strong>Defender’s response:</strong> ${esc(fact.defense)}</p>
        <a href="${esc(fact.url)}" target="_blank" rel="noopener noreferrer" data-brand-decorated="true">${esc(fact.source)} ↗</a>
      </article>`).join("")}</div></div>
    </details>`;
  }
  function lobby(cards) {
    return `${sectionTitle()}<div class="${cls("lobby")}">
      <section aria-labelledby="inta-duel-side"><div class="${cls("eyebrow")}">01 / Choose allegiance</div><h2 id="inta-duel-side">Which signal is yours?</h2>
      <div class="${cls("choose-side")}">${Object.entries(FACTIONS).map(([key, faction]) =>
        `<button type="button" class="${cls("faction")}" data-move="faction" data-key="${key}" data-focus="${key}" data-faction="${key}" aria-pressed="${current.faction === key}"><span aria-hidden="true">${faction.cue}</span><strong>${faction.title}</strong><small>${faction.label} · ${faction.power}</small></button>`).join("")}</div>
      <p class="${cls("fiction")}">The Phantom Circuit makes only fictional moves. The case files teach real impacts and defenses. Tool cards are references, not executable actions.</p>
      </section><section aria-labelledby="inta-duel-difficulty"><div class="${cls("eyebrow")}">02 / Pick a rival</div><h2 id="inta-duel-difficulty">Set the difficulty</h2>
      <div class="${cls("difficulty")}">${Object.entries(DIFFICULTIES).map(([key, d]) => `<button type="button" data-move="difficulty" data-key="${key}" data-focus="${key}" aria-pressed="${current.difficulty === key}"><strong>${d.title}</strong><small>${d.summary}</small></button>`).join("")}</div>
      <button type="button" class="${cls("primary")}" data-move="start" data-focus="start"${Object.keys(cards).length < PHANTOMS.length + 8 ? " disabled" : ""}>Open the duel <span aria-hidden="true">↗</span></button>
      ${Object.keys(cards).length < PHANTOMS.length + 8 ? "<p>The bundled tool catalog is required to deal a defensive deck.</p>" : ""}</section>
      </div>${recordMarkup()}${renderRules(cards, true)}${caseFiles()}`;
  }
  function openMatch(cards, match) {
    return `${sectionTitle()}<section class="${cls("opening")}" aria-labelledby="inta-duel-opening">
      <div class="${cls("eyebrow")}">OPENING HAND / ${DIFFICULTIES[match.difficulty].title}</div><h2 tabindex="-1" id="inta-duel-opening" data-focus="opening-title">Keep them or take a chance.</h2>
      <p>Select any starting cards you want to swap. Unselected cards stay in your hand.</p>
      <div class="${cls("hand")}">${match.sides[0].hand.map((id, i) => makeCard(cards[id], {action:"swap",key:i,id:`swap-${i}`,selected:current.swaps.has(i),opening:true})).join("")}</div>
      <div class="${cls("buttons")}"><button class="${cls("primary")}" type="button" data-move="deal" data-focus="deal">${current.swaps.size ? `Swap ${current.swaps.size} & start` : "Keep hand & start"}</button>
      <button class="${cls("alt")}" type="button" data-move="leave" data-focus="leave">Back to lobby</button></div>
    </section>${renderRules(cards)}`;
  }
  function sideHeader(player, who, target = false) {
    const label = FACTIONS[player.faction];
    return `<div class="${cls("core-group")}" data-faction="${player.faction}">
      <div class="${cls("portrait")}" aria-hidden="true">${label.cue}</div>
      <div class="${cls("core-meta")}"><strong>${who ? "Rival" : "You"} · ${label.title}</strong>
      <div class="${cls("life")}"><meter min="0" max="${player.maxHealth}" value="${player.health}" aria-label="${who ? "Rival" : "Your"} core integrity"></meter>
        <span>${player.health}/${player.maxHealth}</span></div>
      <small>${player.deck.length} in deck · ${player.hand.length} in hand</small></div>
      ${who && target ? `<button type="button" class="${cls("target-core")}" data-move="target" data-key="core" data-focus="t-core" data-target>Target core</button>` : ""}
    </div>`;
  }
  function targetable(match, cards, target) {
    const sel = current.picked;
    if (!sel) return false;
    return sel.type === "attack" ? legalStrike(match, cards, 0, sel.uid, target) : legalPlay(match, cards, 0, sel.i, target);
  }
  function matchPage(cards, match) {
    const wanted = current.picked;
    const rival = match.sides[1], you = match.sides[0];
    const prompt = wanted ? TARGET_PROMPT + (wanted.type === "attack" ? " Guard blocks direct attacks while standing." : " Actions ignore Guard.") :
      "Your turn. Play a card or select a ready unit. When you pass, the rival plays immediately.";
    const yours = you.board.map(unit => {
      const eligible = ["core", ...rival.board.map(x => x.uid)].some(target => legalStrike(match, cards, 0, unit.uid, target));
      return makeCard(cards[unit.id], { action:"select-attack", key:unit.uid, id:`y-${unit.uid}`, unit,
        selected:wanted?.uid === unit.uid, disabled:!eligible });
    }).join("");
    const theirs = rival.board.map(unit => {
      const possible = targetable(match,cards,unit.uid);
      return makeCard(cards[unit.id], { action:"target", key:unit.uid, id:`t-${unit.uid}`, unit, target:possible, disabled:!possible });
    }).join("");
    const isPower = you.power && you.energy >= 2 && (you.faction === "phantom" || you.health < you.maxHealth);
    return `${sectionTitle()}<div class="${cls("match")}" aria-label="Packet Duel board">
      <div class="${cls("turnbar")}"><span>ROUND ${String(match.round).padStart(2,"0")}</span><span>${DIFFICULTIES[match.difficulty].title} rival</span><span>YOUR TURN</span></div>
      <div class="${cls("arena")}">
        ${sideHeader(rival, 1, targetable(match, cards, "core"))}
        <div class="${cls("board-row")}" aria-label="Opposing board">${theirs || '<p>No rival units yet</p>'}</div>
        <div class="${cls("field-line")}"><span>◆</span><b>THE SIGNAL</b><span>◆</span></div>
        <div class="${cls("board-row")}" aria-label="Your board">${yours || '<p>Deploy a unit from your hand</p>'}</div>
        ${sideHeader(you, 0)}
      </div>
      <div class="${cls("command")}">
        <p tabindex="-1" data-focus="turn-notice" role="status" aria-live="polite">${esc(prompt)}</p>
        <div class="${cls("buttons")}">
          <span class="${cls("energy")}" aria-label="${you.energy} of ${you.cap} energy">◆ ${you.energy}/${you.cap} energy</span>
          ${wanted ? `<button class="${cls("alt")}" type="button" data-move="cancel" data-focus="cancel">Cancel selection</button>` : ""}
          <button class="${cls("alt")}" type="button" title="${esc(FACTIONS[you.faction].help)}" data-move="power" data-focus="power"${isPower ? "" : " disabled"}>${FACTIONS[you.faction].power} · 2</button>
          <button class="${cls("primary")}" type="button" data-move="pass" data-focus="pass">End turn <span aria-hidden="true">↗</span></button>
        </div>
      </div>
      <div class="${cls("hand")}" aria-label="Your hand">${you.hand.map((id, i) => {
        const card = cards[id], possible = legalPlay(match, cards, 0, i, card.effect === "hit" ? "core" : "");
        return makeCard(card, {action:"play", key:i, id:`h-${i}`, disabled:!possible,
          selected:wanted?.type === "play" && wanted.i === i});
      }).join("") || "<p>No cards in hand.</p>"}</div>
      <details class="${cls("log")}"><summary>Battle feed <span>${match.log.length} events</span></summary>
        <ul>${match.log.slice().reverse().map(line => `<li>${esc(line)}</li>`).join("")}</ul>
        <button class="${cls("alt")}" type="button" data-move="ask-concede" data-focus="ask-concede">Concede match</button>
        ${current.confirm ? `<div class="${cls("buttons")}"><button type="button" class="${cls("alt")}" data-move="concede" data-focus="concede">Confirm concede</button><button type="button" class="${cls("alt")}" data-move="back-concede" data-focus="back-concede">Keep playing</button></div>` : ""}
      </details>
    </div>${renderRules(cards)}`;
  }
  function endMatch(cards, match) {
    const win = match.winner === 0, draw = match.winner === -1;
    const words = win ? ["SIGNAL SECURED", "Victory!"] : draw ? ["SIGNAL COLLISION","Draw"] : ["SIGNAL LOST","Defeat"];
    return `${sectionTitle()}<section class="${cls("final")}" data-win="${win}" aria-labelledby="inta-duel-result">
      <div class="${cls("award")}" aria-hidden="true">${win ? "✦" : "◆"}</div>
      <p class="${cls("eyebrow")}">${words[0]}</p><h2 tabindex="-1" id="inta-duel-result" data-focus="result">${words[1]}</h2>
      <p>${FACTIONS[match.sides[0].faction].title} · ${DIFFICULTIES[match.difficulty].title} · ${match.round} round${match.round === 1 ? "" : "s"}</p>
      <p>${win ? "Guard, careful trades and energy discipline turn a chaotic board into a win." : "Try a different opening hand. Cheap units build the board; fresh Guard buys you a turn."}</p>
      <div class="${cls("buttons")}"><button class="${cls("primary")}" type="button" data-move="start" data-focus="start">New match</button>
        <button class="${cls("alt")}" type="button" data-move="retry" data-focus="retry">Replay this deal</button>
        <button class="${cls("alt")}" type="button" data-move="leave" data-focus="leave">Change sides</button></div></section>
      ${recordMarkup()}${renderRules(cards)}${caseFiles()}`;
  }

  function mount(container, data) {
    if (!container) return;
    if (data !== poolSource || !pool) { pool = prepare(data); poolSource = data; }
    if (container.firstElementChild?.hasAttribute("data-inta-card-game")) return;
    container.innerHTML = `<div class="inta-duel" data-inta-card-game data-no-link></div>`;
    const pane = container.firstElementChild;
    function paint(preferred = "") {
      const m = current.match;
      pane.innerHTML = !m ? lobby(pool) : m.phase === "opening" ? openMatch(pool, m) : m.phase === "playing" ? matchPage(pool, m) : endMatch(pool, m);
      if (current.confirm) pane.querySelector(`.${cls("log")}`).open = true;
      if (!preferred) return;
      const nodes = Array.from(pane.querySelectorAll("[data-focus]"));
      const position = nodes.find(el => el.dataset.focus === preferred && !el.disabled) || nodes.find(el => el.dataset.focus === "turn-notice");
      position?.focus({preventScroll:true});
    }
    function commit(move, keep = "") {
      const next = transition(current.match, pool, move);
      if (!next) return;
      current.match = next;
      current.picked = null;
      current.confirm = false;
      finish();
      paint(next.phase === "ended" ? "result" : keep || "turn-notice");
    }
    pane.addEventListener("click", event => {
      const button = event.target.closest("button[data-move]");
      if (!button || !pane.contains(button) || button.disabled) return;
      current.deckOpen = Boolean(pane.querySelector(`.${cls("rules")}`)?.open);
      const move = button.dataset.move, key = button.dataset.key, focus = button.dataset.focus;
      if (move === "faction" && !current.match && Object.hasOwn(FACTIONS, key)) {current.faction = key; paint(focus); return;}
      if (move === "difficulty" && !current.match && Object.hasOwn(DIFFICULTIES, key)) {current.difficulty = key; paint(focus); return;}
      if (move === "start" || move === "retry") {current.confirm = false; makeFresh(move === "retry"); paint("opening-title"); return;}
      if (move === "leave" && current.match?.phase !== "playing") {current.match = null; current.confirm = false; paint("start"); return;}
      if (move === "swap" && current.match?.phase === "opening") {
        const i = Number(key); current.swaps.has(i) ? current.swaps.delete(i) : current.swaps.add(i); paint(focus); return;
      }
      if (move === "deal") { commit({type:"opening",indices:[...current.swaps]}); return; }
      if (current.match?.phase !== "playing") return;
      if (move === "play") {
        const i = Number(key), card = pool[current.match.sides[0].hand[i]];
        if (card?.effect === "hit") { current.picked = {type:"play",i}; paint("t-core"); }
        else commit({type:"play",i}, focus);
      } else if (move === "select-attack") {
        current.picked = {type:"attack",uid:key};
        const legal = targetable(current.match, pool, "core") ? "core" :
          current.match.sides[1].board.find(unit => targetable(current.match, pool, unit.uid))?.uid;
        paint(legal ? `t-${legal}` : focus);
      } else if (move === "target" && current.picked) commit({...current.picked,target:key});
      else if (move === "cancel") { current.picked = null; paint("turn-notice"); }
      else if (move === "ask-concede" || move === "back-concede") { current.confirm = move === "ask-concede"; paint(current.confirm ? "concede" : "turn-notice"); }
      else if (["pass","power","concede"].includes(move)) commit({type:move},focus);
    });
    paint();
  }
  const api = Object.freeze({mount, prepare, createMatch, transition, legalPlay, legalStrike});
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.INTACardGame = api;
})(typeof globalThis !== "undefined" ? globalThis : this);
