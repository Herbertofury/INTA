"""Regression against built static output. Needs sandboxed Chromium + Playwright.

python tests/browser_windows_platform_links.py --chromium /path/to/chromium [--screenshots /path]
The fixture never opens a vendor page; compare exact hrefs to canonical inputs.
"""
import argparse
import json
from pathlib import Path
import re

from playwright.sync_api import expect, sync_playwright

ROOT = Path(__file__).resolve().parents[1]
PLATFORM_IDS = ("app-windows", "app-windows10", "app-windows11", "app-windows-server")
ORIGIN = "http://127.0.0.1:8141/"
GENERIC_IMAGES = ",".join(f'[data-brand-icon="{brand}"]' for brand in PLATFORM_IDS)
GENERIC_ICON_LINKS = ",".join(f'.brand-icon-link[data-brand-link="{brand}"]' for brand in PLATFORM_IDS)


def context(browser, html, width):
    browser_context = browser.new_context(viewport={"width": width, "height": 960},
                                          reduced_motion="reduce", service_workers="block")
    browser_context.route("**/*", lambda route: route.fulfill(status=200, content_type="text/html", body=html)
                          if route.request.url == ORIGIN else route.abort())
    return browser_context


def check(html, browser, width, screenshots=None):
    ctx = context(browser, html, width)
    page = ctx.new_page()
    page_errors = []
    page.on("pageerror", lambda error: page_errors.append(str(error)))
    page.goto(ORIGIN)
    expect(page.get_by_role("heading", name="Technician home", exact=True)).to_be_visible()
    picks = page.locator('[data-quick-start="suspected-malware"]')
    picks.locator('summary').press('Enter')
    platform = page.locator('main a[data-link-name="Windows"][href="https://www.microsoft.com/windows"]').first
    expect(platform).to_have_text("Windows")
    expect(page.locator(GENERIC_IMAGES)).to_have_count(0)
    expect(page.locator(GENERIC_ICON_LINKS)).to_have_count(0)
    expect(platform.locator("img")).to_have_count(0)
    assert platform.evaluate("a => a.href") == "https://www.microsoft.com/windows"
    assert platform.evaluate("a => a.tabIndex") == 0
    expect(picks.locator('[data-brand-icon="malwarebytes"]').first).to_be_visible()
    if screenshots:
        picks.scroll_into_view_if_needed()
        page.screenshot(path=screenshots / f"inta-platform-links-{width}.png")

    page.goto(ORIGIN + "#tools?q=windows+terminal")
    card = page.locator('#tool-results [data-tool-card="windows-terminal"]')
    expect(card.locator('[data-brand-icon="windows-terminal"]')).to_be_visible()
    access = json.loads((ROOT / "data/tool-access.json").read_text())["tools"]["windows-terminal"]
    expect(card.locator('.official-page')).to_have_attribute("href", access["officialUrl"])
    expect(card.locator('.primary-acquisition')).to_have_attribute("href", access["primary"]["url"])
    page.get_by_role("button", name="Gallery", exact=True).press("Enter")
    expect(card.locator('[data-brand-icon="windows-terminal"]')).to_be_visible()
    expect(page.locator(GENERIC_IMAGES)).to_have_count(0)

    page.goto(ORIGIN + "#updates")
    expect(page.locator('main [data-brand-icon="app-windows-update"]').first).to_be_visible()
    expect(page.locator(GENERIC_IMAGES)).to_have_count(0)
    page.goto(ORIGIN + "#job/windows-repair")
    expect(page.get_by_role("button", name="Export checklist", exact=True)).to_be_visible()
    expect(page.locator(GENERIC_IMAGES)).to_have_count(0)
    with page.expect_download() as pending:
        page.get_by_role("button", name="Export checklist", exact=True).press("Enter")
    markdown = pending.value.path().read_text()
    assert re.search(r"\[Windows\]\(https://www\.microsoft\.com/windows\)", markdown)
    for brand in json.loads((ROOT / "data/brand-icons.json").read_text())["brands"]:
        if brand["id"] in PLATFORM_IDS:
            assert not re.search(r"\[<img[^\]\n]*\]\(" + re.escape(brand["downloadPage"]) + r"\)", markdown)
    assert "<img " in markdown, "non-platform images were removed from the export"
    assert not page_errors, page_errors
    ctx.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--chromium")
    parser.add_argument("--screenshots", type=Path)
    args = parser.parse_args()
    if args.screenshots:
        args.screenshots.mkdir(parents=True, exist_ok=True)
    with sync_playwright() as runner:
        browser = runner.chromium.launch(executable_path=args.chromium, chromium_sandbox=True)
        html = (ROOT / "site/index.html").read_text()
        for width in (390, 1440):
            check(html, browser, width, args.screenshots)
        print(f"PASS Chromium {browser.version}: Home, filtered tool list/gallery, Updates, playbook and exported Markdown at 390/1440.")
        browser.close()


if __name__ == "__main__":
    main()
