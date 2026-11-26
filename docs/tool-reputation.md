# Tool reputation, recommendations and source intelligence

**Status:** canonical implementation specification for the INTA tool reputation layer.  
**Reviewed:** 2026-11-26.

## Goal

Give every catalog tool a transparent reputation profile that technicians can search, filter, compare and audit. Preserve popularity, expert opinion, independent testing, customer reviews, community recommendations, security evidence and INTA's own validation as distinct evidence classes. Never collapse them into an unexplained star score.

## Per-tool profile

Every record in `data/tools.json` receives a stable `reputation` object. The object is an evidence ledger, not a manually invented score:

```json
{
  "reputation": {
    "github": {
      "repository": "https://github.com/OWNER/REPOSITORY",
      "stars": null,
      "forks": null,
      "watchers": null,
      "contributors": null,
      "openIssues": null,
      "latestRelease": null,
      "checkedAt": null,
      "status": "not-applicable"
    },
    "ratings": [],
    "recommendations": [],
    "securitySignals": [],
    "consensus": {
      "distinctSources": 0,
      "distinctProfessionalSources": 0,
      "distinctRecommenders": 0,
      "lastCalculatedAt": null,
      "status": "unrated"
    }
  }
}
```

Use `not-applicable` for proprietary tools without a public repository, `not-found` only after a documented search, `unavailable` for temporary provider failure, and `unverified` when identity is ambiguous. A zero is a measured zero, never a substitute for missing data.

### Rating record

Each external rating contains `providerId`, `productUrl`, `ratingValue`, `ratingScaleMax`, `reviewCount`, `edition`, `capturedAt`, `sourceType`, and `notes`. Preserve the provider's native scale. Normalize only for cross-provider display, retain the original value, and never average without showing the methodology and review counts. Do not scrape gated reviews or fabricate unavailable values.

### Recommendation record

Each recommendation contains `sourceId`, `sourceClass`, `sourceUrl`, `recommender` (public author or organization when available), `publishedAt`, `capturedAt`, `recommendationType`, `context`, `evidenceExcerpt` (short), `independenceKey`, and `confidence`. Recommendation types: `hands-on-review`, `editor-pick`, `professional-use`, `community-recommendation`, `benchmark-result`, `official-reference`. An official product page is evidence of identity/capability, not an independent endorsement.

## Search, filters and sorting

Expose independent filters for:
- Category, platform, delivery mode, price/licensing, tier and intended job.
- GitHub stars/forks/release activity and repository availability.
- User rating and minimum user review count.
- Each external review provider and minimum review count.
- Professional recommendations, publication recommendations and community recommendations.
- Number of distinct recommending sources and number of distinct recommending people.
- Independent lab evidence, security advisories, maintenance freshness and INTA validation level.

Sort options include most stars, most forks, highest native rating (within one provider), highest normalized rating (only with sample-size disclosure), most distinct recommenders, most independent sources, most professional recommendations, most recent recommendation, latest release, healthiest maintenance and strongest independently tested result.

Search must cover tool names, aliases, use cases, tags, provider names, review excerpts, recommending organizations, source titles and recommendation contexts. Filters compose rather than replace one another. Missing metrics must be filterable as unknown and must not silently rank as zero.

## Consensus and anti-manipulation

Count unique source URLs and independent source organizations, not duplicated syndicated articles. Count unique public recommenders where identity is available; do not infer identity from similar usernames. Collapse cross-posts, affiliate roundups, vendor-authored placements and repeated links into one evidence family. Detect sudden bursts, review stuffing, incentivized reviews, suspicious account patterns and coordinated brigading; flag for human review rather than silently deleting.

A source's authority depends on the claim. Vendor docs are strongest for supported features and licensing; independent labs for their exact test; standards bodies for standards; professional review sites for user experience; practitioner communities for field discovery. No source is universally authoritative.

## Scoring display

Prefer a transparent evidence panel over one overall score. If a composite is ever added, publish its formula, weights, freshness decay, sample-size adjustment, source independence rules and missing-data handling. Show component scores beside it. Never let GitHub stars dominate security suitability or let a high consumer rating imply enterprise readiness.

