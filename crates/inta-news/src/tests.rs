use super::*;
use std::sync::Mutex;
fn publisher() -> Source {
    source(
        "primary",
        "https://publisher.example/feed",
        &["publisher.example"],
        &["/news/"],
        &["media.example"],
        FEED_BYTES,
    )
}
fn broken() -> Source {
    source(
        "broken",
        "https://broken.example/feed",
        &["broken.example"],
        &[],
        &[],
        FEED_BYTES,
    )
}
const RSS:&[u8]=br#"<rss version="2.0" xmlns:media="http://search.yahoo.com/mrss/"><channel>
<item><title><![CDATA[Security &amp; <em>Updates</em><script>no</script>]]></title>
<link>https://publisher.example/news/one?utm_source=test&amp;keep=1</link>
<pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate><description>unwanted private/full text</description>
<media:thumbnail url="https://media.example/card.jpg"/><enclosure url="https://publisher.example/news/audio.mp3" type="audio/mpeg"/>
<media:player url="https://www.youtube.com/embed/zFwAT04IvGA"/><media:content url="https://evil.example/x.png" medium="image" type="image/png"/>
</item><item><title>Duplicate</title><link>https://publisher.example/news/one?keep=1</link><pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Meta fallback</title><link>https://publisher.example/news/two</link><pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Not a news endpoint</title><link>https://publisher.example/account</link><pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
<item><title>Other host</title><link>https://publisher.example@evil.example/news/one</link><pubDate>Fri, 27 Nov 2026 00:00:00 GMT</pubDate></item>
</channel></rss>"#;
const ATOM_FEED:&[u8]=br#"<feed xmlns="http://www.w3.org/2005/Atom"><entry><title>Atom headline</title><link rel="alternate" href="/news/atom"/>
<link rel="enclosure" href="https://media.example/content.mp4" type="video/mp4"/><published>2026-11-26T23:25:00-04:00</published><content>body not syndicated</content></entry></feed>"#;
#[test]
fn rss_retains_only_metadata_and_deduplicates() {
    let s = publisher();
    let rows = parse_feed(&s, RSS, &s.feed).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["title"], "Security & Updates");
    assert_eq!(rows[0]["url"], "https://publisher.example/news/one?keep=1");
    assert_eq!(
        rows[0]["media"],
        json!([{"kind":"image","url":"https://media.example/card.jpg"},{"kind":"audio","url":"https://publisher.example/news/audio.mp3"},{"kind":"embed","url":"https://www.youtube.com/watch?v=zFwAT04IvGA"}])
    );
    let encoded = serde_json::to_string(&rows).unwrap();
    assert!(!encoded.contains("unwanted"));
    assert!(!encoded.contains("evil.example"));
}
#[test]
fn atom_normalizes_enclosure_dates_and_rejects_entity_documents() {
    let s = publisher();
    let rows = parse_feed(&s, ATOM_FEED, &s.feed).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["date"], "2026-11-27");
    assert_eq!(
        rows[0]["media"],
        json!([{"kind":"video","url":"https://media.example/content.mp4"}])
    );
    assert!(parse_feed(&s, b"<!DOCTYPE rss [<!ENTITY a 'xx'>]><rss/>", &s.feed).is_err());
    assert!(parse_feed(&s, &vec![b'x'; FEED_BYTES + 1], &s.feed).is_err());
}
#[test]
fn publisher_redirect_boundary_and_port_normalization_are_enforced() {
    let s = publisher();
    for raw in [
        "https://evil.example/raw",
        "http://publisher.example/raw",
        "https://publisher.example@evil.example/raw",
        "https://publisher.example:8080/raw",
        "https://publisher.example:443/raw",
        "https://publisher.example/news/%2e%2e/compose",
        "https://publisher.example/news/%2Fprivate",
        "https://publisher.example/x\\y",
    ] {
        assert!(safe_url(raw, &s.feed, &s.hosts, false).is_none(), "{raw}");
    }
    assert_eq!(
        safe_url("/atom", &s.feed, &s.hosts, false).as_deref(),
        Some("https://publisher.example/atom")
    );
}
#[test]
fn promotional_posts_and_unapproved_media_are_excluded() {
    let mut s = publisher();
    s.id = "bleeping".into();
    assert!(promotional(&s, &["Deals".into()], &s.feed, "Shopping"));
    assert!(promotional(
        &s,
        &[],
        "https://publisher.example/news/deals/a",
        "Sale"
    ));
    assert!(promotional(&s, &[], &s.feed, "Black Friday discount"));
    assert!(!promotional(
        &s,
        &[],
        &s.feed,
        "Black Friday shoppers targeted by malware"
    ));
    assert!(pick_media(
        &s,
        &[("embed".into(), "https://vimeo.com/123456".into(), "".into())],
        &s.feed
    )
    .is_empty());
    let sans = sources().into_iter().find(|s| s.id == "sans-isc").unwrap();
    let rows = pick_media(
        &sans,
        &[
            (
                "audio".into(),
                "https://traffic.libsyn.com/securitypodcast/10194.mp3".into(),
                "audio/mpeg".into(),
            ),
            (
                "audio".into(),
                "https://traffic.libsyn.com/anotherpodcast/10194.mp3".into(),
                "audio/mpeg".into(),
            ),
        ],
        &sans.feed,
    );
    assert_eq!(
        rows,
        json!([{"kind":"audio","url":"https://traffic.libsyn.com/securitypodcast/10194.mp3"}])
            .as_array()
            .unwrap()
            .clone()
    );
}
#[test]
fn page_metadata_is_head_only_origin_limited_and_bounded() {
    let s = publisher();
    let calls = Mutex::new(Vec::new());
    let (_, status, entries) = refresh_source(&s, true, &|u, _, limit, _, page| {
        calls.lock().unwrap().push((u.to_string(), limit, page));
        Ok((
            if page {
                br#"<html><head><meta property="og:video" content="https://media.example/demo.mp4"><meta property="og:image" content="https://evil.example/ad.png"><meta name="twitter:image" content="https://media.example/fake.svg"></head><body><meta property="og:image" content="https://media.example/not-head.png"></body></html>"#.to_vec()
            } else {
                RSS.to_vec()
            },
            u.to_string(),
        ))
    });
    assert_eq!(status, "ok");
    assert_eq!(
        *calls.lock().unwrap(),
        vec![
            (s.feed.clone(), FEED_BYTES, false),
            (
                "https://publisher.example/news/two".into(),
                PAGE_BYTES,
                true
            )
        ]
    );
    assert_eq!(
        entries[1]["media"],
        json!([{"kind":"video","url":"https://media.example/demo.mp4"}])
    );
    assert_eq!(entries[2]["media"], json!([]));
}
#[test]
fn partial_failure_retains_metadata_and_all_failed_is_atomic() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("digest.json");
    fs::write(&output,json!({"schemaVersion":1,"items":[{"sourceId":"broken","title":"Preserved","url":"https://broken.example/item","date":"2026-11-25","media":[{"kind":[],"url":"https://broken.example/image.png"}]}]}).to_string()).unwrap();
    let fetcher = |u: &str, _: &[String], _: usize, _: Instant, _: bool| {
        if u == broken().feed {
            Err("publisher down".into())
        } else {
            Ok((RSS.to_vec(), u.to_string()))
        }
    };
    let now = snapshot_time("2026-11-27T03:00:00Z").unwrap();
    let obj = refresh(
        &output,
        &[publisher(), broken()],
        false,
        None,
        &fetcher,
        now,
    )
    .unwrap();
    assert_eq!(obj["generatedAt"], "2026-11-27T03:00:00Z");
    assert_eq!(
        obj["sources"],
        json!([{"id":"primary","status":"ok","origin":"feed","lastSuccessfulCheckAt":"2026-11-27T03:00:00Z"},{"id":"broken","status":"unavailable","origin":"feed"}])
    );
    assert_eq!(
        obj["items"].as_array().unwrap().last().unwrap()["title"],
        "Preserved"
    );
    assert_eq!(
        obj["items"].as_array().unwrap().last().unwrap()["media"],
        json!([])
    );
    let before = fs::read(&output).unwrap();
    assert!(refresh(&output, &[broken()], false, None, &fetcher, now)
        .unwrap_err()
        .contains("previous output left unchanged"));
    assert_eq!(fs::read(&output).unwrap(), before);
}
#[test]
fn freshness_survives_partial_failure_and_recovers() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("digest.json");
    let phase = Mutex::new(BTreeSet::<String>::new());
    let fetcher = |u: &str, _: &[String], _: usize, _: Instant, _: bool| {
        if phase.lock().unwrap().contains(u) {
            Err("publisher down".into())
        } else {
            Ok((
                if u == broken().feed {
                    br#"<rss><channel><item><title>Kept article</title><link>https://broken.example/item</link><pubDate>Thu, 26 Nov 2026 00:00:00 GMT</pubDate></item></channel></rss>"#.to_vec()
                } else {
                    RSS.to_vec()
                },
                u.to_string(),
            ))
        }
    };
    let run = |hour| {
        refresh(
            &output,
            &[publisher(), broken()],
            false,
            None,
            &fetcher,
            snapshot_time(&format!("2026-11-27T0{hour}:00:00Z")).unwrap(),
        )
    };
    let first = run(1).unwrap();
    phase.lock().unwrap().insert(broken().feed);
    let partial = run(2).unwrap();
    assert_eq!(partial["items"], first["items"]);
    assert_eq!(
        partial["sources"][1],
        json!({"id":"broken","status":"unavailable","origin":"feed","lastSuccessfulCheckAt":"2026-11-27T01:00:00Z"})
    );
    phase.lock().unwrap().insert(publisher().feed);
    let before = fs::read(&output).unwrap();
    assert!(run(3).is_err());
    assert_eq!(fs::read(&output).unwrap(), before);
    phase.lock().unwrap().clear();
    let recovered = run(4).unwrap();
    assert_eq!(recovered["items"], first["items"]);
    assert_eq!(
        recovered["sources"][1]["lastSuccessfulCheckAt"],
        "2026-11-27T04:00:00Z"
    );
    let empty = refresh(
        &output,
        &[broken()],
        false,
        None,
        &|u, _, _, _, _| Ok((b"<rss><channel/></rss>".to_vec(), u.into())),
        snapshot_time("2026-11-27T05:00:00Z").unwrap(),
    )
    .unwrap();
    assert_eq!(empty["items"], json!([]));
    assert_eq!(
        empty["sources"][0]["lastSuccessfulCheckAt"],
        "2026-11-27T05:00:00Z"
    );
}
#[test]
fn editorial_seed_is_approved_normalized_and_optional() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("seed.json");
    let mut item = json!({"sourceId":"windows-health","title":"Windows video","url":"https://learn.microsoft.com/en-us/windows/release-health/","date":"2026-09-29","media":[{"kind":"embed","url":"https://www.youtube.com/watch?v=zFwAT04IvGA"}]});
    fs::write(&path, json!({"items":[item.clone()]}).to_string()).unwrap();
    let rows = load_seed(&path).unwrap();
    assert_eq!(rows[0].0, "windows-health");
    assert_eq!(rows[0].2[0]["media"], item["media"]);
    item["url"] = json!("https://attacker.example/item");
    fs::write(&path, json!([item]).to_string()).unwrap();
    assert!(load_seed(&path).is_err());
}
fn data(name: &str) -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../data")
                .join(name),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_news_catalog_and_snapshot_validate_without_changing_review_dates() {
    let news = data("news-sources.json");
    let digest = data("news-digest.json");
    validate_news(&news).unwrap();
    validate_digest(&digest, &news).unwrap();
    let categories: BTreeSet<_> = items(&news, "sources")
        .iter()
        .map(|v| field(v, "category"))
        .collect();
    assert_eq!(
        categories,
        BTreeSet::from([
            "advisory",
            "compliance",
            "vendor",
            "reporting",
            "community",
            "status"
        ])
    );
}
#[test]
fn invalid_sources_links_and_future_briefings_are_rejected() {
    let news = data("news-sources.json");
    for (key, value) in [
        ("url", json!("javascript:alert(1)")),
        ("feed", json!("https://fake@evil.example/")),
        ("category", json!("unknown")),
    ] {
        let mut bad = news.clone();
        bad["sources"][0][key] = value;
        assert!(validate_news(&bad).is_err(), "{key}");
    }
    for (key, value) in [
        ("sourceId", json!("nonexistent")),
        ("date", json!("2099-01-01")),
    ] {
        let mut bad = news.clone();
        bad["briefing"][0][key] = value;
        assert!(validate_news(&bad).is_err(), "{key}");
    }
}
#[test]
fn digest_rejects_cross_source_links_malicious_media_and_forged_ids() {
    let news = data("news-sources.json");
    let snapshot = data("news-digest.json");
    let pairs = [
        ("url", json!("http://publisher.example/item")),
        ("url", json!("https://user@publisher.example/item")),
        ("sourceId", json!("not-a-source")),
        ("date", json!("2099-03-01")),
        (
            "media",
            json!([{"kind":"embed","url":"https://video.invalid/iframe"}]),
        ),
        (
            "media",
            json!([{"kind":"video","url":"javascript:alert(1)"}]),
        ),
        (
            "media",
            json!([{"kind":"image","url":"https://unknown.example/private.png"}]),
        ),
        ("url", json!("https://githubstatus.com.example.com/update")),
        ("id", json!("forged-identity")),
    ];
    for (key, value) in pairs {
        let mut bad = snapshot.clone();
        bad["items"][0][key] = value;
        assert!(validate_digest(&bad, &news).is_err(), "{key}");
    }
}
#[test]
fn source_freshness_is_optional_qualified_and_never_faked() {
    let news = data("news-sources.json");
    let mut snapshot = data("news-digest.json");
    let generated = snapshot["generatedAt"].clone();
    for row in snapshot["sources"].as_array_mut().unwrap() {
        match field(row, "id") {
            "windows-health" => row["origin"] = json!("editorial"),
            "github-status" => {
                row["origin"] = json!("feed");
                row["lastSuccessfulCheckAt"] = generated.clone();
            }
            "bleeping" => {
                row["origin"] = json!("feed");
                row["lastSuccessfulCheckAt"] = json!("2026-11-26T19:00:00Z");
            }
            _ => {}
        }
    }
    validate_digest(&snapshot, &news).unwrap();
    for (id, key, value) in [
        ("windows-health", "lastSuccessfulCheckAt", generated.clone()),
        ("github-status", "origin", json!("editorial")),
        (
            "github-status",
            "lastSuccessfulCheckAt",
            json!("2026-11-25T19:00:00Z"),
        ),
        (
            "bleeping",
            "lastSuccessfulCheckAt",
            json!("2026-11-29T05:00:00Z"),
        ),
        (
            "bleeping",
            "lastSuccessfulCheckAt",
            json!("2026-11-26T19:00:00+00:00"),
        ),
        ("bleeping", "lastSuccessfulCheckAt", Value::Null),
    ] {
        let mut bad = snapshot.clone();
        let row = bad["sources"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| field(r, "id") == id)
            .unwrap();
        row[key] = value;
        assert!(validate_digest(&bad, &news).is_err(), "{id}.{key}");
    }
}
