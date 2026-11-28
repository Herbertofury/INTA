#!/usr/bin/env python3
"""Verify pinned Downloads cards from built INTA Resources, hosted and file://."""
import argparse
import functools
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from threading import Thread

from playwright.sync_api import expect, sync_playwright


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass


def check(page):
    cover = page.get_by_role("region", name="Download INTA")
    expect(cover).to_be_visible()
    expected = {
        "desktop": ("desktop-0.2-preview.1", "Preview", ["INTA-Desktop-x64.msi", "INTA-Desktop-portable-x64.zip"]),
        "mobile": ("mobile-0.2-preview.3", "Preview", ["INTA-Mobile-Android.apk", "verification.txt"]),
        "rescue": ("rescue-recovery-20261128.2", "Stable", ["inta-rescue-recovery-amd64.iso", "INTA-Media-Creator.pyz", "verification.json"]),
    }
    assert cover.locator("article").count() == len(expected)
    assert cover.locator(".product-release-actions .brand-icon").count() == 0
    for product, (tag, channel, assets) in expected.items():
        card = cover.locator(f'[data-product-release="{product}"]')
        expect(card.get_by_text(channel, exact=True)).to_be_visible()
        urls = card.locator("a").evaluate_all("(links) => links.map(a => a.href)")
        root = "https://github.com/Herbertofury/INTA/releases"
        assert {url for url in urls if url.startswith(f"{root}/")} == {
            f"{root}/tag/{tag}",
            *(f"{root}/download/{tag}/{asset}" for asset in [*assets, "SHA256SUMS"]),
        }, (product, urls)
    expect(cover.get_by_text("No antivirus scanner", exact=False)).to_be_visible()
    expect(cover.get_by_text("does not control Desktop or Rescue live", exact=False)).to_be_visible()
    expect(cover.get_by_text("file downloads need a connection", exact=False)).to_be_visible()
    expect(cover.get_by_role("link", name="Connected release status")).to_have_attribute(
        "href", "https://github.com/Herbertofury/INTA/issues/76")
    assert page.evaluate("document.documentElement.scrollWidth <= window.innerWidth + 1")
    assert page.locator(".secondary-nav a").evaluate_all(
        "(links) => links.filter(a => a.textContent.includes('Install INTA Mobile')).map(a => a.href)") == [
            "https://github.com/Herbertofury/INTA/releases/tag/mobile-0.2-preview.3"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, default=Path("site/index.html"))
    parser.add_argument("--browser")
    parser.add_argument("--proof-dir", type=Path)
    args = parser.parse_args()
    html = args.html.resolve()
    handler = functools.partial(QuietHandler, directory=html.parent)
    with ThreadingHTTPServer(("127.0.0.1", 0), handler) as server, sync_playwright() as runtime:
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        browser = runtime.chromium.launch(executable_path=args.browser, args=["--no-sandbox"])
        try:
            urls = [f"http://127.0.0.1:{server.server_port}/{html.name}", html.as_uri()]
            for mode, width, url in [("hosted", 1280, urls[0]), ("offline-file", 390, urls[1])]:
                with browser.new_context(viewport={"width": width, "height": 1000}, reduced_motion="reduce") as context:
                    if mode == "offline-file":
                        context.set_offline(True)
                    page = context.new_page()
                    errors, remote = [], []
                    page.on("pageerror", lambda error: errors.append(str(error)))
                    page.on("request", lambda request: remote.append(request.url) if request.url.startswith("https://") else None)
                    page.goto(f"{url}#resources")
                    check(page)
                    assert not errors, errors
                    if mode == "offline-file":
                        assert not remote, remote
                    if args.proof_dir:
                        args.proof_dir.mkdir(parents=True, exist_ok=True)
                        page.get_by_role("region", name="Download INTA").screenshot(
                            path=args.proof_dir / f"product-releases-{mode}.png")
        finally:
            browser.close()
            server.shutdown()
            thread.join()
    print("PASS the three Resources cards, exact product/file links, scope, badges and no overflow: 1280px hosted and 390px standalone offline.")


if __name__ == "__main__":
    main()
