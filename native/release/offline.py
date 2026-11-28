import os
import re
from pathlib import Path


def offline_guides(out):
    wiki = out / "wiki"
    hub = out / "site/index.html"
    wiki_url = "https://github.com/Herbertofury/INTA/wiki"
    hub_url = "https://herbertofury.github.io/INTA/"
    destinations = re.compile(r"(?<=\]\()(https://[^)\s]+)(?=\))")
    for folder in (wiki, out / "docs/runbooks"):
        for page in folder.glob("*.md"):

            def local_link(match):
                url = match.group(0)
                if url == hub_url or url.startswith(hub_url + "#"):
                    target = hub
                    fragment = url[len(hub_url) :]
                elif url == wiki_url or url.startswith(wiki_url + "/"):
                    slug, separator, anchor = (
                        url[len(wiki_url) :].strip("/").partition("#")
                    )
                    if not re.fullmatch(r"[A-Za-z0-9_-]*", slug):
                        return url
                    target = wiki / ((slug or "Home") + ".md")
                    if not target.is_file():
                        return url
                    fragment = separator + anchor
                elif url.startswith(
                    (
                        "https://github.com/Herbertofury/INTA/raw/main/docs/",
                        "https://github.com/Herbertofury/INTA/blob/main/docs/",
                    )
                ):
                    rel, separator, anchor = url.split("/main/", 1)[1].partition("#")
                    target = (out / rel).resolve()
                    if not target.is_relative_to(out.resolve()) or not target.is_file():
                        return url
                    fragment = separator + anchor
                else:
                    return url
                return Path(os.path.relpath(target, page.parent)).as_posix() + fragment

            content = destinations.sub(local_link, page.read_text(encoding="utf8"))
            page.write_text(content, encoding="utf8", newline="\n")