Recommended default presentation:
1. INTA validation level and current support status.
2. Independent lab results relevant to the exact product/version.
3. Professional review ratings with review counts.
4. Distinct expert/publication recommendations.
5. Distinct practitioner recommendations and community discussion volume.
6. GitHub popularity and maintenance metrics, when applicable.
7. Security advisories, provenance and license caveats.

## Sources and discovery hubs

The canonical provider registry is `data/reputation-sources.json`. It includes code forges, review platforms, technical publications, practitioner communities, standards bodies, security intelligence and official documentation. Expand it by discipline, not by indiscriminate link dumping.

High-value discovery routes include:
- **Code and releases:** GitHub, GitLab, Codeberg, SourceForge, official vendor release pages.
- **Professional reviews:** G2, Capterra, TrustRadius, Gartner Peer Insights and SourceForge Reviews.
- **Practitioner communities:** Spiceworks Community, Reddit's r/sysadmin, r/msp, r/techsupport, r/datarecovery, r/cybersecurity, r/netsec and r/homelab; Server Fault, Super User, Stack Overflow.
- **Technical publications:** BleepingComputer, Ars Technica, The Register, ServeTheHome, Level1Techs, Tom's Hardware and PCMag.
- **Independent testing and standards:** AV-Comparatives, AV-TEST, MITRE ATT&CK, NIST NVD, CISA KEV, CISA, OWASP and SANS.
- **Threat and detection intelligence:** VirusTotal, URLhaus, MalwareBazaar, SigmaHQ and YARA.
- **Primary technical documentation:** Microsoft Learn, Debian, Red Hat and each tool's official documentation.

Use social networks as discovery channels, not as unverified review authorities. Preserve the original post and author, distinguish a personal anecdote from a reproducible test, and never count a like or follower count as a recommendation.

## Freshness and acquisition

Store `capturedAt`, provider URL, retrieval status and provider-specific identifiers for every observation. GitHub repository metrics can be refreshed through the documented GitHub REST/GraphQL APIs subject to authentication and rate limits. Other providers must use documented APIs, feeds or permitted public pages. Respect terms, robots directives, authentication, rate limits and privacy. No paywall, CAPTCHA, login, access-control or anti-bot bypass.

Do not poll continuously. Refresh on explicit user request, scheduled low-frequency maintenance, tool-profile opening, or a controlled batch job. Cache provider responses, apply conditional requests where supported, and retain historical snapshots so growth and decline can be charted. Failed fetches preserve the last good value with a stale badge; they do not overwrite it with zero.

## Editorial and safety rules

- A tool's INTA tier (`core`, `specialist`, `evaluate`) is not a public rating.
- A catalog listing or inclusion in a boot image is not an endorsement.
- Record exact product edition, operating system, version, test conditions and date.
- Separate vendor claims, third-party findings, technician reports and INTA test results.
- Disclose sponsorship, affiliate links, review incentives and conflicts where known.
- Never publish client identities, ticket details, private forum content, credentials or telemetry.
- Preserve licenses and redistribution restrictions; recommendation metadata never grants permission to bundle a tool.
- Security ratings must be contextual. A vulnerability match is not a generic quality score; a clean reputation lookup is not proof of safety.

## Acceptance criteria

1. Every existing catalog tool can render a reputation section, including an explicit "not yet measured" state.
2. All metrics show provider, exact source, native scale, sample size and checked date.
3. Users can search and combine every filter listed above without losing catalog entries.
4. GitHub metrics are available only for correctly matched repositories; proprietary tools remain first-class.
5. Recommendation counts deduplicate sources and recommenders and expose their underlying evidence.
6. Ratings from different scales are never averaged as if equivalent.
7. Stale, missing, failed and not-applicable states are visually distinct.
8. Tests cover sorting, filtering, unknown values, deduplication, provider failures, stale data, URL validation and malicious/untrusted source content.
9. The full hub, wiki, offline HTML and keyboard/small-screen experiences remain consistent.
10. No reputation data promotes a tool to `core` or to the rescue image without the existing technical, licensing, safety and runtime gates.
