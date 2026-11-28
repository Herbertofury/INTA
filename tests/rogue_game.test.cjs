const test = require('node:test');
const assert = require('node:assert/strict');
const game = require('../portal/rogue-game.js');

const setup = (seed = 'INTA', kit = 'analyst') => ({ run: game.newRun(seed, kit), profile: game.freshProfile() });
const take = (state, kind, key) => assert.equal(game.transition(state, kind, key), true);

test('deterministic rooms vary by seed and by lane; stronger severity increases same-lane stats', () => {
  const a = game.route(setup('daily-2026-11-28').run);
  assert.deepEqual(a, game.route(setup('daily-2026-11-28', 'responder').run));
  assert.notEqual(a[0].enemy, a[1].enemy);
  const variants = new Set(Array.from({ length: 32 }, (_, i) => game.route(setup('SEED-' + i).run)[0].enemy));
  assert.ok(variants.size > 1);
  assert.ok(game.stats('log4shell', 2, 'boss').hp >= game.stats('citrix', 2, 'boss').hp);
  assert.ok(game.stats('log4shell', 2, 'boss').hp > game.stats('log4shell', 2, 'respond').hp);
});

test('real CVSS and fictional impact charts are clearly labeled; all reports include defensive evidence', () => {
  for (const [id, threat] of Object.entries(game.threats)) {
    const value = game.report(id);
    assert.ok(threat.source[1].startsWith('https://'));
    assert.match(value, /Incident lore:/);
    assert.match(value, /what to do on a real system/);
    assert.match(value, threat.cvss ? /CVSS 3\.x/ : /game rating/);
    assert.ok(threat.fix.every(text => text.length > 60));
  }
});

test('blocked/unknown moves and wrong-stage controls do not consume state', () => {
  const s = setup();
  assert.equal(game.transition(s, 'route', '__proto__'), false);
  take(s, 'route', 'respond');
  assert.equal(game.transition(s, 'act', 'repair'), false);
  take(s, 'act', 'patch');
  assert.equal(game.transition(s, 'act', 'patch'), false);
  assert.equal(game.transition(s, 'next', 'cache'), false);
  assert.equal(s.run.field.ap, 1);
});

test('shield prevents telegraphed effects; a save restores the exact ongoing encounter', () => {
  const s = setup('INTA', 'responder');
  take(s, 'route', 'service');
  take(s, 'route', 'service');
  take(s, 'route', 'boss');
  take(s, 'act', 'trace');
  take(s, 'end');
  assert.equal(game.intent(s.run).name, 'Reinforce');
  take(s, 'act', 'isolate');
  assert.deepEqual(game.reload(JSON.parse(JSON.stringify(s))), s);
  const hp = s.run.hp;
  take(s, 'end');
  assert.equal(s.run.field.armored, 0);
  assert.equal(s.run.hp, hp);
});

test('clear, archive, upgrade, continue to a deeper cycle, and bank an actual boss win', () => {
  const s = setup('RECORD');
  let sawUnlocked = false;
  for (let floor = 1; floor <= 12; floor++) {
    if (floor % 3 !== 0) { take(s, 'route', 'service'); continue; }
    take(s, 'route', 'boss');
    for (let i = 0; i < 30 && s.run.phase === 'fight'; i++) {
      if (s.run.field.ap > 2) {
        if (s.run.hp <= s.run.max - 8 && s.run.charges) take(s, 'act', 'repair');
        else take(s, 'act', 'trace');
      }
      if (s.run.phase !== 'fight') break;
      take(s, 'act', 'patch');
      if (s.run.phase === 'fight' && s.run.field.ap && game.intent(s.run).add) take(s, 'act', 'isolate');
      if (s.run.phase === 'fight') take(s, 'end');
    }
    assert.equal(s.run.phase, 'won', 'boss ' + floor);
    sawUnlocked = s.profile.archive.includes(s.run.last);
    assert.match(game.report(s.run.last), /<li>/);
    if (floor === 12) take(s, 'finish', 'finish');
    else take(s, 'next', s.run.choices.find(x => x === 'toolkit') || s.run.choices[0] || 'continue');
  }
  assert.ok(sawUnlocked);
  assert.equal(s.profile.archive.length, 3);
  assert.equal(s.profile.circuits, 1);
  assert.equal(s.profile.high, s.run.score);
  assert.deepEqual(game.reload(JSON.parse(JSON.stringify(s))), s, 'banked victory also reloads');
});

test('discard unknown cards, invalid progress and encounter mismatch on restore', () => {
  const s = setup();
  s.profile.archive = ['__proto__', 'akira', 'akira'];
  s.profile.high = 'Infinity';
  s.run.upgrades.push('__proto__');
  const valid = game.reload(s);
  assert.deepEqual(valid.profile.archive, ['akira']);
  assert.equal(valid.profile.high, 0);
  assert.deepEqual(valid.run.upgrades, ['sensor']);
  take(s, 'route', 'respond');
  s.run.field.enemy = 'log4shell';
  assert.equal(game.reload(s).run, null);
});
