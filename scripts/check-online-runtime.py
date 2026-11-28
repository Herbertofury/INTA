#!/usr/bin/env python3
"""Exercise the built online directory in Chromium, including offline use.

Requires the optional Playwright Python package and Chromium. This script reads
the actual HTML artifact; it never writes generated source or submits data to
an external service. Use --baseline for an equivalent-result DOM comparison.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from statistics import median
from urllib.parse import parse_qs, urlsplit

from playwright.sync_api import sync_playwright


def check(value, message):
    if not value:
        raise AssertionError(message)


def ids(page):
    return page.locator('[data-online-tool]').evaluate_all(
        '(rows) => rows.map(row => row.dataset.onlineTool)')


def navigate(page, fragment):
    page.evaluate('(fragment) => { location.hash = fragment; }', fragment)
    page.wait_for_function('''(fragment) => {
      if (location.hash !== fragment || !document.getElementById('online-search')) return false;
      const tool = new URLSearchParams(fragment.split('?')[1] || '').get('tool');
      return !tool || document.activeElement.id === 'online-heading-' + tool;
    }''', arg=fragment)


def query(page):
    return parse_qs(urlsplit(page.url).fragment.partition('?')[2])


def choose_task(page, identity):
    page.locator(f'[data-online-task="{identity}"]').click()
    page.wait_for_function('''(identity) => document.querySelector('[data-online-workflow]')
      ?.dataset.onlineWorkflow === identity''', arg=identity)


def benchmark(page, path):
    page.goto(path.resolve().as_uri() + '#online')
    page.wait_for_selector('[data-online-tool]')
    catalog = page.evaluate('''({tools: window.INTA_DATA.tools,
      toolAccess: window.INTA_DATA.toolAccess, onlineTools: window.INTA_DATA.onlineTools,
      brands: window.INTA_DATA.brands})''')
    result = page.evaluate('''() => {
      const input = document.getElementById('online-search');
      const list = document.getElementById('online-results');
      const expected = [...list.querySelectorAll('[data-online-tool]')].map(row => row.dataset.onlineTool);
      const runs = [];
      for (let sample = 0; sample < 3; sample++) {
        const observer = new MutationObserver(() => {});
        observer.observe(list, {childList: true});
        const start = performance.now();
        for (let index = 0; index < 12; index++) {
          input.value = ' '.repeat(index % 3);
          input.dispatchEvent(new Event('input', {bubbles: true}));
        }
        const elapsedMs = performance.now() - start;
        const records = observer.takeRecords(); observer.disconnect();
        const result = [...list.querySelectorAll('[data-online-tool]')].map(row => row.dataset.onlineTool);
        if (JSON.stringify(result) !== JSON.stringify(expected)) throw new Error('Benchmark lost result coverage');
        runs.push({elapsedMs, replacements: records.length, removedRows: records.reduce((total, record) => total + record.removedNodes.length, 0)});
      }
      return {tools: expected.length, eventsPerSample: 12, runs};
    }''')
    result['catalogSha256'] = hashlib.sha256(
        json.dumps(catalog, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--html', type=Path, default=Path('site/index.html'))
    parser.add_argument('--baseline', type=Path)
    parser.add_argument('--browser', help='Explicit Chromium executable; defaults to the Playwright installation')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    report = {'artifact': str(args.html.resolve()),
              'sha256': hashlib.sha256(args.html.read_bytes()).hexdigest(), 'checks': []}
    errors, requests = [], []
    with sync_playwright() as runtime:
        browser = runtime.chromium.launch(executable_path=args.browser, headless=True,
                                          args=['--no-sandbox'])
        context = browser.new_context(viewport={'width': 1440, 'height': 1000})
        context.set_offline(True)
        page = context.new_page()
        page.on('pageerror', lambda error: errors.append(str(error)))
        page.on('request', lambda request: requests.append(request.url)
                if request.url.startswith(('https://', 'http://')) else None)
        page.goto(args.html.resolve().as_uri() + '#home')
        home_mark = page.locator('#app .home-heritage-mark')
        nav_mark = page.locator('.brand > .brand-mark')
        home_mark.wait_for()
        check(home_mark.get_attribute('src') != nav_mark.get_attribute('src'),
              'Front-page Heritage and navigation Application must be different')
        check(page.locator('[data-brand-icon="app-inta"]').count() > 0, 'Missing INTA inline logo')
        check(page.locator('[data-inta-brand-mark]').evaluate_all(
            '(icons) => icons.every(i => getComputedStyle(i).backgroundColor === "rgba(0, 0, 0, 0)")'),
            'INTA inline logo has a visible backing')
        for width in (320, 1440):
            page.set_viewport_size({'width': width, 'height': 1000})
            check(home_mark.is_visible(), 'Front-page Heritage not visible')
            check(page.evaluate('document.documentElement.scrollWidth <= innerWidth'),
                  'Brand layout overflows viewport')
        report['checks'].append('Application chrome, transparent inline marks and front-page-only Heritage')
        expected_home = page.evaluate("window.INTA_DATA.quickStart.items.map(x => x.id).sort()")
        actual_home = page.locator('#app [data-quick-start]').evaluate_all(
            '(rows) => rows.map(row => row.dataset.quickStart).sort()')
        check(actual_home == expected_home, 'Missing or repeated Home repair scenario')
        categories = page.evaluate("[...new Set(window.INTA_DATA.tools.map(x => x.category))].sort()")
        check(page.locator('[data-home-category]').evaluate_all(
            '(tiles) => tiles.map(tile => tile.dataset.homeCategory).sort()') == categories,
            'Home category cards do not match the complete source')
        page.screenshot(path=args.output / 'technician-home-desktop.png')
        panel = page.locator('[data-quick-start="suspected-malware"]')
        check(panel.locator('details[open]').count() == 0, 'Home should stay compact at initial load')
        panel.locator('summary').press('Enter')
        check(panel.locator('.quick-first-step').is_visible(), 'First steps not visible after keyboard activation')
        check(panel.locator('.acquisition-link').first.is_visible(), 'Official acquisition hidden after opening')
        check(panel.locator('a.home-case-link').get_attribute('href') == '#job/malware-removal/quick/suspected-malware',
              'Home scenario did not preserve direct job link')
        page.set_viewport_size({'width': 390, 'height': 1000})
        page.get_by_role('navigation', name='Jump to a Home section').get_by_role('link', name='Find a tool').click()
        page.wait_for_url('**#directory-heading')
        page.wait_for_function("document.activeElement.id === 'directory-heading'")
        check(panel.locator('details[open]').count() == 1, 'Home jump closed the open repair row')
        page.go_back()
        page.wait_for_url('**#home')
        check(panel.locator('details[open]').count() == 1, 'Back to Home closed the open repair row')
        panel.locator('summary').press('Enter')
        page.screenshot(path=args.output / 'technician-home-mobile.png')
        page.get_by_role('navigation', name='Jump to a Home section').get_by_role('link', name='Find a tool').click()
        page.wait_for_function("document.activeElement.id === 'directory-heading'")
        page.locator('[data-home-category="storage"]').click()
        page.locator('#category-filter').wait_for(state='attached')
        check(page.locator('#category-filter').input_value() == 'storage', 'Home tile did not select its category')
        page.set_viewport_size({'width': 1440, 'height': 1000})
        report['checks'].append('All Home topics and tool categories, keyboard details and real filtered library route')


        page.goto(args.html.resolve().as_uri() + '#tools?cli=partial')
        check(home_mark.count() == 0, 'Heritage leaked into tool workspace')
        cli_ids = page.evaluate("Object.entries(window.INTA_DATA.toolCliCoverage).filter(([,x])=>x.status==='partial').map(([id])=>id)")
        page.locator('#tool-results [data-tool-id=powercfg]').wait_for()
        check(set(page.locator('#tool-results [data-tool-id]').evaluate_all(
            '(rows)=>rows.map(row=>row.dataset.toolId)')) == set(cli_ids), 'CLI filter includes a tool outside its evidence state')
        check(page.locator('#cli-filter').input_value() == 'partial', 'CLI deep link did not restore the filter')
        page.locator('#tool-results [data-tool-id=powercfg] [data-library-detail]').click()
        evidence = page.get_by_role('region', name='Command-line control')
        evidence.get_by_text('Partial CLI evidence', exact=True).wait_for()
        check(evidence.get_by_text('power plans', exact=True).count() == 1, 'Reviewed operation missing')
        check(evidence.get_by_role('link', name='Runtime evidence').count() == 1, 'Published runtime evidence missing')
        check(evidence.get_by_role('button').count() == 0, 'Partial or preview evidence exposes an action')
        page.goto(args.html.resolve().as_uri() + '#tool/defender-offline')
        evidence.get_by_text('CLI not verified', exact=True).wait_for()
        check(evidence.get_by_role('link', name='Runtime evidence').count() == 0, 'Unknown tool claimed proof')
        report['checks'].append('CLI coverage deep link, partial/unknown details and no action')
        page.goto(args.html.resolve().as_uri() + '#online')
        page.wait_for_selector('[data-online-tool]')
        source = page.evaluate('window.INTA_DATA.onlineTools')
        expected_ids = [entry['toolId'] for entry in source['tools']]
        check(ids(page) == expected_ids, 'Default view must contain every online tool in source order')
        report['checks'].append(f"Complete default directory: {len(expected_ids)} tools")

        # Group selection retains every matching record and accurate counts.
        for group in source['groups']:
            page.locator('#online-group').select_option(group['id'])
            check(ids(page) == [entry['toolId'] for entry in source['tools']
                                if entry['group'] == group['id']], group['id'])
        page.locator('[data-reset-online]').first.click()
        check(ids(page) == expected_ids, 'Clear filters restores every record')

        # Query/group survive task navigation and an actual browser Back.
        page.locator('#online-search').fill('hash')
        page.locator('#online-group').select_option(source['groups'][0]['id'])
        target_task = source['workflows'][1]['id']
        choose_task(page, target_task)
        check(query(page).get('q') == ['hash'], 'Task navigation lost query')
        check(query(page).get('group') == [source['groups'][0]['id']], 'Task navigation lost group')
        prior_url = page.url
        first_tool = ids(page)[0]
        page.locator(f'[data-online-tool="{first_tool}"] a[href="#tool/{first_tool}"]').click()
        page.wait_for_function('''(identity) => location.hash === '#tool/' + identity
          && !document.getElementById('online-search')''', arg=first_tool)
        page.go_back()
        page.wait_for_selector('#online-search')
        check(page.url == prior_url and page.locator('#online-search').input_value() == 'hash',
              'Back did not restore filtered directory')
        report['checks'].append('Task navigation, browser Back and URL filters retain state')
        page.locator('[data-reset-online]').first.click()

        # Each authored task is an exact set, not a positional featured cap.
        page.locator('#online-task-only').focus()
        page.keyboard.press('Space')
        check(page.locator('#online-task-only').is_checked(), 'Task filter is not keyboard operable')
        for workflow in source['workflows']:
            choose_task(page, workflow['id'])
            check(set(ids(page)) == set(workflow['toolIds']), workflow['id'] + ' task set')
            check(query(page).get('scope') == ['task'], 'Task filter was lost')
            check(page.locator('[data-online-inspect]').count() == len(workflow['toolIds']),
                  'Each task tool needs an access/sharing route')
        report['checks'].append(f"All {len(source['workflows'])} task scopes and sharing shortcuts")
        page.locator('#online-search').fill('no-service-with-this-name-91874')
        check(not ids(page), 'Impossible query must not show unrelated tools')
        page.locator('[data-reset-online]').first.click()
        check(ids(page) == expected_ids and not page.locator('#online-task-only').is_checked(),
              'Clear must reset every filter')

        # Favorites use the existing store, including reload and removing the last row.
        favorite = expected_ids[0]
        page.locator(f'[data-online-tool="{favorite}"] [data-save]').click()
        page.locator('#online-saved-only').check()
        check(ids(page) == [favorite], 'Saved filter must use existing favorites')
        page.reload()
        page.wait_for_selector('[data-online-tool]')
        check(ids(page) == [favorite] and page.locator('#online-saved-only').is_checked(),
              'Saved filter and favorites must survive reload')
        page.locator(f'[data-online-tool="{favorite}"] [data-save]').click()
        check(not ids(page), 'Unsave must update the saved-only view immediately')
        check(page.evaluate('document.activeElement.id') == 'online-saved-only',
              'Removing last saved row must leave keyboard focus on a live control')
        report['checks'].append('Saved-only filtering, persistence, removal and focus recovery')
        page.locator('[data-reset-online]').first.click()

        # No result mutation for equivalent queries; open evidence stays open.
        page.locator('#online-search').fill('virustotal')
        page.locator('[data-online-tool="virustotal"] details').evaluate('(element) => { element.open = true; }')
        page.evaluate('window.__onlineRetainedRow = document.querySelector("[data-online-tool=virustotal]")')
        page.locator('#online-search').fill('VIRUSTOTAL ')
        check(page.evaluate('window.__onlineRetainedRow === document.querySelector("[data-online-tool=virustotal]")'),
              'Equivalent query rebuilt the same result')
        check(page.locator('[data-online-tool="virustotal"] details').evaluate('(element) => element.open'),
              'Equivalent query collapsed evidence')
        report['checks'].append('Equivalent searches preserve row identity and expanded evidence')

        # Every source record has a stable address and focuses its own heading.
        for identity in expected_ids:
            navigate(page, '#online?tool=' + identity)
            check(page.locator(f'#online-service-{identity}').count() == 1, identity + ' deep link')
            check(page.evaluate('document.activeElement.id') == 'online-heading-' + identity,
                  identity + ' deep link focus')
            check(ids(page) == expected_ids, 'A service deep link lost full-directory coverage')
        page.reload()
        page.wait_for_selector('[data-online-tool]')
        check(page.evaluate('document.activeElement.id') == 'online-heading-' + expected_ids[-1],
              'Deep-link reload did not focus its service')
        report['checks'].append(f"All {len(expected_ids)} service deep links, full coverage and reload")

        # Mobile, light/dark and literal offline-file routes, without external requests.
        for width in (1440, 800, 390, 320):
            page.set_viewport_size({'width': width, 'height': 900})
            navigate(page, '#online?tool=virustotal')
            for theme in ('dark', 'light'):
                page.evaluate('(theme) => { document.documentElement.dataset.theme = theme; }', theme)
                check(page.evaluate('document.documentElement.scrollWidth <= innerWidth + 1'),
                      f'{width}px {theme} document overflow')
                check(ids(page) == expected_ids, f'{width}px lost directory content')
            page.screenshot(path=str(args.output / f'online-{width}.png'))
        report['checks'].append('1440/800/390/320px, dark/light, offline file and complete content')
        report['benchmark'] = {'candidate': benchmark(page, args.html)}
        if args.baseline:
            report['benchmark']['baseline'] = benchmark(page, args.baseline)
            check(report['benchmark']['baseline']['catalogSha256']
                  == report['benchmark']['candidate']['catalogSha256'],
                  'Performance comparison requires identical complete catalog, access, evidence and icon data')
            report['benchmark']['medianMs'] = {name: median(run['elapsedMs'] for run in result['runs'])
                                               for name, result in report['benchmark'].items()
                                               if name in ('baseline', 'candidate')}
        check(not errors, 'JavaScript errors: ' + repr(errors))
        check(not requests, 'Unexpected runtime network requests: ' + repr(requests))
        report['javascriptErrors'] = errors
        report['externalRequests'] = requests
        report['browser'] = browser.version
        browser.close()
    report['status'] = 'passed'
    (args.output / 'online-runtime.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
