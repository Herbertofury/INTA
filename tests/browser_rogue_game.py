"""Exercise the real Defense Circuit browser module, including its saved run."""
import argparse
import tempfile
from pathlib import Path

from playwright.sync_api import sync_playwright


ROOT = Path(__file__).resolve().parents[1]


def exercise(browser, address, screenshot):
    context = browser.new_context(viewport={"width": 320, "height": 760}, reduced_motion="reduce")
    errors = []
    remote = []
    context.route("http**", lambda route: (remote.append(route.request.url), route.abort()))
    page = context.new_page()
    page.on("pageerror", lambda exc: errors.append(str(exc)))
    page.goto(address, wait_until="load")
    page.get_by_role("heading", name="Defense Circuit").wait_for()
    assert page.evaluate("document.documentElement.scrollWidth") <= 320

    def hit(kind, id=None):
        selector = f'[data-dc-action="{kind}"]'
        if id is not None:
            selector += f'[data-dc-id="{id}"]'
        page.locator(".dc-stage " + selector).first.click()

    page.locator('[data-dc-action=start][data-dc-id=analyst]').press("Enter")
    assert page.locator(".dc-stage").evaluate("node => node === document.activeElement")
    hit("route", "service")
    hit("route", "service")
    hit("route", "boss")
    page.get_by_role("heading", name="NotPetya", exact=True).wait_for()
    hit("act", "trace")
    page.reload(wait_until="load")
    page.get_by_text("2 marks", exact=False).wait_for()
    hit("act", "patch")
    hit("end", "end")
    for _ in range(8):
        if page.locator("[data-dc-victory]").count():
            break
        hit("act", "trace")
        if not page.locator("[data-dc-victory]").count():
            hit("act", "patch")
        if not page.locator("[data-dc-victory]").count():
            hit("end", "end")
    field_report = page.locator(".dc-stage [data-dc-report=notpetya]")
    field_report.wait_for()
    assert field_report.get_by_text("Incident lore:", exact=False).is_visible()
    assert field_report.locator("ol > li").count() == 2
    assert "/overview-of-petya-" in field_report.locator("a").get_attribute("href")
    assert page.evaluate("document.documentElement.scrollWidth") <= 320
    if screenshot:
        page.screenshot(path=str(screenshot), full_page=True)
    hit("next")
    assert "Gateway district" in page.locator(".dc-stage").inner_text()
    page.locator(".dc-archive > summary").click()
    assert page.locator(".dc-archive > [data-dc-report=notpetya]").is_visible()
    assert not remote
    assert not errors
    context.close()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--chromium")
    parser.add_argument("--screenshot", type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as d, sync_playwright() as p:
        path = Path(d) / "index.html"
        style = "\n".join((ROOT / "portal" / file).read_text(encoding="utf-8") for file in ("styles.css", "rogue-game.css"))
        script = (ROOT / "portal" / "rogue-game.js").read_text(encoding="utf-8")
        path.write_text(
            '<!doctype html><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">'
            f'<style>{style}</style><main id="test-root"></main><script>{script}</script>'
            '<script>window.INTARogue.mount(document.querySelector("main"))</script>',
            encoding="utf-8",
        )
        browser = p.chromium.launch(executable_path=args.chromium)
        try:
            exercise(browser, path.as_uri(), args.screenshot)
        finally:
            browser.close()
    print("PASS: mobile keyboard start, real boss fight, reload, defensive report, upgrade, archive, zero remote requests.")


if __name__ == "__main__":
    main()
