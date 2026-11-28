"use strict";
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const {test} = require("node:test");
const duel = require("../portal/card-game.js");
const source = JSON.parse(fs.readFileSync(path.join(__dirname,"..","site","catalog.json"), "utf8"));
const cards = duel.prepare(source);
const begin = (faction="defender", difficulty="adept", seed=42) =>
  duel.transition(duel.createMatch(cards,faction,difficulty,seed), cards, {type:"opening",indices:[]});

test("catalog maps all 20 original tool IDs, each with publisher art; only observed ratings earn health", () => {
  const guild=Object.values(cards).filter(card=>card.faction==="defender");
  assert.equal(guild.length,20);
  assert.ok(guild.every(card=>card.art.startsWith("data:image/")));
  assert.equal(cards["g:crystaldiskinfo"].health,3);
  assert.match(cards["g:crystaldiskinfo"].rating.info,/4\.8\/5 SourceForge/);
  assert.equal(cards["g:wireshark"].health,2);
  assert.match(cards["g:wireshark"].rating.info,/Neutral/);
  const fake=structuredClone(source);
  fake.tools.find(t=>t.id==="crystaldiskinfo").reputation.sourceforge.status="unverified";
  assert.equal(duel.prepare(fake)["g:crystaldiskinfo"].health,2);
  assert.equal(Object.values(cards).filter(c=>c.lore).length,6);
});
test("mulligan is deterministic, preserves the source state and conserves all cards", () => {
  const m=duel.createMatch(cards,"defender","adept",242);
  const before=structuredClone(m);
  const n=duel.transition(m,cards,{type:"opening",indices:[0,2]});
  assert.deepEqual(m,before);
  assert.deepEqual(n,duel.transition(m,cards,{type:"opening",indices:[2,0,2]}));
  assert.equal(n.sides[0].hand[1],m.sides[0].hand[1]);
  assert.equal(new Set(n.sides[0].deck.concat(n.sides[0].hand)).size,20);
  assert.equal(n.sides[0].energy,1);
  assert.equal(duel.transition(m,cards,{type:"opening",indices:[3]}),null);
});
test("only legal units and actions resolve; Guard blocks strikes not actions", () => {
  const m=begin();
  const [you,other]=m.sides;
  you.energy=2; you.hand=["g:adwcleaner"];
  you.board=[{id:"g:wireshark",uid:"u1",ready:"any",health:2,shield:false}];
  other.board=[{id:"p:velvet-sentinel",uid:"u2",ready:"",health:4,shield:false},{id:"p:pixel-mite",uid:"u3",ready:"",health:1,shield:false}];
  assert.equal(duel.transition(m,cards,{type:"attack",uid:"u1",target:"core"}),null);
  assert.equal(duel.transition(m,cards,{type:"attack",uid:"u1",target:"u3"}),null);
  const strike=duel.transition(m,cards,{type:"attack",uid:"u1",target:"u2"});
  assert.equal(strike.sides[1].board[0].health,1);
  assert.equal(strike.sides[0].board[0].health,1);
  assert.equal(strike.sides[0].board[0].ready,"");
  assert.equal(duel.transition(m,cards,{type:"play",i:0,target:"u3"}).sides[1].board.length,1);
  assert.equal(duel.transition(m,cards,{type:"play",i:0,target:"core"}).sides[1].health,21);
  assert.equal(duel.transition(m,cards,{type:"play",i:0,target:"u1"}),null);
});
test("Shield absorbs one hit, Rush cannot target the core immediately", () => {
  const m=begin("phantom");
  const [you,other]=m.sides;
  you.energy=5; you.hand=["p:flicker-wisp"];
  other.board=[{id:"g:system-informer",uid:"s1",ready:"",health:3,shield:true}];
  const n=duel.transition(m,cards,{type:"play",i:0});
  const uid=n.sides[0].board[0].uid;
  assert.equal(duel.legalStrike(n,cards,0,uid,"core"),false);
  const a=duel.transition(n,cards,{type:"attack",uid,target:"s1"});
  assert.equal(a.sides[1].board[0].health,3);
  assert.equal(a.sides[1].board[0].shield,false);
  assert.equal(a.sides[0].board.length,0);
});
test("empty decks terminate deterministically and each faction/difficulty reaches a bounded end", () => {
  for (const faction of ["defender","phantom"]) for (const difficulty of ["scout","adept","master"]) {
    let m=begin(faction,difficulty);
    for (let i=0;i<50&&m.phase==="playing";i++) m=duel.transition(m,cards,{type:"pass"});
    assert.equal(m.phase,"ended");
    assert.ok([-1,0,1].includes(m.winner));
    assert.equal(duel.transition(m,cards,{type:"pass"}),null);
  }
});
