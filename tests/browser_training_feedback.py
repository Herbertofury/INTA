#!/usr/bin/env python3
"""Check genuine gain output, opt-in and muting on the built offline Hub."""
import argparse
from pathlib import Path
from playwright.sync_api import sync_playwright

PROBE = """() => {
  const Native = window.AudioContext || window.webkitAudioContext;
  window.__toneContexts = 0;
  window.__toneAnalyser = null;
  class Monitored extends Native {
    constructor() { super(); window.__toneContexts++; }
    createGain() {
      const node = super.createGain();
      if (!window.__toneAnalyser) {
        window.__toneAnalyser = this.createAnalyser();
        node.connect(window.__toneAnalyser);
        window.__toneData = new Float32Array(window.__toneAnalyser.fftSize);
      }
      return node;
    }
  }
  window.AudioContext = Monitored;
}"""

def peak(page):
    return page.evaluate("""() => {
      if (!window.__toneAnalyser) return 0;
      window.__toneAnalyser.getFloatTimeDomainData(window.__toneData);
      return Math.max(...window.__toneData.map(Math.abs));
    }""")


def check(browser, html):
    context = browser.new_context(viewport={"width": 390, "height": 1000})
    context.set_offline(True)
    page = context.new_page()
    errors, remote = [], []
    page.on('pageerror', lambda e: errors.append(str(e)))
    page.on('request', lambda req: remote.append(req.url) if req.url.startswith(('http://','https://')) else None)
    page.add_init_script('(' + PROBE + ')()')
    page.goto(html.as_uri()+'#checklist')
    settings = page.locator('.training-audio')
    settings.locator('summary').press('Enter')
    enable = settings.get_by_role('checkbox', name='Enable sounds')
    preview = settings.get_by_role('button', name='Preview', exact=True)
    assert preview.is_disabled() and page.evaluate('window.__toneContexts') == 0
    enable.press('Space')
    assert preview.is_enabled() and page.evaluate('window.__toneContexts') == 0
    preview.press('Enter')
    page.wait_for_function('''() => {
      if (!window.__toneAnalyser) return false;
      window.__toneAnalyser.getFloatTimeDomainData(window.__toneData);
      return Math.max(...window.__toneData.map(Math.abs)) > .003;
    }''')
    enable.press('Space')
    page.wait_for_function('''() => {
      window.__toneAnalyser.getFloatTimeDomainData(window.__toneData);
      return Math.max(...window.__toneData.map(Math.abs)) < .0001;
    }''')
    assert preview.is_disabled()
    enable.press('Space')
    page.reload()
    settings.locator('summary').press('Enter')
    assert enable.is_checked()
    assert page.evaluate('window.__toneContexts') == 0
    step = page.locator('[data-mission-row] input').first
    step.press('Space')
    page.wait_for_function('window.__toneContexts === 1')
    assert step.is_checked()
    page.goto(html.as_uri()+'#packet-duel')
    page.get_by_role('heading', name='Packet Duel').wait_for()
    page.goto(html.as_uri()+'#defense-circuit')
    page.get_by_role('heading', name='Defense Circuit').wait_for()
    assert page.evaluate('window.__toneContexts') == 1
    assert not errors, errors
    assert not remote, remote
    assert peak(page) < .01
    context.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--html', type=Path, default=Path('site/index.html'))
    parser.add_argument('--browser')
    opts = parser.parse_args()
    with sync_playwright() as runtime:
        browser = runtime.chromium.launch(executable_path=opts.browser, args=['--no-sandbox'])
        check(browser, opts.html.resolve())
        browser.close()
    print('PASS actual Web Audio signal; no autoplay; reload, mute, one context, and zero remote requests.')


if __name__ == '__main__':
    main()
