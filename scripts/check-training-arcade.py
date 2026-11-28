#!/usr/bin/env python3
"""Exercise both training routes through the same packaged, offline Hub."""
from pathlib import Path


def check_training_arcade(page, html: Path, output: Path):
    location = html.resolve().as_uri()
    page.goto(location + '#checklist')
    page.locator('.mission-training').get_by_role('link', name='Defense Circuit').click()
    page.get_by_role('heading', name='Defense Circuit', exact=True).wait_for()
    start = page.locator('button[data-dc-action="start"]')
    assert start.count() >= 3
    page.screenshot(path=output / 'defense-circuit-desktop.png')
    start.first.click()
    room = page.locator('button[data-dc-action="route"]')
    assert room.count() >= 2
    room.first.click()
    encounter = page.locator('.dc-encounter')
    encounter.wait_for()
    page.locator('button[data-dc-action="act"]:not(:disabled)').first.click()
    page.set_viewport_size({'width':390,'height':1000})
    assert page.evaluate('document.documentElement.scrollWidth <= innerWidth')
    page.evaluate('window.scrollTo(0,0)')
    page.screenshot(path=output / 'defense-circuit-mobile.png')
    page.reload()
    assert encounter.is_visible(), 'The current Defense Circuit run should survive reload'
    page.goto(location + '#checklist')
    before = page.locator('#progress-count').inner_text()
    page.locator('.mission-training').get_by_role('link',name='Packet Duel').click()
    page.get_by_role('heading', name='Packet Duel',exact=True).wait_for()
    page.set_viewport_size({'width':1440,'height':1000})
    page.screenshot(path=output / 'packet-duel-desktop.png')
    assert page.locator('button[data-move="faction"]').count() == 2
    page.locator('button[data-move="start"]').click()
    assert page.locator('button[data-move="swap"]').count() >= 3
    page.locator('button[data-move="swap"]').first.click()
    page.locator('button[data-move="deal"]').click()
    page.locator('button[data-move="pass"]').click()
    page.set_viewport_size({'width':390,'height':1000})
    assert page.evaluate('document.documentElement.scrollWidth <= innerWidth')
    page.evaluate('window.scrollTo(0,0)')
    page.screenshot(path=output / 'packet-duel-mobile.png')
    page.goto(location + '#checklist')
    assert page.locator('#progress-count').inner_text() == before
    page.set_viewport_size({'width':1440,'height':1000})
