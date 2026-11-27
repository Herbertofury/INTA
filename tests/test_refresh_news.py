import datetime as dt
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from urllib.request import Request

script = Path(__file__).resolve().parents[1] / "scripts" / "refresh-news.py"
spec = importlib.util.spec_from_file_location("refresh_news", script)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

source = m.Source("primary", "https://publisher.example/feed", frozenset({"publisher.example"}),
                  ("/news/",), frozenset({"media.example"}))
broken = m.Source("broken", "https://broken.example/feed", frozenset({"broken.example"}))
RSS = b"""<rss version="2.0" xmlns:media="http://search.yahoo.com/mrss/"><channel>
<item><title><![CDATA[Security &amp; <em>Updates</em><script>no</script>]]></title>
<link>https://publisher.example/news/one?utm_source=test&amp;keep=1</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate>
<description>unwanted private/full text</description>
<media:thumbnail url="https://media.example/card.jpg"/>
<enclosure url="https://publisher.example/news/audio.mp3" type="audio/mpeg"/>
<media:player url="https://www.youtube.com/embed/zFwAT04IvGA"/>
<media:content url="https://evil.example/x.png" medium="image" type="image/png"/>
</item>
<item><title>Duplicate</title><link>https://publisher.example/news/one?keep=1</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Meta fallback</title><link>https://publisher.example/news/two</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Not a news endpoint</title><link>https://publisher.example/account</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Other host</title><link>https://publisher.example@evil.example/news/one</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
</channel></rss>"""
ATOM = b"""<feed xmlns="http://www.w3.org/2005/Atom">
<entry><title>Atom headline</title><link rel="alternate" href="/news/atom" />
<link rel="enclosure" href="https://media.example/content.mp4" type="video/mp4" />
<published>2026-11-26T23:25:00-04:00</published><content>body not syndicated</content></entry>
</feed>"""

