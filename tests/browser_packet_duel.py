"""Exercise the generated offline INTA Portal #packet-duel entry point in Chromium."""
import argparse
import json
from pathlib import Path

from playwright.sync_api import sync_playwright


def check(browser, html, width):
    context = browser.new_context(viewport={"width": width, "height": 900})
    page = context.new_page()
    failures, remote = [], []
    page.on("pageerror", lambda error: failures.append(str(error)))
    page.on("request", lambda req: remote.append(req.url) if req.url.startswith(("https://", "http://")) else None)
    page.add_init_script("Object.defineProperty(Math, 'random', {value:()=>0.141}); Date.now=()=>1717171717;")
    page.goto(html.as_uri() + "#packet-duel")
    duel = page.locator("[data-inta-card-game]")
    duel.get_by_role("heading", name="Packet Duel").wait_for()
    assert page.evaluate("document.documentElement.scrollWidth <= innerWidth")
    duel.get_by_text("How to play & card collection").click()
    assert duel.get_by_role("link", name="CrystalDiskInfo", exact=True).get_attribute("href") == "#tool/crystaldiskinfo"
    assert duel.get_by_role("link", name="4.8/5 SourceForge · 20 reviews · +1 health ↗").get_attribute("href").startswith("https://sourceforge.net/")
    assert duel.locator(".inta-duel-collection img").count() == 20
    duel.get_by_text("Real-world case files").click()
    assert duel.get_by_role("heading", name="WannaCry").is_visible()
    assert duel.get_by_role("heading", name="Log4Shell").is_visible()
    duel.get_by_role("button", name="Phantom Circuit", exact=False).click()
    duel.get_by_role("button", name="Scout", exact=False).click()
    duel.get_by_role("button", name="Open the duel", exact=False).press("Enter")
    assert duel.get_by_role("heading", name="Keep them or take a chance.").evaluate("el => el===document.activeElement")
    first = duel.locator('[data-move="swap"]').first
    first.press("Enter")
    assert first.get_attribute("aria-pressed") == "true"
    first.press("Enter")
    duel.get_by_role("button", name="Keep hand & start").press("Enter")
    assert duel.get_by_role("status").inner_text().startswith("Your turn")
    assert page.evaluate("document.documentElement.scrollWidth <= innerWidth")
    # Play all affordable cards and ready units; choose only targets offered by the actual UI.
    for _ in range(40):
        end = duel.locator(".inta-duel-final")
        if end.count():
            break
        for _ in range(30):
            play = duel.locator('button[data-move="play"]:enabled')
            attack = duel.locator('button[data-move="select-attack"]:enabled')
            if play.count():
                play.first.click()
            elif attack.count():
                attack.first.click()
            elif duel.locator('button[data-move="power"]:enabled').count():
                duel.locator('button[data-move="power"]:enabled').click()
            else:
                break
            target = duel.locator("button[data-target]:enabled")
            if target.count():
                target.first.click()
            if end.count():
                break
        if end.count():
            break
        duel.get_by_role("button", name="End turn", exact=False).click()
    assert end.count()
    result = end.get_by_role("heading").inner_text()
    stored = page.evaluate("JSON.parse(localStorage.getItem('inta.packet-duel.record.v1'))")
    assert stored["wins"] + stored["losses"] + stored["draws"] == 1
    duel.get_by_role("button", name="Replay this deal").click()
    assert duel.locator('[data-move="swap"]').count() == 3
    assert failures == [], failures
    assert remote == [], remote
    context.close()
    return {"width": width, "result": result, "offline_requests": len(remote)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, default=Path("site/index.html"))
    parser.add_argument("--browser")
    args = parser.parse_args()
    with sync_playwright() as runner:
        browser = runner.chromium.launch(executable_path=args.browser, args=["--no-sandbox"])
        print(json.dumps([check(browser, args.html.resolve(), w) for w in (320, 1440)]))
        browser.close()


if __name__ == "__main__":
    main()
