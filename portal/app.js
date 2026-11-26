(function () {
  'use strict';

  const data = window.INTA_DATA || { tools: [], jobs: [], meta: {} };
  const tools = Array.isArray(data.tools) ? data.tools : [];
  const jobs = Array.isArray(data.jobs) ? data.jobs : [];
  const meta = data.meta || {};
  const toolMap = new Map(tools.map(tool => [tool.id, tool]));
  const jobMap = new Map(jobs.map(job => [job.id, job]));
  const app = document.getElementById('app');
  const search = document.getElementById('global-search');
  const searchResults = document.getElementById('search-results');
  const $ = selector => document.querySelector(selector);
  const $$ = selector => Array.from(document.querySelectorAll(selector));
  const repo = 'https://github.com/Herbertofury/INTA';
  const reviewDate = meta.reviewedAt || meta.reviewed_at || '2026-11-26';
  const savedKey = 'inta:favorite-tools:v1';
  const checksKey = 'inta:job-checklists:v1';
  const themeKey = 'inta:theme:v1';
  let storageAvailable = true;
  let lastRoute = '';
  let toastTimer;
  let undoAction = null;

  function load(key, fallback) {
    try {
      const value = localStorage.getItem(key);
      return value === null ? fallback : JSON.parse(value);
    } catch (_) { storageAvailable = false; return fallback; }
  }
  function store(key, value) {
    try { localStorage.setItem(key, JSON.stringify(value)); return true; }
    catch (_) { storageAvailable = false; return false; }
  }
  const rawFavorites = load(savedKey, []);
  let favorites = new Set(Array.isArray(rawFavorites) ? rawFavorites.filter(id => toolMap.has(id)) : []);
  let checklists = load(checksKey, {});
  if (!checklists || typeof checklists !== 'object' || Array.isArray(checklists)) checklists = {};
  const theme = load(themeKey, 'dark');
  document.documentElement.dataset.theme = theme === 'light' ? 'light' : 'dark';

  const paths = {
    home: '<path d="m3 10 9-7 9 7v10a1 1 0 0 1-1 1h-5v-7H9v7H4a1 1 0 0 1-1-1Z"/>',
    grid: '<rect x="3" y="3" width="7" height="7" rx="1.5"/><rect x="14" y="3" width="7" height="7" rx="1.5"/><rect x="3" y="14" width="7" height="7" rx="1.5"/><rect x="14" y="14" width="7" height="7" rx="1.5"/>',
    book: '<path d="M12 5c-3-2-7-2-10-1v15c3-1 7-1 10 1 3-2 7-2 10-1V4c-3-1-7-1-10 1Z"/><path d="M12 5v15"/>',
    bookmark: '<path d="M6 4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v17l-6-4-6 4Z"/>',
    compass: '<circle cx="12" cy="12" r="9"/><path d="m16 8-2.5 5.5L8 16l2.5-5.5Z"/>',
    usb: '<path d="M8 8h8v12H8zM9 8V3h6v5M11 5h2M10 17h4"/>',
    external: '<path d="M14 3h7v7M21 3l-11 11M10 3H4a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-6"/>',
    layers: '<path d="m12 3 10 5-10 5L2 8ZM2 12l10 5 10-5M2 16l10 5 10-5"/>',
    shield: '<path d="M12 3c3 2 6 3 9 3v6c0 5-5 8-9 10-4-2-9-5-9-10V6c3 0 6-1 9-3Z"/><path d="m8 12 3 3 5-6"/>',
    sun: '<circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M5 5l1.5 1.5M17.5 17.5 19 19M19 5l-1.5 1.5M6.5 17.5 5 19"/>',
    moon: '<path d="M20.8 13.2A9 9 0 0 1 10.8 3.2a9 9 0 1 0 10 10Z"/>',
    search: '<circle cx="10.5" cy="10.5" r="6.5"/><path d="m16 16 5 5"/>',
    github: '<path d="M9 19c-4 1-4-2-6-2m12 5v-3.5c0-1 .1-1.5-.5-2 3-.3 6-1.5 6-6a4.5 4.5 0 0 0-1.2-3c.2-.8.3-2-.3-3-1.5 0-3 1-3.5 1.3a13 13 0 0 0-7 0C8 5.5 6.5 4.5 5 4.5c-.6 1-.5 2.2-.3 3a4.5 4.5 0 0 0-1.2 3c0 4.5 3 5.7 6 6-.6.5-.5 1-.5 2V22"/>',
    arrow: '<path d="M4 12h16m-6-6 6 6-6 6"/>',
    back: '<path d="M20 12H4m6-6-6 6 6 6"/>',
    menu: '<path d="M4 6h16M4 12h16M4 18h16"/>',
    clock: '<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>',
    check: '<path d="m5 12 4 4L19 6"/>',
    checkCircle: '<circle cx="12" cy="12" r="9"/><path d="m7 12 3 3 7-7"/>',
    activity: '<path d="M2 12h5l3-8 4 16 3-8h5"/>',
    network: '<rect x="8" y="3" width="8" height="6" rx="1"/><rect x="2" y="16" width="7" height="5" rx="1"/><rect x="15" y="16" width="7" height="5" rx="1"/><path d="M12 9v4M5.5 16v-3h13v3"/>',
    monitor: '<rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/>',
    drive: '<rect x="3" y="4" width="18" height="16" rx="3"/><path d="M3 13h18M7 16h.01M11 16h.01"/>',
    wrench: '<path d="M14 3a6 6 0 0 0-7 7L3 17a3 3 0 0 0 4 4l7-7a6 6 0 0 0 7-7l-4 4-4-4Z"/>',
    life: '<circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="4"/><path d="m5.6 5.6 3.6 3.6m5.6 5.6 3.6 3.6m0-12.8-3.6 3.6m-5.6 5.6-3.6 3.6"/>',
    cloud: '<path d="M7 18a5 5 0 1 1 .7-9.9A7 7 0 0 1 21 11a3.5 3.5 0 0 1-.5 7Z"/><path d="m9 13 3-3 3 3M12 10v10"/>',
    mail: '<rect x="2" y="4" width="20" height="16" rx="2"/><path d="m3 6 9 7 9-7"/>',
    printer: '<path d="M6 8V3h12v5M6 17H3V8h18v9h-3M6 14h12v7H6zM17 11h1"/>',
    lock: '<rect x="4" y="10" width="16" height="11" rx="2"/><path d="M8 10V6a4 4 0 0 1 8 0v4M12 14v3"/>',
    warning: '<path d="m12 3 10 18H2Z"/><path d="M12 9v5M12 17h.01"/>',
    terminal: '<rect x="2" y="3" width="20" height="18" rx="2"/><path d="m6 8 4 4-4 4M13 16h5"/>',
    copy: '<rect x="8" y="8" width="13" height="13" rx="2"/><path d="M16 8V3H3v13h5"/>',
    download: '<path d="M12 3v12m-5-5 5 5 5-5M3 16v5h18v-5"/>',
    link: '<path d="m10 13 4-4M8 15l-2 2a4 4 0 0 1-6-6l5-5a4 4 0 0 1 6 0M16 9l2-2a4 4 0 0 1 6 6l-5 5a4 4 0 0 1-6 0" transform="translate(1 -1) scale(.9)"/>',
    file: '<path d="M14 2H4v20h16V8Z"/><path d="M14 2v6h6M8 13h8M8 17h6"/>',
    folder: '<path d="M3 5h7l2 3h9v13H3Z"/>',
    refresh: '<path d="M20 8a8 8 0 0 0-14-3L3 8m0-6v6h6M4 16a8 8 0 0 0 14 3l3-3m0 6v-6h-6"/>',
    route: '<circle cx="5" cy="5" r="2"/><circle cx="19" cy="19" r="2"/><path d="M5 7v7a5 5 0 0 0 5 5h7M10 5h4a5 5 0 0 1 5 5v4"/>',
    zap: '<path d="m13 2-9 12h7l-1 8 10-13h-7Z"/>',
    info: '<circle cx="12" cy="12" r="9"/><path d="M12 11v6M12 7h.01"/>',
    filter: '<path d="M3 4h18l-7 8v7l-4 2v-9Z"/>',
    close: '<path d="m6 6 12 12M18 6 6 18"/>'
  };
  function icon(name, className = '') {
    return `<svg class="${className}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.55" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${paths[name] || paths.wrench}</svg>`;
  }
  function esc(value) {
    return String(value == null ? '' : value).replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char]));
  }
  function safeUrl(value) {
    try { const parsed = new URL(String(value)); return ['https:', 'http:'].includes(parsed.protocol) ? parsed.href : ''; }
    catch (_) { return ''; }
  }
  function external(value, content, className = '') {
    const href = safeUrl(value);
    return href ? `<a href="${esc(href)}" target="_blank" rel="noopener noreferrer"${className ? ` class="${esc(className)}"` : ''}>${content}</a>` : '';
  }
  function text(value) { return Array.isArray(value) ? value.join('\n') : String(value == null ? '' : value); }
  function formatted(value) { return esc(text(value)).replace(/`([^`\n]+)`/g, '<code>$1</code>').replace(/\*\*([^*\n]+)\*\*/g, '<strong>$1</strong>'); }
  function array(value) { return Array.isArray(value) ? value : value ? [value] : []; }
  function registryUrl(value) {
    if (/^#[a-z0-9][a-z0-9/_?=&.%~-]*$/i.test(String(value))) return String(value);
    try {
      const url = new URL(String(value));
      return url.protocol === 'https:' && !url.username && !url.password ? url.href : '';
    } catch (_) { return ''; }
  }
  const proseRegistry = new Map();
  array(data.links).forEach(entry => {
    if (!entry || typeof entry.name !== 'string' || !entry.name.trim()) return;
    const href = registryUrl(entry.url);
    const key = entry.name.toLocaleLowerCase('en-US');
    if (href && !proseRegistry.has(key)) proseRegistry.set(key, { ...entry, href });
  });
  const prosePattern = [...proseRegistry.values()].sort((a, b) => b.name.length - a.name.length)
    .map(entry => entry.name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|');
  const proseMatcher = prosePattern ? new RegExp(`(?<![\\p{L}\\p{N}_])(?:${prosePattern})(?![\\p{L}\\p{N}_])`, 'giu') : null;
  const noProseLinks = 'a,button,code,pre,script,style,svg,textarea,select,option,kbd,samp,[data-no-link]';
  function linkifyProse(container = app) {
    if (!container || !proseMatcher) return;
    const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        return node.nodeValue.trim() && node.parentElement && !node.parentElement.closest(noProseLinks)
          ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT;
      }
    });
    const nodes = [];
    while (walker.nextNode()) nodes.push(walker.currentNode);
    nodes.forEach(node => {
      const value = node.nodeValue;
      const protectedSpans = [];
      if (/https?:|`|<a\b/i.test(value)) {
        for (const span of value.matchAll(/https?:\/\/[^\s<>]+|`[^`\n]+`|<a\b[^>]*>[\s\S]*?<\/a>/gi)) {
          protectedSpans.push([span.index, span.index + span[0].length]);
        }
      }
      proseMatcher.lastIndex = 0;
      let match, last = 0, fragment = null;
      while ((match = proseMatcher.exec(value))) {
        const entry = proseRegistry.get(match[0].toLocaleLowerCase('en-US'));
        if (!entry || (entry.caseSensitive && match[0] !== entry.name)) continue;
        if (protectedSpans.some(([start, end]) => match.index < end && match.index + match[0].length > start)) continue;
        if (!fragment) fragment = document.createDocumentFragment();
        fragment.append(document.createTextNode(value.slice(last, match.index)));
        const anchor = document.createElement('a');
        anchor.className = 'prose-link';
        anchor.href = entry.href;
        anchor.textContent = match[0];
        anchor.dataset.linkName = entry.name;
        if (entry.kind) anchor.dataset.linkKind = entry.kind;
        if (entry.id) anchor.dataset.linkId = entry.id;
        if (!entry.href.startsWith('#')) { anchor.target = '_blank'; anchor.rel = 'noopener noreferrer'; }
        fragment.append(anchor);
        last = match.index + match[0].length;
      }
      if (fragment) { fragment.append(document.createTextNode(value.slice(last))); node.replaceWith(fragment); }
    });
  }
  function commandReferences(value) {
    if (!proseMatcher) return [];
    const found = new Map();
    const source = text(value);
    proseMatcher.lastIndex = 0;
    let match;
    while ((match = proseMatcher.exec(source))) {
      const entry = proseRegistry.get(match[0].toLocaleLowerCase('en-US'));
      if (entry && (!entry.caseSensitive || match[0] === entry.name) && !found.has(entry.href)) found.set(entry.href, entry);
    }
    return [...found.values()];
  }
  function commandLegend(value) {
    const references = commandReferences(value);
    return references.length ? `<p class="command-legend">Command references: ${references.map(entry => entry.href.startsWith('#') ? `<a href="${esc(entry.href)}">${esc(entry.name)}</a>` : external(entry.href, esc(entry.name))).join(' · ')}</p>` : '';
  }
  function dateLabel(value = reviewDate, long = false) {
    const parsed = new Date(`${String(value).slice(0, 10)}T12:00:00Z`);
    return Number.isNaN(parsed.getTime()) ? String(value) : parsed.toLocaleDateString('en-US', { month: long ? 'long' : 'short', day: 'numeric', ...(long ? { year: 'numeric' } : {}), timeZone: 'UTC' });
  }
  function jobIcon(job) {
    const value = `${job.id} ${job.title} ${job.category}`.toLowerCase();
    if (/malware|virus|ransom|incident/.test(value)) return 'shield';
    if (/internet|dns|network/.test(value)) return 'network';
    if (/remote/.test(value)) return 'monitor';
    if (/mail|outlook/.test(value)) return 'mail';
    if (/print/.test(value)) return 'printer';
    if (/backup|restore|migration|imaging/.test(value)) return 'cloud';
    if (/disk|drive|storage|recovery/.test(value)) return 'drive';
    if (/boot|usb/.test(value)) return 'usb';
    if (/\bai\b|assistant/.test(value)) return 'terminal';
    if (/trust|domain|bitlocker|account|offboard/.test(value)) return 'lock';
    if (/slow|performance|hardware|bsod/.test(value)) return 'activity';
    if (/intake|handoff|onboard/.test(value)) return 'file';
    return 'wrench';
  }
  const categoryLabels = { ai: 'AI workspace', boot: 'Boot & rescue', hardware: 'Hardware diagnostics', malware: 'Malware removal', network: 'Networking', productivity: 'Technician essentials', remote: 'Remote support', security: 'Security & investigation', storage: 'Storage & recovery', windows: 'Windows repair' };
  function categoryLabel(value) { return categoryLabels[value] || value; }
  function modeLabel(value) { return ({ 'built-in': 'Built into the system', portable: 'Portable', installed: 'Installed application', web: 'Web application', boot: 'Bootable media' })[value] || value; }
  function tierLabel(tier) { return ({ core: 'Everyday pick', specialist: 'Specialist', evaluate: 'Evaluate first' })[tier] || 'Reference'; }
  function tierPill(tier) { return `<span class="recommendation ${esc(tier)}">${esc(tierLabel(tier))}</span>`; }
  function reviewBadge() { return `<span class="review-badge">Source review <time datetime="${esc(reviewDate)}">${esc(dateLabel(reviewDate, true))}</time></span>`; }
  function heading(eyebrow, title, subtitle, badge = true) {
    return `<header class="page-heading"><div><p class="page-label">${esc(eyebrow)}</p><h1>${esc(title)}</h1><p class="page-description">${esc(subtitle)}</p></div>${badge ? reviewBadge() : ''}</header>`;
  }
  function saveButton(tool) {
    const saved = favorites.has(tool.id);
    return `<button class="save-tool" data-save="${esc(tool.id)}" aria-pressed="${saved}" aria-label="${saved ? 'Unsave' : 'Save'} ${esc(tool.name)}" title="${saved ? 'Remove from saved tools' : 'Save this tool'}">${icon('bookmark')}</button>`;
  }
  function toolTableHeader() {
    return '<div class="tool-table-head" aria-hidden="true"><span>Tool &amp; purpose</span><span>Platforms</span><span>Recommendation</span><span>Resources</span><span>Save</span></div>';
  }
  const firstJobForTool = new Map();
  jobs.forEach(job => array(job.tools).forEach(id => { if (!firstJobForTool.has(id)) firstJobForTool.set(id, job); }));
  function toolRow(tool, directory = false) {
    const related = firstJobForTool.get(tool.id);
    const description = directory ? tool.bestFor || tool.summary : tool.summary;
    return `<article class="tool-row${directory ? ' directory-tool-row' : ''}" role="listitem" data-tool-id="${esc(tool.id)}"${directory ? '' : ` data-tool-card="${esc(tool.id)}"`}><div class="tool-name-cell"><h3><a class="${directory ? 'directory-tool-name' : 'tool-name'}" href="#tool/${encodeURIComponent(tool.id)}">${esc(tool.name)}</a></h3><p>${formatted(description)}</p><span class="row-meta">${directory ? esc(modeLabel(tool.mode)) : esc(categoryLabel(tool.category)) + ' · ' + esc(modeLabel(tool.mode))}</span></div><div class="tool-platform-cell" data-label="Platforms"><span class="sr-only">Platforms: </span>${esc(array(tool.platforms).join(', '))}</div><div class="tool-tier-cell" data-label="Recommendation"><span class="sr-only">Recommendation: </span>${tierPill(tool.tier)}</div><div class="tool-links-cell" data-label="Resources">${external(tool.downloadUrl || tool.homepage, `Official source ${icon('external')}`)}<a href="#tool/${encodeURIComponent(tool.id)}">Details</a>${directory ? related ? `<a href="#job/${encodeURIComponent(related.id)}">Job guide ${icon('arrow')}</a>` : external(`${repo}/wiki/Tools-${encodeURIComponent(tool.category)}`, `Wiki ${icon('external')}`) : external(tool.docsUrl, `Documentation ${icon('external')}`)}</div><div class="tool-save-cell">${saveButton(tool)}</div></article>`;
  }
  function toolCard(tool) { return toolRow(tool); }
  function jobCard(job) {
    const href = `#job/${encodeURIComponent(job.id)}`;
    return `<article class="job-row" role="listitem" data-job-route="${esc(href)}"><span class="job-row-icon">${icon(jobIcon(job))}</span><div class="job-row-main"><h3><a href="${href}">${esc(job.title)}</a></h3><p>${esc(job.summary)}</p></div><div class="job-row-meta"><span>${esc(job.estimatedTime || `${array(job.steps).length} steps`)}</span><span>${esc(job.difficulty || job.category)}</span></div><a class="job-row-open" href="${href}" aria-label="Open ${esc(job.title)}">Open guide ${icon('arrow')}</a></article>`;
  }
  function quickTool(tool) {
    return `<a class="quick-tool" href="#tool/${encodeURIComponent(tool.id)}"><span>${esc(tool.name)}</span><span>${esc(categoryLabel(tool.category))}</span>${icon('arrow')}</a>`;
  }
  function selectFeatured(items, patterns, count) {
    const chosen = [];
    patterns.forEach(pattern => {
      const item = items.find(value => pattern.test(`${value.id} ${value.title || value.name}`) && !chosen.includes(value));
      if (item) chosen.push(item);
    });
    for (const item of items) { if (chosen.length >= count) break; if (!chosen.includes(item)) chosen.push(item); }
    return chosen;
  }
  const knowledgeResources = [
    { slug: 'Decision-Desk', icon: 'compass', title: 'Decision desk', body: 'Start with the symptom. Follow the decision paths to the right playbook and tool.', label: 'Choose a path' },
    { slug: 'Methods', icon: 'book', title: 'Methods library', body: 'Learn the repeatable techniques behind a diagnosis, a repair, and a verified recovery.', label: 'Explore methods' },
    { slug: 'Knowledge-Standards', icon: 'layers', title: 'Knowledge standards', body: 'Use one clear structure for evidence, links, diagrams, review dates, and useful guidance.', label: 'Read the standards' },
    { slug: 'Templates', icon: 'file', title: 'Service templates', body: 'Turn a service visit, a tool evaluation, or a proven fix into a complete record.', label: 'Choose a template' },
    { slug: 'Contribute', icon: 'github', title: 'Contribute a fix', body: 'Bring a source, explain the improvement, and help the next technician reproduce the result.', label: 'Contribution guide' },
    { slug: 'Continuous-Improvement', icon: 'refresh', title: 'Continuous improvement', body: 'Review what changed, close gaps in a method, and use lessons from real jobs to improve the guidance.', label: 'Review the process' },
    { slug: 'Interface-Design', icon: 'grid', title: 'Interface design', body: 'Read the layout, typography, interaction, and accessibility standards for this workspace.', label: 'Design standards' },
    { slug: 'Tool-Selection', icon: 'filter', title: 'Tool selection', body: 'Compare a tool’s fit, evidence, and operating requirements before using it for a job.', label: 'Selection guide' },
    { slug: 'Service-Evidence', icon: 'checkCircle', title: 'Service evidence', body: 'Keep findings, changes, and verification connected in a useful service record.', label: 'Evidence guide' }
  ];
  function knowledgeDesk(compact = false) {
    if (compact) return `<section class="home-knowledge"><h2>Knowledge desk</h2><nav aria-label="Knowledge desk references">${knowledgeResources.map(resource => external(`${repo}/wiki/${resource.slug}`, `${icon(resource.icon)}<span>${esc(resource.title)}</span>`)).join('')}</nav></section>`;
    return `<div class="knowledge-list" role="list">${knowledgeResources.map(resource => `<article class="knowledge-row" role="listitem"><span class="reference-icon">${icon(resource.icon)}</span><div><h2>${external(`${repo}/wiki/${resource.slug}`, esc(resource.title))}</h2><p>${esc(resource.body)}</p></div>${external(`${repo}/wiki/${resource.slug}`, `${esc(resource.label)} ${icon('external')}`, 'row-action')}</article>`).join('')}</div>`;
  }
  function knowledgePage() {
    return heading('INTA field guide', 'Knowledge desk', 'Decision paths, methods, standards, and templates for repeatable service work.') + knowledgeDesk() + `<section class="document-section"><h2>From service work to shared guidance</h2><ol class="process-list"><li><strong>Choose the path.</strong> Start at the decision desk and name the problem you need to solve.</li><li><strong>Follow the method.</strong> Use the relevant playbook, primary references, and a measured next step.</li><li><strong>Verify the result.</strong> Reproduce the original task and record the outcome in a private service ticket.</li><li><strong>Improve the guide.</strong> Share the reusable lesson with a source and a clear verification.</li></ol></section>`;
  }
  let groupedToolDirectory;
  function toolDisciplines() {
    if (groupedToolDirectory) return groupedToolDirectory;
    const grouped = new Map();
    tools.forEach(tool => { if (!grouped.has(tool.category)) grouped.set(tool.category, []); grouped.get(tool.category).push(tool); });
    const preferred = ['malware', 'security', 'remote', 'windows', 'storage', 'hardware', 'network', 'boot', 'productivity', 'ai'];
    const categories = [...grouped.keys()].sort((a, b) => {
      const ai = preferred.indexOf(a), bi = preferred.indexOf(b);
      return (ai < 0 ? 999 : ai) - (bi < 0 ? 999 : bi) || categoryLabel(a).localeCompare(categoryLabel(b));
    });
    groupedToolDirectory = categories.map(category => ({ category, items: grouped.get(category).slice().sort((a, b) => a.name.localeCompare(b.name)) }));
    return groupedToolDirectory;
  }
  function compactToolDirectory() {
    const disciplines = toolDisciplines();
    return `<section class="all-tools-section" aria-labelledby="directory-heading"><div class="section-title directory-title"><div><h2 id="directory-heading">Tool directory</h2><p>${tools.length} tools across ${disciplines.length} disciplines. Choose a tool for its full details and official references.</p></div><a href="#tools">Filter the library ${icon('filter')}</a></div><nav class="discipline-jump" aria-label="Jump to a tool discipline">${disciplines.map(({category, items}) => `<a href="#discipline-${esc(category)}">${esc(categoryLabel(category))}<span>${items.length}</span></a>`).join('')}</nav><div class="directory-grid">${disciplines.map(({category, items}) => `<section class="directory-section" id="discipline-${esc(category)}" data-discipline="${esc(category)}"><header class="directory-section-header"><h2>${esc(categoryLabel(category))}<span>${items.length} ${items.length === 1 ? 'tool' : 'tools'}</span></h2>${external(`${repo}/wiki/Tools-${encodeURIComponent(category)}`, `Discipline guide ${icon('external')}`)}</header>${toolTableHeader()}<div class="directory-tool-list tool-list" role="list">${items.map(tool => toolRow(tool, true)).join('')}</div></section>`).join('')}</div><p class="directory-footnote"><strong>Everyday</strong> is a practical starting point. <strong>Specialist</strong> serves a focused need. <strong>Evaluate first</strong> identifies a research candidate that needs local validation. Check the official release, compatibility, and terms for the intended job.</p></section>`;
  }
  function home() {
    const featuredJobs = selectFeatured(jobs, [/malware|virus/i, /slow|performance/i, /network|internet|dns/i, /backup|restore/i, /boot|bitlocker/i, /remote/i], 6);
    const kit = selectFeatured(tools, [/sysinternals/i, /rustdesk/i, /crystaldiskinfo/i, /ventoy/i], 4);
    const intake = jobs.find(job => /intake|triage/i.test(job.id + ' ' + job.title));
    return heading('INTA Computer Services', 'Technician home', 'Tools and procedures for computer repair, maintenance, and support.') + `<div class="home-actions"><a class="button primary" href="${intake ? '#job/' + encodeURIComponent(intake.id) : '#playbooks'}">${icon('file')}Start a service visit</a><a class="button ghost" href="#playbooks">${icon('book')}${jobs.length} job playbooks</a><a class="button ghost" href="#knowledge">${icon('layers')}Knowledge desk</a><a class="button ghost" href="#resources">${icon('download')}Offline copy &amp; resources</a></div>${compactToolDirectory()}<div class="home-support"><section><div class="section-title"><h2>Common service jobs</h2><a href="#playbooks">All playbooks ${icon('arrow')}</a></div><div class="job-list" role="list">${featuredJobs.map(jobCard).join('')}</div></section><section><h2>Everyday shortcuts</h2><div class="quick-tools">${kit.map(quickTool).join('')}</div>${knowledgeDesk(true)}</section></div>`;
  }
  function selectOptions(values, label, selected = '') {
    return `<option value="">${esc(label)}</option>${[...new Set(values.filter(Boolean))].sort((a, b) => a.localeCompare(b)).map(value => `<option value="${esc(value)}"${selected === value ? ' selected' : ''}>${esc(categoryLabel(value))}</option>`).join('')}`;
  }
  const toolSearchText = new Map(tools.map(tool => [tool.id, text([tool.name, tool.category, categoryLabel(tool.category), tool.summary, tool.bestFor, ...array(tool.tags), ...array(tool.platforms)]).toLowerCase()]));
  const jobSearchText = new Map(jobs.map(job => [job.id, text([job.title, job.category, job.summary, job.startHere, ...array(job.steps).map(step => step.title)]).toLowerCase()]));
  function matches(haystack, query) { return query.trim().toLowerCase().split(/\s+/).filter(Boolean).every(term => haystack.includes(term)); }
  function library(savedOnly = false, params = new URLSearchParams()) {
    const query = params.get('q') || '';
    const category = params.get('category') || '';
    const platform = params.get('platform') || '';
    const tier = params.get('tier') || '';
    const base = savedOnly ? tools.filter(tool => favorites.has(tool.id)) : tools;
    return heading('INTA tools', savedOnly ? 'Saved tools' : 'Tool library', savedOnly ? 'Your saved tools, kept in this browser for the next service visit.' : 'Search the complete catalog by capability, platform, or recommendation.', !savedOnly) + (savedOnly && !base.length ? empty('bookmark', 'No saved tools yet', 'Use the bookmark control beside any tool to keep it here for your next visit.', 'Explore the tool library', '#tools') : `<div class="filters" id="tool-filters"><label class="input-wrap">${icon('search')}<span class="sr-only">Search tools</span><input id="library-search" type="search" placeholder="Search tools, symptoms, or capabilities…" value="${esc(query)}"></label><label class="sr-only" for="category-filter">Tool category</label><select class="filter-select" id="category-filter">${selectOptions(tools.map(tool => tool.category), 'All categories', category)}</select><label class="sr-only" for="platform-filter">Platform</label><select class="filter-select" id="platform-filter">${selectOptions(tools.flatMap(tool => array(tool.platforms)), 'All platforms', platform)}</select><label class="sr-only" for="tier-filter">Recommendation level</label><select class="filter-select" id="tier-filter"><option value="">All recommendations</option><option value="core"${tier === 'core' ? ' selected' : ''}>Everyday picks</option><option value="specialist"${tier === 'specialist' ? ' selected' : ''}>Specialist tools</option><option value="evaluate"${tier === 'evaluate' ? ' selected' : ''}>Evaluate first</option></select></div><div class="filter-result"><span id="library-result-count" aria-live="polite"></span><div class="legend">Everyday · Specialist · Evaluate first</div><button data-reset-filters>Clear filters</button></div>${toolTableHeader()}<div class="tool-list" id="tool-results" role="list" data-saved-only="${savedOnly}"></div>`);
  }
  function filterTools(linkProse = true) {
    const container = $('#tool-results');
    if (!container) return;
    const query = $('#library-search').value;
    const category = $('#category-filter').value;
    const platform = $('#platform-filter').value;
    const tier = $('#tier-filter').value;
    const base = container.dataset.savedOnly === 'true' ? tools.filter(tool => favorites.has(tool.id)) : tools;
    const found = base.filter(tool => matches(toolSearchText.get(tool.id), query) && (!category || tool.category === category) && (!platform || array(tool.platforms).includes(platform)) && (!tier || tool.tier === tier));
    container.innerHTML = found.length ? found.map(toolCard).join('') : `<div style="grid-column:1/-1">${empty('search', 'No tools match this combination.', 'Try a different search or clear the filters to see the full library.', 'Clear filters', '', 'reset-filters')}</div>`;
    $('#library-result-count').innerHTML = `<strong>${found.length}</strong> of ${base.length} ${base.length === 1 ? 'tool' : 'tools'}`;
    rememberFilters({ q: query, category, platform, tier });
    if (linkProse) linkifyProse(container);
  }
  function playbooks(params = new URLSearchParams()) {
    return heading('INTA field guide', 'Job playbooks', 'Choose a job for its procedure, recommended tools, verification, and escalation path.') + `<div class="filters"><label class="input-wrap">${icon('search')}<span class="sr-only">Search playbooks</span><input id="jobs-search" type="search" value="${esc(params.get('q') || '')}" placeholder="Describe the job or the symptom…"></label><label class="sr-only" for="jobs-category">Playbook category</label><select class="filter-select" id="jobs-category">${selectOptions(jobs.map(job => job.category), 'All job types', params.get('category') || '')}</select></div><div class="filter-result"><span id="jobs-result-count" aria-live="polite"></span><button data-reset-jobs>Clear filters</button></div><div class="job-list" id="job-results" role="list"></div>`;
  }
  function filterJobs(linkProse = true) {
    const container = $('#job-results');
    if (!container) return;
    const query = $('#jobs-search').value;
    const category = $('#jobs-category').value;
    const found = jobs.filter(job => matches(jobSearchText.get(job.id), query) && (!category || job.category === category));
    container.innerHTML = found.length ? found.map(jobCard).join('') : `<div style="grid-column:1/-1">${empty('search', 'Let’s try another symptom.', 'Search by a broad term such as network, boot, backup, or malware.', 'Clear filters', '', 'reset-jobs')}</div>`;
    $('#jobs-result-count').innerHTML = `<strong>${found.length}</strong> of ${jobs.length} playbooks`;
    rememberFilters({ q: query, category });
    if (linkProse) linkifyProse(container);
  }
  function rememberFilters(values) {
    const route = parseRoute();
    const params = new URLSearchParams();
    Object.entries(values).forEach(([key, value]) => { if (value) params.set(key, value); });
    const query = params.toString();
    history.replaceState(null, '', '#' + route.name + (query ? '?' + query : ''));
  }
  function empty(type, title, summary, action, href, dataAction) {
    return `<div class="empty-state"><h2>${esc(title)}</h2><p>${esc(summary)}</p>${action ? dataAction ? `<button class="button primary" data-${dataAction}>${esc(action)} ${icon('arrow')}</button>` : `<a class="button primary" href="${esc(href)}">${esc(action)} ${icon('arrow')}</a>` : ''}</div>`;
  }
  // Anchors survive inserted steps. A unique unchanged title is the fallback for
  // verification items and older catalogs. Duplicate titles are never migrated.
  const checklistIdentities = new Map(jobs.map(job => {
    const entries = [
      ...array(job.steps).map((step, index) => ({ type: 'step', index, title: text(step.title), anchor: step.anchor })),
      ...array(job.verify).map((item, index) => ({ type: 'verify', index, title: text(item) }))
    ];
    const titles = new Map(), anchors = new Map();
    entries.forEach(entry => {
      const title = `${entry.type}:${entry.title}`;
      titles.set(title, (titles.get(title) || 0) + 1);
      if (entry.anchor) {
        const anchor = `${entry.type}:${entry.anchor}`;
        anchors.set(anchor, (anchors.get(anchor) || 0) + 1);
      }
    });
    entries.forEach(entry => {
      entry.uniqueTitle = titles.get(`${entry.type}:${entry.title}`) === 1;
      entry.key = entry.anchor && anchors.get(`${entry.type}:${entry.anchor}`) === 1
        ? `${entry.type}:anchor:${entry.anchor}`
        : entry.uniqueTitle ? `${entry.type}:title:${entry.title}` : `${entry.type}:index:${entry.index}:${entry.title}`;
    });
    return [job.id, { entries, byPosition: new Map(entries.map(entry => [`${entry.type}:${entry.index}`, entry])) }];
  }));
  function checklistKey(job, type, index, title) {
    return checklistIdentities.get(job.id)?.byPosition.get(`${type}:${index}`)?.key || `${type}:title:${title}`;
  }
  function migrateChecklistState() {
    let changed = false;
    jobs.forEach(job => {
      const state = checklists[job.id];
      if (!state || typeof state !== 'object' || Array.isArray(state)) return;
      const identity = checklistIdentities.get(job.id);
      identity.entries.filter(entry => entry.uniqueTitle).forEach(entry => {
        if (Object.prototype.hasOwnProperty.call(state, entry.key)) return;
        const candidates = Object.keys(state).filter(key => {
          const legacy = /^(step|verify):\d+:([\s\S]*)$/.exec(key);
          return legacy ? legacy[1] === entry.type && legacy[2] === entry.title : key === `${entry.type}:title:${entry.title}` && key !== entry.key;
        });
        if (candidates.length !== 1 || typeof state[candidates[0]] !== 'boolean') return;
        state[entry.key] = state[candidates[0]];
        delete state[candidates[0]];
        changed = true;
      });
    });
    if (changed) store(checksKey, checklists);
  }
  function checked(job, type, index, title) { return !!(checklists[job.id] && checklists[job.id][checklistKey(job, type, index, title)]); }
  function progress(job) {
    const all = [...array(job.steps).map((step, i) => checked(job, 'step', i, step.title)), ...array(job.verify).map((item, i) => checked(job, 'verify', i, text(item)))];
    const done = all.filter(Boolean).length;
    return { total: all.length, done, percentage: all.length ? Math.round(done / all.length * 100) : 0 };
  }
  function progressCard(job) {
    const state = progress(job);
    return `<h3>Your checklist</h3><div class="progress-top"><strong id="progress-percentage">${state.percentage}%</strong><span id="progress-count">${state.done} of ${state.total} checks</span></div><div class="progress-track" role="progressbar" aria-label="Playbook completion" aria-valuemin="0" aria-valuemax="100" aria-valuenow="${state.percentage}"><span style="width:${state.percentage}%"></span></div><p class="progress-note" id="progress-note">${storageAvailable ? 'Progress stays in this browser. Start a fresh checklist for each new job.' : 'Progress lasts for this visit because browser storage is unavailable. Export your checklist to keep it.'}</p><button class="checklist-reset" data-reset-checklist="${esc(job.id)}">Start a fresh checklist</button>`;
  }
  function sourceList(sources) {
    return `<ul class="source-list">${array(sources).filter(source => safeUrl(source.url || source)).map(source => `<li>${external(source.url || source, `${esc(source.title || source.url || source)}${icon('external')}`)}</li>`).join('')}</ul>`;
  }
  function relatedTools(ids) {
    return array(ids).map(id => toolMap.get(typeof id === 'string' ? id : id.id)).filter(Boolean);
  }
  function jobDetail(id) {
    const job = jobMap.get(id);
    if (!job) return notFound('playbook');
    const suggested = relatedTools(job.tools);
    const related = array(job.related).map(id => jobMap.get(id)).filter(Boolean);
    const steps = array(job.steps);
    const verifies = array(job.verify);
    return `<a class="back-link" href="#playbooks">${icon('back')}All job playbooks</a><header class="detail-header"><div class="page-eyebrow">${icon(jobIcon(job))}${esc(job.category)} / Playbook</div><h1>${esc(job.title)}</h1><p class="summary">${esc(job.summary)}</p><div class="detail-meta"><span class="pill">${icon('clock')}${esc(job.estimatedTime || 'Work at the pace the job requires')}</span><span class="pill">${icon('layers')}${esc(job.difficulty || 'Technician guide')}</span><span class="pill">${icon('checkCircle')}${steps.length} steps · ${verifies.length} verification checks</span></div><div class="detail-actions"><button class="button primary" data-export-job="${esc(job.id)}">${icon('download')}Export checklist</button><button class="button ghost" data-print>${icon('printer')}Print guide</button><button class="button ghost" data-copy-link>${icon('link')}Copy link</button></div></header><div class="detail-layout"><div class="detail-main"><div class="callout">${icon('compass')}<div><h3>Start here</h3><p>${formatted(job.startHere || job.summary)}</p></div></div><h2 class="block-heading">${icon('route')}Work through the job</h2><div class="checklist">${steps.map((step, index) => `<section class="step"><label class="step-label"><input type="checkbox" data-check-job="${esc(job.id)}" data-check-type="step" data-check-index="${index}" data-check-title="${esc(step.title)}"${checked(job, 'step', index, step.title) ? ' checked' : ''}><span><h3><span class="step-number">${index + 1}.</span>${esc(step.title)}</h3></span></label><div class="step-detail">${formatted(step.detail)}</div>${step.command ? `<div class="command-block"><div class="command-bar"><span>Command · follow the step above</span><button class="copy-command" data-copy-command="${esc(job.id)}" data-command-index="${index}">${icon('copy')}Copy</button></div><pre><code>${esc(text(step.command))}</code></pre></div>${commandLegend(step.command)}` : ''}</section>`).join('')}</div><h2 class="block-heading">${icon('checkCircle')}Verify the result</h2><div class="verify-list">${verifies.map((item, index) => `<label class="verify-item"><input type="checkbox" data-check-job="${esc(job.id)}" data-check-type="verify" data-check-index="${index}" data-check-title="${esc(text(item))}"${checked(job, 'verify', index, text(item)) ? ' checked' : ''}><span>${formatted(item)}</span></label>`).join('')}</div>${array(job.sources).length ? `<h2 class="block-heading">${icon('book')}Official references</h2>${sourceList(job.sources)}<p class="sources-note">Reviewed ${esc(dateLabel(job.reviewedAt || reviewDate, true))}. Follow the vendor's current instructions when the product or environment differs.</p>` : ''}${related.length ? `<h2 class="block-heading">${icon('route')}Keep going</h2><div class="detail-related">${related.map(jobCard).join('')}</div>` : ''}</div><aside class="detail-aside"><section class="aside-card" id="job-progress">${progressCard(job)}</section>${suggested.length ? `<section class="aside-card"><h3>Tools for this job</h3><div class="aside-tools">${suggested.map(tool => `<a href="#tool/${encodeURIComponent(tool.id)}"><span>${esc(tool.name)}</span>${icon('arrow')}</a>`).join('')}</div></section>` : ''}${array(job.stopIf).length ? `<section class="aside-card caution-card"><h3>${icon('warning')}Pause &amp; escalate when</h3><ul class="stop-list">${array(job.stopIf).map(item => `<li>${formatted(item)}</li>`).join('')}</ul></section>` : ''}<section class="aside-card"><h3>Leave a useful handoff</h3><p>Record the symptom, findings, changes, and verification in your team's private ticket. Keep passwords, customer files, and recovery keys out of the public wiki.</p></section></aside></div>`;
  }
  function toolDetail(id) {
    const tool = toolMap.get(id);
    if (!tool) return notFound('tool');
    const related = jobs.filter(job => array(job.tools).includes(tool.id));
    const downloadLabel = tool.mode === 'built-in' ? 'Open usage guide' : tool.mode === 'web' ? 'Open official site' : 'Open official download';
    const references = [{ title: `${tool.name} · official website`, url: tool.homepage }, { title: 'Documentation & usage', url: tool.docsUrl }, { title: 'Source or upstream reference', url: tool.sourceUrl }].filter((source, i, values) => source.url && values.findIndex(value => value.url === source.url) === i);
    return `<a class="back-link" href="#tools">${icon('back')}All tools</a><header class="detail-header"><div class="detail-tool-title"><div><div class="page-eyebrow">${esc(categoryLabel(tool.category))}</div><h1>${esc(tool.name)}</h1></div></div><p class="summary">${esc(tool.summary)}</p><div class="detail-meta">${tierPill(tool.tier)}${array(tool.platforms).map(platform => `<span class="pill">${esc(platform)}</span>`).join('')}<span class="pill">${esc(modeLabel(tool.mode))}</span></div><div class="detail-actions">${external(tool.downloadUrl || tool.homepage, `${icon('external')}${downloadLabel}`, 'button primary')}${external(tool.docsUrl, `${icon('book')}Documentation`, 'button ghost')}<button class="button ghost" data-save="${esc(tool.id)}" aria-pressed="${favorites.has(tool.id)}">${icon('bookmark')}<span>${favorites.has(tool.id) ? 'Saved to your toolkit' : 'Save to your toolkit'}</span></button></div></header><div class="detail-layout"><div class="detail-main"><div class="callout">${icon('compass')}<div><h3>When to reach for it</h3><p>${formatted(tool.bestFor || tool.summary)}</p></div></div><div class="tool-info-grid"><section class="info-block"><span class="small-label">Platforms</span><p>${esc(array(tool.platforms).join(' · '))}</p></section><section class="info-block"><span class="small-label">How you use it</span><p>${esc(modeLabel(tool.mode) || 'See official documentation')}</p></section><section class="info-block"><span class="small-label">Availability &amp; pricing</span><p>${formatted(tool.pricing || 'See the official source for current terms.')}</p></section><section class="info-block"><span class="small-label">Recommendation</span><p>${esc(tierLabel(tool.tier))}${tool.tier === 'evaluate' ? ' · Validate it against your own workflow before relying on it.' : tool.tier === 'specialist' ? ' · Best for a specific diagnostic or recovery need.' : ' · A practical starting point for routine service work.'}</p></section></div>${tool.cautions ? `<div class="callout warning">${icon('info')}<div><h3>Before you use it</h3><p>${formatted(tool.cautions)}</p></div></div>` : ''}${array(tool.tags).length ? `<div class="detail-tags">${array(tool.tags).map(tag => `<span class="pill">${esc(tag)}</span>`).join('')}</div>` : ''}<h2 class="block-heading">${icon('book')}Go to the source</h2>${sourceList(references)}<p class="sources-note">Source review: ${esc(dateLabel(tool.reviewedAt || reviewDate, true))}. Download links point to official upstream pages; available versions can change after review.</p>${array(tool.provenance).length ? `<h2 class="block-heading">${icon('layers')}Why it is in the toolkit</h2><ul class="bullet-list">${array(tool.provenance).map(item => `<li>${safeUrl(item) ? external(item, `${esc(item)} ${icon('external')}`) : formatted(item)}</li>`).join('')}</ul>` : ''}</div><aside class="detail-aside"><section class="aside-card"><h3>${external(`${repo}/wiki/Tool-Selection`, 'Tool selection')}</h3><p>Compare diagnostic roles, supported systems, required access, and operating requirements before choosing a tool for the job.</p></section>${related.length ? `<section class="aside-card"><h3>Use it in a playbook</h3><div class="aside-tools">${related.map(job => `<a href="#job/${encodeURIComponent(job.id)}"><span class="aside-link-icon">${icon(jobIcon(job))}</span><span>${esc(job.title)}</span>${icon('arrow')}</a>`).join('')}</div></section>` : ''}<section class="aside-card"><h3>Contribute a correction</h3><p>Bring the official source and a short explanation of what it improves to the INTA repository.</p>${external(`${repo}/issues/new`, `Suggest an update ${icon('arrow')}`, 'button text')}</section></aside></div>`;
  }
  const projectResources = [
    { icon: 'layers', title: 'The INTA wiki', body: 'Browse the complete technician field guide on GitHub. Read a procedure, follow its sources, or propose an improvement.', label: 'Open the wiki', url: `${repo}/wiki` },
    { icon: 'usb', title: 'Rescue & boot media', body: 'Find INTA Rescue releases and the Media Creator. Keep a recovery workspace ready for systems that need attention.', label: 'View releases', url: `${repo}/releases` },
    { icon: 'book', title: 'Athena research', body: 'Explore the research collection behind specialist additions to the toolkit. Bring proven findings into a practical workflow.', label: 'Explore Athena', url: 'https://github.com/Herbertofury/Athena' },
    { icon: 'github', title: 'Source repository', body: 'The repository holds the hub, guides, source catalog, and rescue tooling. Share a reproducible fix or suggest a useful tool.', label: 'Open the repository', url: repo },
    { icon: 'activity', title: 'Build & publication checks', body: 'Follow the repository workflows and review the result of each build. Release and source history remain easy to inspect.', label: 'View workflow runs', url: `${repo}/actions` },
    { icon: 'download', title: 'Offline workspace', body: 'Save a complete copy of this workspace. The catalog and playbooks work offline; vendor downloads and external references need a connection.', label: 'Save offline workspace', action: 'offline' }
  ];
  function resources() {
    return heading('INTA references', 'Resources', 'Official references, project releases, research, and an offline copy of the workspace.') + `<div class="resource-list" role="list">${projectResources.map(resource => `<article class="resource-row" role="listitem"><span class="reference-icon">${icon(resource.icon)}</span><div><h2>${esc(resource.title)}</h2><p>${esc(resource.body)}</p></div>${resource.action ? `<button class="button ghost" data-download-offline>${esc(resource.label)} ${icon('download')}</button>` : external(resource.url, `${esc(resource.label)} ${icon('external')}`, 'button ghost')}</article>`).join('')}</div><section class="resource-note"><h2>Current sources, honest expectations.</h2><p>The source review date tells you when this collection was checked. It does not mean every program was installed or tested on every system. Official download pages are used so you can check the current release and compatibility before a job.</p><p><strong>Everyday picks</strong> are practical first choices. <strong>Specialist</strong> tools serve a more focused need. <strong>Evaluate first</strong> entries are candidates to test before they become part of a routine workflow.</p><p>Favorites and checklist progress stay in this browser. Use your team's private ticketing system for client records. ${storageAvailable ? 'Export a checklist when you need a portable handoff.' : 'Browser storage is unavailable in this session; export a checklist to preserve it.'}</p></section><section class="section-space"><div class="section-title"><div><h2>Official reference desk</h2><p>The primary references used by the job playbooks.</p></div></div><label class="input-wrap">${icon('search')}<span class="sr-only">Filter official references</span><input type="search" id="source-search" placeholder="Find a vendor, document, or reference…"></label><div class="filter-result"><span id="source-count" aria-live="polite"></span></div><div class="source-directory" id="source-directory"></div></section>`;
  }
  const allSources = (() => {
    const unique = new Map();
    jobs.forEach(job => array(job.sources).forEach(source => { if (safeUrl(source.url)) unique.set(source.url, source); }));
    return [...unique.values()].sort((a, b) => String(a.title || a.url).localeCompare(String(b.title || b.url)));
  })();
  function filterSources() {
    const directory = $('#source-directory');
    if (!directory) return;
    const query = $('#source-search').value;
    const found = allSources.filter(source => matches(`${source.title} ${source.url}`.toLowerCase(), query));
    directory.innerHTML = found.length ? found.map(source => `<div class="source-list">${external(source.url, `<span>${esc(source.title || source.url)}<span class="source-domain">${esc(new URL(source.url).hostname)}</span></span>${icon('external')}`)}</div>`).join('') : '<p class="prose">No references match. Try another search.</p>';
    $('#source-count').textContent = `${found.length} of ${allSources.length} official references`;
  }
  function notFound(type) { return empty('compass', `That ${type} is not in this edition.`, 'The link may have changed. The full library and playbooks are still available.', 'Go to the workspace', '#home'); }
  function parseRoute() {
    const raw = location.hash.slice(1) || 'home';
    const separator = raw.indexOf('?');
    const path = separator >= 0 ? raw.slice(0, separator) : raw;
    const query = separator >= 0 ? raw.slice(separator + 1) : '';
    const [name, ...parts] = path.split('/');
    let id = parts.join('/');
    try { id = decodeURIComponent(id); } catch (_) { id = ''; }
    const anchor = path.startsWith('discipline-') ? path : '';
    return { name: anchor ? 'home' : name, id, params: new URLSearchParams(query), raw, anchor };
  }
  function closeNavigation() {
    $('#sidebar').classList.remove('is-open');
    $('#mobile-menu').setAttribute('aria-expanded', 'false');
    $('#nav-backdrop').hidden = true;
  }
  function render() {
    const route = parseRoute();
    const names = { home: 'Technician home', tools: 'Tool library', playbooks: 'Job playbooks', saved: 'Saved tools', resources: 'Resources & wiki', knowledge: 'Knowledge desk', job: 'Job playbooks', tool: 'Tool library' };
    const active = route.name === 'job' ? 'playbooks' : route.name === 'tool' ? 'tools' : route.name;
    $$('#primary-nav [data-nav]').forEach(link => { link.classList.toggle('active', link.dataset.nav === active); if (link.dataset.nav === active) link.setAttribute('aria-current', 'page'); else link.removeAttribute('aria-current'); });
    $('#breadcrumb').innerHTML = `Workspace <span>/</span> <strong>${esc(names[route.name] || 'Technician home')}</strong>`;
    if (route.name === 'home') app.innerHTML = home();
    else if (route.name === 'tools') app.innerHTML = library(false, route.params);
    else if (route.name === 'saved') app.innerHTML = library(true, route.params);
    else if (route.name === 'playbooks') app.innerHTML = playbooks(route.params);
    else if (route.name === 'job') app.innerHTML = jobDetail(route.id);
    else if (route.name === 'tool') app.innerHTML = toolDetail(route.id);
    else if (route.name === 'resources') app.innerHTML = resources();
    else if (route.name === 'knowledge') app.innerHTML = knowledgePage();
    else app.innerHTML = notFound('page');
    const title = route.name === 'job' ? jobMap.get(route.id)?.title : route.name === 'tool' ? toolMap.get(route.id)?.name : names[route.name];
    document.title = `${title || 'Technician workspace'} | INTA Computer Services`;
    filterTools(false); filterJobs(false); filterSources(); linkifyProse(app); updateCounts();
    closeNavigation(); closeSearch();
    if (route.anchor && document.getElementById(route.anchor)) document.getElementById(route.anchor).scrollIntoView({ block: 'start', behavior: 'instant' });
    else if (lastRoute !== route.raw) window.scrollTo({ top: 0, behavior: 'instant' });
    if (lastRoute) $('#main').focus({ preventScroll: true });
    lastRoute = route.raw;
  }
  function updateCounts() {
    $('#tool-count').textContent = tools.length;
    $('#job-count').textContent = jobs.length;
    $('#saved-count').textContent = favorites.size;
    $$('[data-save]').forEach(button => {
      const saved = favorites.has(button.dataset.save);
      const tool = toolMap.get(button.dataset.save);
      button.setAttribute('aria-pressed', saved);
      if (button.classList.contains('save-tool')) {
        button.setAttribute('aria-label', `${saved ? 'Unsave' : 'Save'} ${tool?.name || 'tool'}`);
        button.title = saved ? 'Remove from saved tools' : 'Save this tool';
      } else if (button.querySelector('span')) button.querySelector('span').textContent = saved ? 'Saved to your toolkit' : 'Save to your toolkit';
    });
  }
  function notify(message, undo) {
    clearTimeout(toastTimer);
    const toast = $('#toast');
    toast.innerHTML = `<span>${esc(message)}</span>${undo ? '<button data-undo>Undo</button>' : ''}`;
    undoAction = undo || null;
    toast.hidden = false;
    toastTimer = setTimeout(() => { toast.hidden = true; undoAction = null; }, undo ? 9000 : 3800);
  }
  function toggleFavorite(id) {
    if (!toolMap.has(id)) return;
    const wasSaved = favorites.has(id);
    if (wasSaved) favorites.delete(id); else favorites.add(id);
    store(savedKey, [...favorites]);
    updateCounts();
    if (parseRoute().name === 'saved') {
      if (favorites.size) filterTools(); else render();
    }
    notify(`${toolMap.get(id).name} ${wasSaved ? 'removed from' : 'saved to'} your toolkit.${!storageAvailable ? ' Browser storage is unavailable; this lasts for this visit.' : ''}`);
  }
  function changeCheck(input) {
    const job = jobMap.get(input.dataset.checkJob);
    if (!job) return;
    if (!checklists[job.id] || typeof checklists[job.id] !== 'object' || Array.isArray(checklists[job.id])) checklists[job.id] = {};
    const key = checklistKey(job, input.dataset.checkType, input.dataset.checkIndex, input.dataset.checkTitle);
    checklists[job.id][key] = input.checked;
    store(checksKey, checklists);
    const state = progress(job);
    $('#progress-percentage').textContent = `${state.percentage}%`;
    $('#progress-count').textContent = `${state.done} of ${state.total} checks`;
    const track = $('.progress-track');
    track.setAttribute('aria-valuenow', state.percentage);
    track.firstElementChild.style.width = `${state.percentage}%`;
    if (!storageAvailable) $('#progress-note').textContent = 'Progress lasts for this visit because browser storage is unavailable. Export your checklist to keep it.';
    if (state.total && state.done === state.total && input.checked) notify('Checklist complete. Record the verified result in your service ticket.');
  }
  async function copy(value, success) {
    try {
      if (navigator.clipboard && window.isSecureContext) await navigator.clipboard.writeText(value);
      else {
        const textarea = document.createElement('textarea');
        textarea.value = value; textarea.style.position = 'fixed'; textarea.style.opacity = '0';
        document.body.append(textarea); textarea.select();
        const copied = document.execCommand('copy'); textarea.remove();
        if (!copied) throw new Error('Copy unavailable');
      }
      notify(success || 'Copied to clipboard.');
    } catch (_) { notify('Clipboard access is unavailable. Select the text and copy it manually.'); }
  }
  function download(filename, content, type = 'text/markdown;charset=utf-8') {
    const blob = new Blob([content], { type });
    const href = URL.createObjectURL(blob);
    const link = document.createElement('a'); link.href = href; link.download = filename;
    document.body.append(link); link.click(); link.remove();
    setTimeout(() => URL.revokeObjectURL(href), 5000);
  }
  function exportJob(id) {
    const job = jobMap.get(id);
    if (!job) return;
    const state = progress(job);
    const lines = [`# INTA · ${job.title}`, '', job.summary, '', `Source review: ${job.reviewedAt || reviewDate}`, `Exported: ${new Date().toISOString()}`, `Checklist: ${state.done}/${state.total} checks`, '', '## Start here', '', text(job.startHere), '', '## Work through the job', ''];
    array(job.steps).forEach((step, index) => {
      lines.push(`- [${checked(job, 'step', index, step.title) ? 'x' : ' '}] ${index + 1}. ${step.title}`, '', text(step.detail), '');
      if (step.command) {
        lines.push('```text', text(step.command), '```', '');
        const references = commandReferences(step.command);
        if (references.length) lines.push('Command references: ' + references.map(entry => `[${entry.name}](${entry.href})`).join(' · '), '');
      }
    });
    lines.push('## Verify the result', '');
    array(job.verify).forEach((item, index) => lines.push(`- [${checked(job, 'verify', index, text(item)) ? 'x' : ' '}] ${text(item)}`));
    lines.push('', '## Pause and escalate when', '', ...array(job.stopIf).map(item => `- ${text(item)}`), '', '## Tools', '');
    relatedTools(job.tools).forEach(tool => lines.push(`- [${tool.name}](${safeUrl(tool.downloadUrl || tool.homepage)}) — ${tool.summary}`));
    lines.push('', '## Official references', '', ...array(job.sources).map(source => `- [${source.title || source.url}](${safeUrl(source.url)})`), '', 'Record customer-specific details in your private service ticket.', '');
    download(`INTA-${job.id}-checklist.md`, lines.join('\n'));
    notify('Checklist exported with your current progress.');
  }
  function downloadOffline() {
    const clone = document.documentElement.cloneNode(true);
    const target = clone.querySelector('#app'); if (target) target.innerHTML = '';
    const resultTarget = clone.querySelector('#search-results'); if (resultTarget) { resultTarget.innerHTML = ''; resultTarget.hidden = true; }
    const toastTarget = clone.querySelector('#toast'); if (toastTarget) { toastTarget.innerHTML = ''; toastTarget.hidden = true; }
    const searchTarget = clone.querySelector('#global-search'); if (searchTarget) { searchTarget.value = ''; searchTarget.setAttribute('value', ''); searchTarget.setAttribute('aria-expanded', 'false'); }
    clone.querySelector('#sidebar')?.classList.remove('is-open');
    const backdrop = clone.querySelector('#nav-backdrop'); if (backdrop) backdrop.hidden = true;
    clone.querySelector('#mobile-menu')?.setAttribute('aria-expanded', 'false');
    download(`INTA-Technician-Workspace-${String(reviewDate).slice(0, 10)}.html`, '<!doctype html>\n' + clone.outerHTML, 'text/html;charset=utf-8');
    notify('Offline workspace saved. External downloads still need internet.');
  }
  function updateSearch() {
    const query = search.value.trim();
    if (!query) return closeSearch();
    const foundJobs = jobs.filter(job => matches(jobSearchText.get(job.id), query));
    const foundTools = tools.filter(tool => matches(toolSearchText.get(tool.id), query));
    let html = `<div class="search-summary">${foundJobs.length + foundTools.length} results for “${esc(query)}”</div>`;
    if (foundJobs.length) html += `<div class="search-label">Job playbooks · ${foundJobs.length}</div>${foundJobs.map(job => `<a class="search-result" href="#job/${encodeURIComponent(job.id)}"><span class="result-icon">${icon(jobIcon(job))}</span><span><strong>${esc(job.title)}</strong><small>${esc(job.category)}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (foundTools.length) html += `<div class="search-label">Tools · ${foundTools.length}</div>${foundTools.map(tool => `<a class="search-result" href="#tool/${encodeURIComponent(tool.id)}"><span class="result-icon">${icon('grid')}</span><span><strong>${esc(tool.name)}</strong><small>${esc(categoryLabel(tool.category))} · ${esc(array(tool.platforms).join(', '))}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (!foundJobs.length && !foundTools.length) html += '<div class="search-empty">No results yet. Try a tool name or a symptom such as “network” or “malware”.</div>';
    searchResults.innerHTML = html;
    searchResults.hidden = false;
    search.setAttribute('aria-expanded', 'true');
  }
  function closeSearch() { searchResults.hidden = true; search.setAttribute('aria-expanded', 'false'); }
  function setTheme() {
    const light = document.documentElement.dataset.theme === 'light';
    $('#theme-toggle').innerHTML = icon(light ? 'moon' : 'sun');
    $('#theme-toggle').setAttribute('aria-label', `Switch to ${light ? 'dark' : 'light'} theme`);
    $('meta[name="theme-color"]').setAttribute('content', light ? '#f7f7f8' : '#111214');
  }
  function resetToolFilters() {
    ['library-search', 'category-filter', 'platform-filter', 'tier-filter'].forEach(id => { const field = document.getElementById(id); if (field) field.value = ''; });
    filterTools();
  }
  function resetJobFilters() { $('#jobs-search').value = ''; $('#jobs-category').value = ''; filterJobs(); }

  document.addEventListener('click', event => {
    const target = event.target instanceof Element ? event.target : null;
    if (!target) return;
    if (target.closest('.skip-link')) { event.preventDefault(); $('#main').focus(); return; }
    const jobCardTarget = target.closest('[data-job-route]');
    if (jobCardTarget && !target.closest('a,button,input,select,textarea') && !window.getSelection()?.toString()) { location.hash = jobCardTarget.dataset.jobRoute; return; }
    const save = target.closest('[data-save]'); if (save) return toggleFavorite(save.dataset.save);
    const exportButton = target.closest('[data-export-job]'); if (exportButton) return exportJob(exportButton.dataset.exportJob);
    if (target.closest('[data-download-offline]')) return downloadOffline();
    if (target.closest('[data-print]')) return window.print();
    if (target.closest('[data-copy-link]')) return copy(location.href, location.protocol === 'file:' ? 'Local file link copied. Share the offline file with teammates to use it.' : 'Playbook link copied.');
    const command = target.closest('[data-copy-command]'); if (command) {
      const job = jobMap.get(command.dataset.copyCommand);
      const step = job && array(job.steps)[Number(command.dataset.commandIndex)];
      if (step) copy(text(step.command), 'Command copied. Review the step before running it.');
      return;
    }
    if (target.closest('[data-reset-filters]')) return resetToolFilters();
    if (target.closest('[data-reset-jobs]')) return resetJobFilters();
    const reset = target.closest('[data-reset-checklist]'); if (reset) {
      const id = reset.dataset.resetChecklist;
      const previous = { ...(checklists[id] || {}) };
      checklists[id] = {}; store(checksKey, checklists); render();
      notify('Fresh checklist started.', () => { checklists[id] = previous; store(checksKey, checklists); render(); notify('Previous checklist restored.'); });
      return;
    }
    if (target.closest('[data-undo]')) { const action = undoAction; if (action) action(); return; }
    if (!target.closest('.global-search-wrap')) closeSearch();
    const hashLink = target.closest('a[href^="#"]');
    if (hashLink && hashLink.getAttribute('href') === location.hash) { closeSearch(); closeNavigation(); }
  });
  document.addEventListener('input', event => {
    if (event.target.id === 'library-search') filterTools();
    else if (event.target.id === 'jobs-search') filterJobs();
    else if (event.target.id === 'source-search') filterSources();
  });
  document.addEventListener('change', event => {
    if (['category-filter', 'platform-filter', 'tier-filter'].includes(event.target.id)) filterTools();
    else if (event.target.id === 'jobs-category') filterJobs();
    else if (event.target.matches('[data-check-job]')) changeCheck(event.target);
  });
  search.addEventListener('input', updateSearch);
  search.addEventListener('focus', () => { if (search.value.trim()) updateSearch(); });
  document.addEventListener('keydown', event => {
    const tag = document.activeElement?.tagName;
    const typing = ['INPUT', 'TEXTAREA', 'SELECT'].includes(tag) || document.activeElement?.isContentEditable;
    if ((event.key === '/' && !typing) || ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k')) {
      event.preventDefault(); search.focus(); search.select();
    } else if (event.key === 'Escape') { closeSearch(); closeNavigation(); }
    else if ((event.key === 'ArrowDown' || event.key === 'Enter') && document.activeElement === search && !searchResults.hidden) {
      const first = searchResults.querySelector('a'); if (first) { event.preventDefault(); first.focus(); }
    } else if (!searchResults.hidden && searchResults.contains(document.activeElement) && ['ArrowDown', 'ArrowUp'].includes(event.key)) {
      const links = Array.from(searchResults.querySelectorAll('a'));
      const index = links.indexOf(document.activeElement);
      event.preventDefault();
      if (event.key === 'ArrowUp' && index <= 0) search.focus();
      else links[Math.max(0, Math.min(links.length - 1, index + (event.key === 'ArrowDown' ? 1 : -1)))]?.focus();
    }
  });
  $('#theme-toggle').addEventListener('click', () => {
    const next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
    document.documentElement.dataset.theme = next; store(themeKey, next); setTheme();
  });
  $('#mobile-menu').addEventListener('click', () => {
    const open = !$('#sidebar').classList.contains('is-open');
    $('#sidebar').classList.toggle('is-open', open);
    $('#mobile-menu').setAttribute('aria-expanded', open);
    $('#nav-backdrop').hidden = !open;
    if (open) $('#sidebar a').focus();
  });
  $('#nav-backdrop').addEventListener('click', closeNavigation);
  window.addEventListener('hashchange', render);
  window.addEventListener('storage', event => {
    if (event.key === savedKey) { const next = load(savedKey, []); favorites = new Set(Array.isArray(next) ? next.filter(id => toolMap.has(id)) : []); updateCounts(); if (parseRoute().name === 'saved') render(); }
    if (event.key === checksKey) { const next = load(checksKey, {}); if (next && typeof next === 'object' && !Array.isArray(next)) checklists = next; migrateChecklistState(); if (parseRoute().name === 'job') render(); }
  });
  $$('[data-icon]').forEach(element => { element.innerHTML = icon(element.dataset.icon); });
  migrateChecklistState(); setTheme(); render();
})();