class NewsRefresh(unittest.TestCase):
    def test_rss_strips_non_metadata_and_dedupes(self):
        rows = m.parse_feed(source, RSS, source.feed)
        self.assertEqual(len(rows), 3)
        item = rows[0]
        self.assertEqual(item["title"], "Security & Updates")
        self.assertEqual(item["url"], "https://publisher.example/news/one?keep=1")
        self.assertEqual({i["kind"] for i in item["media"]}, {"image","audio","embed"})
        self.assertEqual(item["media"][-1]["url"], "https://www.youtube.com/watch?v=zFwAT04IvGA")
        self.assertNotIn("unwanted", json.dumps(rows))
        self.assertNotIn("evil.example", json.dumps(rows))

    def test_atom_enclosures_dates_and_dtd(self):
        [item] = m.parse_feed(source, ATOM, source.feed)
        self.assertEqual(item["date"], "2026-11-27")
        self.assertEqual(item["media"], [{"kind":"video","url":"https://media.example/content.mp4"}])
        with self.assertRaises(m.Unsafe):
            m.parse_feed(source, b'<!DOCTYPE rss [<!ENTITY a "xx">]><rss/>', source.feed)
        with self.assertRaises(m.Unsafe):
            m.parse_feed(source, b'<rss/>' * m.FEED_BYTES, source.feed)

    def test_promotions_are_not_tech_news_and_unsupported_embeds_are_filtered(self):
        from xml.etree.ElementTree import fromstring
        post = fromstring("<item><category>Deals</category></item>")
        ordinary = fromstring("<item><category>Security</category></item>")
        publisher = source._replace(id="bleeping")
        self.assertTrue(m.promotional(publisher,post,"https://publisher.example/news/one","Shopping"))
        self.assertTrue(m.promotional(publisher,ordinary,"https://publisher.example/news/deals/example","Sale"))
        self.assertTrue(m.promotional(publisher,ordinary,"https://publisher.example/news/one","Black Friday discount"))
        self.assertFalse(m.promotional(publisher,ordinary,"https://publisher.example/news/one","Black Friday shoppers targeted by malware"))
        self.assertEqual(m.pick_media(source,[("embed","https://vimeo.com/123456","")],source.feed),[])
        sans = next(x for x in m.SOURCES if x.id == "sans-isc")
        result = m.pick_media(sans,[("audio","https://traffic.libsyn.com/securitypodcast/10194.mp3","audio/mpeg"),
                                    ("audio","https://traffic.libsyn.com/anotherpodcast/10194.mp3","audio/mpeg")],sans.feed)
        self.assertEqual(result,[{"kind":"audio","url":"https://traffic.libsyn.com/securitypodcast/10194.mp3"}])

    def test_redirects_cannot_escape_approved_sources(self):
        request = Request(source.feed)
        handler = m.GuardRedirect(source.hosts)
        for url in ("https://evil.example/raw", "http://publisher.example/raw", "https://publisher.example@evil.example/raw",
                    "https://publisher.example:8080/raw", "https://publisher.example/news/%2e%2e/compose"):
            with self.subTest(url=url), self.assertRaises(m.Unsafe):
                handler.redirect_request(request, None, 302, "Found", {}, url)
        self.assertEqual(handler.redirect_request(request,None,302,"Found",{},"https://publisher.example/atom").full_url,
                         "https://publisher.example/atom")

    def test_page_metadata_is_head_only_and_bounded(self):
        calls = []
        def get(url, hosts, limit, deadline, page=False):
            calls.append((url, limit, page))
            if not page:
                return RSS,url
            return b"""<html><head>
<meta property="og:video" content="https://media.example/demo.mp4">
<meta property="og:image" content="https://evil.example/ad.png">
<meta name="twitter:image" content="https://media.example/fake.svg">
</head><body><meta property="og:image" content="https://media.example/not-head.png"></body></html>""", url
        _,status,items = m.refresh_source(source, page_media=True,fetcher=get)
        self.assertEqual(status,"ok")
        self.assertEqual(calls,[(source.feed,m.FEED_BYTES,False),("https://publisher.example/news/two",m.PAGE_BYTES,True)])
        self.assertEqual(items[1]["media"],[{"kind":"video","url":"https://media.example/demo.mp4"}])
        self.assertEqual(items[2]["media"],[])

    def test_output_contract_retention_and_atomic_all_failed(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "digest.json"
            old = dict(schemaVersion=1,items=[{"sourceId":"broken","title":"Preserved", "url":"https://broken.example/item",
                                              "date":"2026-11-25","media":[{"kind":[],"url":"https://broken.example/image.png"}]}])
            path.write_text(json.dumps(old))
            def get(url,*unused,**ignored):
                if url == broken.feed:
                    raise OSError("publisher down")
                return RSS,source.feed
            now = dt.datetime(2026,11,27,3,0,tzinfo=dt.timezone.utc)
            obj = m.refresh(path,sources=(source,broken),fetcher=get,now=now)
            self.assertEqual(obj["schemaVersion"],1)
            self.assertEqual(obj["generatedAt"],"2026-11-27T03:00:00Z")
            self.assertEqual(obj["sources"],[{"id":"primary","status":"ok"},{"id":"broken","status":"unavailable"}])
            self.assertEqual(obj["items"][-1]["title"],"Preserved")
            self.assertEqual(obj["items"][-1]["media"],[])
            before = path.read_bytes()
            with self.assertRaisesRegex(m.Unsafe,"previous output left unchanged"):
                m.refresh(path,sources=(broken,),fetcher=get)
            self.assertEqual(before,path.read_bytes())

    def test_editorial_seed_is_approved_normalized_and_optional(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "seed.json"
            item = {"sourceId":"windows-health","title":"Windows video",
                    "url":"https://learn.microsoft.com/en-us/windows/release-health/", "date":"2026-09-29",
                    "media":[{"kind":"embed","url":"https://www.youtube.com/watch?v=zFwAT04IvGA"}]}
            path.write_text(json.dumps({"items":[item]}))
            [row] = m.load_seed(path)
            self.assertEqual(row[:2],("windows-health","ok"))
            self.assertEqual(row[2][0]["media"],item["media"])
            item["url"] = "https://attacker.example/item"
            path.write_text(json.dumps([item]))
            with self.assertRaises(m.Unsafe):
                m.load_seed(path)

if __name__ == "__main__":
    unittest.main()
