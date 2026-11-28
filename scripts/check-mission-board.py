#!/usr/bin/env python3
"""Verify the service checklist workflow in the generated offline Hub."""
from pathlib import Path


def check_mission_board(page, html: Path, output: Path):
    location = html.resolve().as_uri() + "#checklist"
    page.goto(location)
    page.evaluate('''() => {
      const name='inta:job-checklists:v1', saved=JSON.parse(localStorage.getItem(name)||'{}');
      saved['client-handoff']={...(saved['client-handoff']||{}),'step:anchor:step-01':true};
      localStorage.setItem(name,JSON.stringify(saved));
    }''')
    page.reload()
    board = page.locator('[data-mission-board]')
    board.get_by_role('heading', name='Technician checklist').wait_for()
    checks = board.get_by_role('checkbox')
    assert checks.count() == 9
    assert board.locator('[data-mission-stage]').count() == 4
    search = board.get_by_role('searchbox', name='Find a checklist step')
    search.fill('sample restored')
    assert checks.count() == 1
    assert checks.first.get_attribute('data-check-title') == 'Protect data and establish the baseline'
    search.fill('firmware relevant')
    assert checks.count() == 1
    assert checks.first.get_attribute('data-check-title') == 'Review drivers and BIOS / UEFI'
    search.fill('billing-no-match')
    assert board.locator('[data-mission-empty]').is_visible()
    assert board.locator('#progress-count').inner_text() == '0 of 9 checks'
    board.get_by_role('button', name='Next unchecked').click()
    assert search.input_value() == ''
    board.get_by_role('button', name='All steps', exact=True).click()

    page.screenshot(path=output / 'technician-checklist-desktop.png')
    board.get_by_role('button', name='Next unchecked').click()
    assert checks.first.evaluate('(c) => c === document.activeElement')
    checks.first.press('Space')
    assert checks.first.is_checked()
    assert board.locator('#progress-percentage').inner_text() == '11%'
    checks.nth(1).press('Space')
    assert board.locator('[data-mission-milestone="protect"]').inner_text().endswith('Verified')
    board.get_by_role('button', name='Remaining', exact=True).click()
    assert checks.count() == 7
    board.get_by_role('button', name='Verified', exact=True).click()
    assert checks.count() == 2
    board.get_by_role('button', name='Next unchecked').click()
    assert checks.count() == 7
    assert checks.first.evaluate('(c) => c === document.activeElement')
    page.reload()
    assert checks.first.is_checked()
    assert checks.nth(1).is_checked()
    assert board.locator('[data-mission-finish]').is_hidden()
    for index in range(2, 9):
        checks.nth(index).press('Space')
    assert board.locator('[data-mission-finish]').is_visible()
    assert board.locator('#progress-percentage').inner_text() == '100%'
    assert board.get_by_role('button', name='Next unchecked').is_disabled()
    with page.expect_download() as downloaded:
        board.get_by_role('button', name='Export checklist', exact=True).click()
    saved = downloaded.value
    assert saved.suggested_filename == 'INTA-technician-checklist.md'
    assert 'Checklist: 9/9 checks' in saved.path().read_text(encoding='utf-8')
    board.get_by_role('button', name='Start a fresh checklist').click()
    assert board.locator('#progress-percentage').inner_text() == '0%'
    assert page.evaluate("JSON.parse(localStorage.getItem('inta:job-checklists:v1'))['client-handoff']['step:anchor:step-01'] === true")
    page.get_by_role('button', name='Undo', exact=True).click()
    assert board.locator('#progress-percentage').inner_text() == '100%'
    board.get_by_role('button', name='Remaining', exact=True).click()
    page.emulate_media(media='print', reduced_motion='reduce')
    assert board.locator('[data-mission-row]').first.is_visible(), 'Printing a filtered list must not omit steps'
    page.emulate_media(media='screen', reduced_motion='reduce')
    board.get_by_role('button', name='All steps', exact=True).click()
    for width in (320,390,800):
        page.set_viewport_size({'width':width,'height':1000})
        assert page.evaluate('document.documentElement.scrollWidth <= innerWidth'), f'Checklist overflows at {width}px'
    page.set_viewport_size({'width':390,'height':1000})
    checks.first.press('Space')
    assert checks.first.evaluate("(c) => getComputedStyle(c.nextElementSibling).animationName === 'none'")
    page.evaluate('window.scrollTo(0,0)')
    page.screenshot(path=output / 'technician-checklist-mobile.png')
    board.get_by_role('button', name='Start a fresh checklist').click()
    page.set_viewport_size({'width':1440,'height':1000})
    page.emulate_media(media='screen', reduced_motion='no-preference')
