(function () {
  'use strict';

  const data = window.INTA_DATA || { tools: [], jobs: [], meta: {} };
  const tools = Array.isArray(data.tools) ? data.tools : [];
  const jobs = Array.isArray(data.jobs) ? data.jobs : [];
  const commandCenter = data.commandCenter || { commands: [], workflows: [], toolkits: [], categories: [] };
  const commandCenterCommands = new Map(array(commandCenter.commands).map(item => [item.id, item]));
  const commandCenterSearch = new Map(array(commandCenter.commands).map(item => [item.id,
    [item.title, item.summary, item.code, item.safety, item.shell, item.sourceTitle, item.category].join(' ').toLowerCase()]));
  const meta = data.meta || {};
  const toolMap = new Map(tools.map(tool => [tool.id, tool]));
  const jobMap = new Map(jobs.map(job => [job.id, job]));
  const accessRegistry = data.toolAccess && typeof data.toolAccess === 'object' ? data.toolAccess : {};
  const quickItems = (Array.isArray(data.quickStart?.items) ? data.quickStart.items : []).filter(item => item && jobMap.has(item.jobId) && (toolMap.has(item.toolId) || (item.guideOnly === true && item.toolId === null)));
  const quickChoicesById = new Map(quickItems.map(item => [item.id, normalizeQuickChoices(item)]));
  const quickByJob = new Map(), quickByTool = new Map();
  quickItems.forEach(item => {
    if (!quickByJob.has(item.jobId)) quickByJob.set(item.jobId, []);
    quickByJob.get(item.jobId).push(item);
    new Set(quickChoices(item).map(choice => choice.toolId)).forEach(toolId => {
      if (!quickByTool.has(toolId)) quickByTool.set(toolId, []);
      quickByTool.get(toolId).push(item);
    });
  });
  const fieldNotes = (Array.isArray(data.fieldNotes?.notes) ? data.fieldNotes.notes : []).filter(note => note && typeof note.id === 'string' && jobMap.has(note.jobId));
  const fieldNotesByJob = new Map();
  fieldNotes.forEach(note => {
    if (!fieldNotesByJob.has(note.jobId)) fieldNotesByJob.set(note.jobId, []);
    fieldNotesByJob.get(note.jobId).push(note);
  });
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
  // Publisher artwork is bundled once with the catalog. No runtime icon service,
  // tracking request, scroll observer, or network dependency is needed offline.
  const brandData = data.brands && typeof data.brands === 'object' ? data.brands : {};
  const brandAssets = brandData.assets || {};
  const brandRecords = array(brandData.brands);
  const brandsByTool = new Map(), brandsByName = new Map(), brandsByUrl = new Map(), brandsByHost = new Map();
  function brandUrlKey(value) {
    try { const url = new URL(value); return url.origin + url.pathname.replace(/\/$/, '') + url.search + url.hash; }
    catch (_) { return ''; }
  }
  brandRecords.forEach(brand => {
    if (!brand || !brand.id || !safeUrl(brand.downloadPage)) return;
    if (brand.toolId && !brandsByTool.has(brand.toolId)) brandsByTool.set(brand.toolId, brand);
    for (const name of array(brand.names).concat(brand.name || [])) {
      const key = String(name).toLocaleLowerCase('en-US');
      if (key && !brandsByName.has(key)) brandsByName.set(key, brand);
    }
    for (const url of array(brand.urls)) {
      const key = brandUrlKey(url);
      if (key && !brandsByUrl.has(key)) brandsByUrl.set(key, brand);
    }
    // A shared domain belongs to its website record, never whichever tool was
    // encountered first (Microsoft and GitHub each host many different tools).
    if (brand.kind === 'website') for (const host of array(brand.hosts)) brandsByHost.set(host.replace(/^www\./, '').toLowerCase(), brand);
  });
  const brandPattern = [...brandsByName.keys()].sort((a, b) => b.length - a.length)
    .map(name => name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|');
  const brandMatcher = brandPattern ? new RegExp(`(?<![\\p{L}\\p{N}_])(?:${brandPattern})(?![\\p{L}\\p{N}_])`, 'giu') : null;
  const sensitiveBrandNames = new Set(array(data.links).filter(entry => entry.caseSensitive).map(entry => entry.name.toLocaleLowerCase('en-US')));
  function mentionedBrands(value) {
    if (!brandMatcher) return [];
    brandMatcher.lastIndex = 0;
    const found = new Map();
    let match;
    while ((match = brandMatcher.exec(value))) {
      const key = match[0].toLocaleLowerCase('en-US');
      const brand = brandsByName.get(key);
      if (!brand || (sensitiveBrandNames.has(key) && !array(brand.names).includes(match[0]))) continue;
      if (!found.has(brand.id)) found.set(brand.id, brand);
    }
    return [...found.values()];
  }
  function brandFor({ toolId, name, url } = {}) {
    if (toolId && brandsByTool.has(toolId)) return brandsByTool.get(toolId);
    const key = String(name || '').trim().toLocaleLowerCase('en-US');
    if (brandsByName.has(key)) return brandsByName.get(key);
    const exact = brandsByUrl.get(brandUrlKey(url));
    if (exact) return exact;
    try { return brandsByHost.get(new URL(url).hostname.replace(/^www\./, '').toLowerCase()) || null; }
    catch (_) { return null; }
  }
  function brandIcon(brand, className = '') {
    const value = brand && brandAssets[brand.assetId];
    const src = typeof value === 'string' ? value : value?.dataUri;
    if (!src || !/^data:image\/(?:png|svg\+xml|x-icon|vnd\.microsoft\.icon|webp|jpeg|gif);/i.test(src)) return '';
    return `<img class="brand-icon${className ? ` ${esc(className)}` : ''}" src="${esc(src)}" width="20" height="20" alt="" aria-hidden="true" decoding="async" data-brand-icon="${esc(brand.id)}">`;
  }
  function brandLink(brand, label, className = '') {
    if (!brand) return esc(label);
    return `<a class="brand-link${className ? ` ${esc(className)}` : ''}" href="${esc(safeUrl(brand.downloadPage))}" target="_blank" rel="noopener noreferrer" data-brand-link="${esc(brand.id)}" data-brand-decorated="true">${brandIcon(brand)}<span>${esc(label || brand.name)}</span></a>`;
  }
  function toolName(tool, className = '') {
    const brand = brandFor({ toolId: tool.id, name: tool.name });
    if (brand) return brandLink(brand, tool.name, className);
    return external(accessInfo(tool).officialUrl, esc(tool.name), `brand-link ${className}`.trim());
  }
  function decorateBrandLinks(container) {
    if (!container || !brandRecords.length) return;
    container.querySelectorAll('a[href]:not([data-brand-decorated])').forEach(anchor => {
      anchor.dataset.brandDecorated = 'true';
      if (anchor.closest('pre,code,[data-no-link],.brand-reference,.brand')) return;
      const href = anchor.getAttribute('href') || '';
      if (!href || href === '#main') return;
      const label = anchor.textContent.trim();
      const toolId = anchor.dataset.linkKind === 'tool' ? anchor.dataset.linkId : /^#tool\/([^/?]+)/.exec(href)?.[1];
      const namedMentions = mentionedBrands(label);
      // Internal guidance is navigation, not a GitHub product mention.
      if (!namedMentions.length && /^https:\/\/(?:github\.com\/Herbertofury\/INTA|herbertofury\.github\.io\/INTA)(?:[/?#]|$)/i.test(href)) return;
      if (!namedMentions.length && anchor.matches('.access-link,.tool-links-cell > a,.workspace-options-link,.search-tool-details')) return;
      const brand = brandFor({ toolId, name: label }) || namedMentions[0] || brandFor({ url: href });
      if (!brand) return;
      const markup = brandIcon(brand);
      if (!markup) return;
      // Existing anchors keep their exact meaning and destination. Product-name
      // links created by toolName/linkifyProse already target the hosted page.
      if (namedMentions.length <= 1 && brandUrlKey(href) === brandUrlKey(brand.downloadPage)) {
        anchor.insertAdjacentHTML('afterbegin', markup);
        anchor.classList.add('brand-link'); anchor.dataset.brandLink = brand.id;
        return;
      }
      // Keep evidence, exact installers, and navigation destinations intact.
      // Their adjacent publisher icon has its own accessible download-page link.
      const wrapper = document.createElement('span');
      wrapper.className = 'brand-reference';
      if (anchor.closest('.source-list,.source-directory')) wrapper.classList.add('brand-source-reference');
      if (anchor.classList.contains('driver-shortcut') || anchor.classList.contains('job-resource-link')) wrapper.classList.add('brand-resource-reference');
      const identities = namedMentions.length ? namedMentions : [brand];
      const iconLinks = identities.map(identity => {
        const mark = brandIcon(identity);
        if (!mark) return null;
        const iconLink = document.createElement('a');
        iconLink.className = 'brand-icon-link';
        iconLink.href = identity.downloadPage;
        iconLink.target = '_blank'; iconLink.rel = 'noopener noreferrer';
        iconLink.dataset.brandLink = identity.id; iconLink.dataset.brandDecorated = 'true';
        iconLink.setAttribute('aria-label', `${identity.name}: official download or service page`);
        iconLink.title = `${identity.name} · Official page`;
        iconLink.innerHTML = mark;
        return iconLink;
      }).filter(Boolean);
      anchor.before(wrapper); wrapper.append(...iconLinks, anchor);
    });
  }
  const proseRegistry = new Map();
  array(data.links).forEach(entry => {
    if (!entry || typeof entry.name !== 'string' || !entry.name.trim()) return;
    const brand = brandFor({ toolId: entry.kind === 'tool' ? entry.id : '', name: entry.name, url: entry.url });
    const href = registryUrl(brand && ['tool', 'application', 'website', 'project'].includes(entry.kind) ? brand.downloadPage : entry.url);
    const key = entry.name.toLocaleLowerCase('en-US');
    if (href && !proseRegistry.has(key)) proseRegistry.set(key, { ...entry, href });
  });
  const prosePattern = [...proseRegistry.values()].sort((a, b) => b.name.length - a.name.length)
    .map(entry => entry.name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|');
  const proseMatcher = prosePattern ? new RegExp(`(?<![\\p{L}\\p{N}_])(?:${prosePattern})(?![\\p{L}\\p{N}_])`, 'giu') : null;
  const noProseLinks = 'a,button,code,pre,script,style,svg,textarea,select,option,kbd,samp,[data-no-link]';
  function linkifyProse(container = app) {
    if (!container) return;
    if (!proseMatcher) { decorateBrandLinks(container); return; }
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
        const brand = brandFor({ toolId: entry.kind === 'tool' ? entry.id : '', name: entry.name, url: entry.href });
        if (brand && ['tool', 'application', 'website', 'project'].includes(entry.kind)) {
          anchor.innerHTML = brandIcon(brand) + `<span>${esc(match[0])}</span>`;
          anchor.classList.add('brand-link'); anchor.dataset.brandLink = brand.id; anchor.dataset.brandDecorated = 'true';
        } else anchor.textContent = match[0];
        anchor.dataset.linkName = entry.name;
        if (entry.kind) anchor.dataset.linkKind = entry.kind;
        if (entry.id) anchor.dataset.linkId = entry.id;
        if (!entry.href.startsWith('#')) { anchor.target = '_blank'; anchor.rel = 'noopener noreferrer'; }
        fragment.append(anchor);
        last = match.index + match[0].length;
      }
      if (fragment) { fragment.append(document.createTextNode(value.slice(last))); node.replaceWith(fragment); }
    });
    decorateBrandLinks(container);
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
  function categoryLabel(value) { return value === 'online' ? 'Online tools' : categoryLabels[value] || value; }
  function modeLabel(value) { return ({ 'built-in': 'Built into the system', portable: 'Portable', installed: 'Installed application', web: 'Web application', boot: 'Bootable media' })[value] || value; }
  function accessInfo(tool) {
    const record = accessRegistry[tool.id] || {};
    const fallbackKind = tool.mode === 'built-in' ? 'builtin' : tool.mode === 'web' ? 'web' : 'release-page';
    const fallbackLabels = { builtin: 'Setup instructions', web: 'Open web tool', 'release-page': 'Download options' };
    const primary = record.primary || { kind: fallbackKind, label: fallbackLabels[fallbackKind], url: tool.downloadUrl || tool.homepage };
    return { officialUrl: record.officialUrl || tool.homepage, options: [primary, ...array(record.alternates)].filter(option => option && safeUrl(option.url)), note: record.note || '' };
  }
  function accessLabel(option) {
    return option.label || ({ file: 'Download file', store: 'Install from store', builtin: 'Setup instructions', web: 'Open web tool', account: 'Account and setup', package: 'Package instructions', source: 'Source and build', 'release-page': 'Download options' })[option.kind] || 'Acquisition instructions';
  }
  function accessActions(tool, variant = '', actionId = '', primaryOnly = false) {
    const access = accessInfo(tool);
    const selected = actionId ? access.options.filter(option => option.id === actionId) : [];
    const options = actionId ? selected : primaryOnly ? (access.options[0] ? [access.options[0]] : []) : access.options;
    return `<div class="tool-access ${esc(variant)}" data-tool-access="${esc(tool.id)}">${external(access.officialUrl, `Official page ${icon('external')}`, 'access-link official-page')}${options.map((option, index) => `<a class="access-link acquisition-link${index === 0 ? ' primary-acquisition' : ''}" data-access-tool="${esc(tool.id)}" data-acquisition-kind="${esc(option.kind)}"${option.id ? ` data-access-id="${esc(option.id)}"` : ''} data-access-role="${index === 0 ? 'primary' : 'alternate'}" href="${esc(safeUrl(option.url))}" target="_blank" rel="noopener noreferrer">${esc(accessLabel(option))} ${icon(option.kind === 'file' ? 'download' : 'external')}</a>`).join('')}</div>`;
  }
  function quickHref(item) { return `#job/${encodeURIComponent(item.jobId)}/quick/${encodeURIComponent(item.id)}`; }
  function choiceKey(choice) {
    const access = accessInfo(toolMap.get(choice.toolId));
    const selected = choice.actionId ? access.options.find(option => option.id === choice.actionId) : access.options[0];
    return `${choice.toolId}:${selected ? safeUrl(selected.url) : choice.actionId || ''}`;
  }
  function normalizeQuickChoices(item) {
    const choices = array(item.choices).filter(choice => choice && toolMap.has(choice.toolId)).map(choice => ({ ...choice, tool: toolMap.get(choice.toolId) }));
    if (toolMap.has(item.toolId)) {
      const primary = { toolId: item.toolId, actionId: item.toolAction || '', tool: toolMap.get(item.toolId), role: 'Starting tool', why: item.why, freeUse: item.freeUse, evidenceUrls: array(item.evidenceUrls), reviewedAt: data.quickStart?.reviewedAt || meta.reviewedAt };
      if (!choices.some(choice => choiceKey(choice) === choiceKey(primary))) choices.push(primary);
    }
    return choices;
  }
  function quickChoices(item) { return quickChoicesById.get(item.id) || []; }
  function choiceTerms(choice) { return [choice.tool.name, ...array(choice.tool.tags), choice.role, choice.why, choice.freeUse]; }
  function quickTerms(item) { return [item.label, item.firstStep, ...array(item.aliases), ...quickChoices(item).flatMap(choiceTerms)]; }
  function quickChoiceRows(item, variant = 'quick-access') {
    const choices = quickChoices(item);
    return `${item.guideOnly ? `<p class="no-extra-tool">No extra download needed${choices.length ? ' · Optional tools for the situation' : ''}</p>` : ''}${choices.length ? `<div class="quick-choice-list">${choices.map(choice => `<article class="quick-choice" data-quick-choice="${esc(choice.toolId)}" data-choice-action="${esc(choice.actionId || '')}"><div class="choice-description"><h4>${toolName(choice.tool)}</h4>${choice.role ? `<p class="choice-role">${formatted(choice.role)}</p>` : ''}${choice.freeUse ? `<p class="choice-use">${formatted(choice.freeUse)}</p>` : ''}</div>${accessActions(choice.tool, variant, choice.actionId, true)}</article>`).join('')}</div>` : ''}`;
  }
  function choiceSelectionNotes(item) {
    const choices = quickChoices(item);
    if (!choices.length) return '';
    return `<div class="selection-choices">${choices.map(choice => `<article class="selection-choice" data-choice-notes="${esc(choice.toolId)}"><h4>${toolName(choice.tool)}${choice.role ? ` <span>· ${formatted(choice.role)}</span>` : ''}</h4><dl><div><dt>Best fit</dt><dd>${formatted(choice.why)}</dd></div><div><dt>Use and cost</dt><dd>${formatted(choice.freeUse)}</dd></div></dl>${accessActions(choice.tool, 'choice-notes-access', choice.actionId, true)}${array(choice.evidenceUrls).length ? sourceList(array(choice.evidenceUrls).map((url, index) => ({ url, title: `Tool selection source ${index + 1} · ${safeUrl(url) ? new URL(url).hostname : ''}` }))) : ''}${choice.reviewedAt ? `<p class="sources-note">Reviewed ${esc(dateLabel(choice.reviewedAt, true))}.</p>` : ''}</article>`).join('')}</div>`;
  }
  function selectionNotes(items, title = 'Tool choice and use notes') {
    if (!items.length) return '';
    return `<section class="selection-notes"><h2 class="block-heading">${icon('info')}${esc(title)}</h2>${items.map(item => `<article class="selection-note" data-selection-note="${esc(item.id)}"><h3><a href="${quickHref(item)}">${esc(item.label)}</a></h3><dl><div><dt>Why this choice</dt><dd>${formatted(item.why)}</dd></div><div><dt>Use and cost</dt><dd>${formatted(item.freeUse)}</dd></div><div><dt>Before you act</dt><dd>${formatted(item.boundary)}</dd></div><div><dt>Selection basis</dt><dd>${formatted(item.qualityBasis)}</dd></div></dl>${array(item.evidenceUrls).length ? sourceList(array(item.evidenceUrls).map((url, index) => ({ url, title: `Selection source ${index + 1} · ${safeUrl(url) ? new URL(url).hostname : ''}` }))) : ''}${choiceSelectionNotes(item)}</article>`).join('')}</section>`;
  }
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
  function toolRow(tool, directory = false, sort = 'catalog') {
    const related = firstJobForTool.get(tool.id);
    const description = directory ? tool.bestFor || tool.summary : tool.summary;
    return `<article class="tool-row${directory ? ' directory-tool-row' : ''}" role="listitem" data-tool-id="${esc(tool.id)}"${directory ? '' : ` data-tool-card="${esc(tool.id)}"`}><div class="tool-name-cell"><h3>${toolName(tool, directory ? 'directory-tool-name' : 'tool-name')}</h3><p>${formatted(description)}</p><span class="row-meta">${directory ? esc(modeLabel(tool.mode)) : esc(categoryLabel(tool.category)) + ' · ' + esc(modeLabel(tool.mode))}</span></div><div class="tool-platform-cell" data-label="Platforms"><span class="sr-only">Platforms: </span>${esc(array(tool.platforms).join(', '))}</div><div class="tool-tier-cell" data-label="Recommendation"><span class="sr-only">Recommendation: </span>${tierPill(tool.tier)}${!directory ? toolSortSignal(tool, sort) : ''}</div><div class="tool-links-cell" data-label="Resources">${accessActions(tool, 'catalog-access')}<a href="#tool/${encodeURIComponent(tool.id)}">Details</a>${directory ? related ? `<a href="#job/${encodeURIComponent(related.id)}">Job guide ${icon('arrow')}</a>` : external(`${repo}/wiki/Tools-${encodeURIComponent(tool.category)}`, `Wiki ${icon('external')}`) : external(tool.docsUrl, `Documentation ${icon('external')}`)}</div><div class="tool-save-cell">${saveButton(tool)}</div></article>`;
  }
  function jobCard(job) {
    const href = `#job/${encodeURIComponent(job.id)}`;
    return `<article class="job-row" role="listitem" data-job-route="${esc(href)}"><span class="job-row-icon">${icon(jobIcon(job))}</span><div class="job-row-main"><h3><a href="${href}">${esc(job.title)}</a></h3><p>${esc(job.summary)}</p></div><div class="job-row-meta"><span>${esc(job.estimatedTime || `${array(job.steps).length} steps`)}</span><span>${esc(job.difficulty || job.category)}</span></div><a class="job-row-open" href="${href}" aria-label="Open ${esc(job.title)}">Open guide ${icon('arrow')}</a></article>`;
  }
  function quickTool(tool) {
    return `<div class="quick-tool">${toolName(tool)}<span>${esc(categoryLabel(tool.category))}</span>${accessActions(tool)}</div>`;
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
    { slug: 'Online-Tool-Review', icon: 'activity', title: 'Online tool review', body: 'Current service changes, practical selections, and evidence from the November 2026 research.', label: 'Read the research' },
    { href: '#online', icon: 'globe', title: 'Online tools', body: 'Open source-reviewed file, URL, network, email and website analysis services.', label: 'Browse online tools' },
    { href: '#services', icon: 'route', title: 'Service map', body: 'Browse every service area, job guide, and specific fix in one place.', label: 'Browse service areas' },
    { slug: 'Decision-Desk', icon: 'compass', title: 'Decision desk', body: 'Start with the symptom. Follow the decision paths to the right playbook and tool.', label: 'Choose a path' },
    { slug: 'Methods', icon: 'book', title: 'Methods library', body: 'Learn the repeatable techniques behind a diagnosis, a repair, and a verified recovery.', label: 'Explore methods' },
    { slug: 'Knowledge-Standards', icon: 'layers', title: 'Knowledge standards', body: 'Use one clear structure for evidence, links, diagrams, review dates, and useful guidance.', label: 'Read the standards' },
    { slug: 'Knowledge-Organization', icon: 'folder', title: 'Knowledge organization', body: 'See where guidance belongs and how its source records stay connected.', label: 'Organization guide' },
    { slug: 'Knowledge-Health', icon: 'activity', title: 'Knowledge health', body: 'Inspect coverage, open gaps, and validation results for the knowledge base.', label: 'View knowledge health' },
    { slug: 'Research-Queue', icon: 'search', title: 'Research queue', body: 'Find proposed improvements and the evidence needed before they are recommended.', label: 'Review research queue' },
    { slug: 'Templates', icon: 'file', title: 'Service templates', body: 'Turn a service visit, a tool evaluation, or a proven fix into a complete record.', label: 'Choose a template' },
    { slug: 'Contribute', icon: 'github', title: 'Contribute a fix', body: 'Bring a source, explain the improvement, and help the next technician reproduce the result.', label: 'Contribution guide' },
    { slug: 'Continuous-Improvement', icon: 'refresh', title: 'Continuous improvement', body: 'Review what changed, close gaps in a method, and use lessons from real jobs to improve the guidance.', label: 'Review the process' },
    { slug: 'Interface-Design', icon: 'grid', title: 'Interface design', body: 'Read the layout, typography, interaction, and accessibility standards for this workspace.', label: 'Design standards' },
    { slug: 'Tool-Selection', icon: 'filter', title: 'Tool selection', body: 'Compare a tool’s fit, evidence, and operating requirements before using it for a job.', label: 'Selection guide' },
    { slug: 'Current-Tool-Choices', icon: 'activity', title: 'Current tool choices', body: 'Check the latest tool review, practical alternatives, and the reasons behind current recommendations.', label: 'Review current choices' },
    { slug: 'Service-Evidence', icon: 'checkCircle', title: 'Service evidence', body: 'Keep findings, changes, and verification connected in a useful service record.', label: 'Evidence guide' }
  ];
  function knowledgeLink(resource, content, className = '') {
    return resource.href ? `<a href="${esc(resource.href)}"${className ? ` class="${esc(className)}"` : ''}>${content}</a>` : external(`${repo}/wiki/${resource.slug}`, content, className);
  }
  function knowledgeDesk(compact = false) {
    if (compact) return `<section class="home-knowledge"><h2>Knowledge desk</h2><nav aria-label="Knowledge desk references">${knowledgeResources.map(resource => knowledgeLink(resource, `${icon(resource.icon)}<span>${esc(resource.title)}</span>`)).join('')}</nav></section>`;
    return `<div class="knowledge-list" role="list">${knowledgeResources.map(resource => `<article class="knowledge-row" role="listitem"><span class="reference-icon">${icon(resource.icon)}</span><div><h2>${knowledgeLink(resource, esc(resource.title))}</h2><p>${esc(resource.body)}</p></div>${knowledgeLink(resource, `${esc(resource.label)} ${icon(resource.href ? 'arrow' : 'external')}`, 'row-action')}</article>`).join('')}</div>`;
  }
  function knowledgePage() {
    return heading('INTA field guide', 'Knowledge desk', 'Decision paths, methods, standards, and templates for repeatable service work.') + knowledgeDesk() + `<section class="document-section"><h2>From service work to shared guidance</h2><ol class="process-list"><li><strong>Choose the path.</strong> Start at the decision desk and name the problem you need to solve.</li><li><strong>Follow the method.</strong> Use the relevant playbook, primary references, and a measured next step.</li><li><strong>Verify the result.</strong> Reproduce the original task and record the outcome in a private service ticket.</li><li><strong>Improve the guide.</strong> Share the reusable lesson with a source and a clear verification.</li></ol></section>`;
  }
  let groupedToolDirectory;
  function toolDisciplines() {
    if (groupedToolDirectory) return groupedToolDirectory;
    const grouped = new Map();
    tools.forEach(tool => { if (!grouped.has(tool.category)) grouped.set(tool.category, []); grouped.get(tool.category).push(tool); });
    const preferred = ['malware', 'online', 'security', 'remote', 'windows', 'storage', 'hardware', 'network', 'boot', 'productivity', 'ai'];
    const categories = [...grouped.keys()].sort((a, b) => {
      const ai = preferred.indexOf(a), bi = preferred.indexOf(b);
      return (ai < 0 ? 999 : ai) - (bi < 0 ? 999 : bi) || categoryLabel(a).localeCompare(categoryLabel(b));
    });
    groupedToolDirectory = categories.map(category => ({ category, items: grouped.get(category).slice().sort((a, b) => a.name.localeCompare(b.name)) }));
    return groupedToolDirectory;
  }
  function compactToolDirectory() {
    const disciplines = toolDisciplines();
    return `<section class="all-tools-section" aria-labelledby="directory-heading"><div class="section-title directory-title"><div><h2 id="directory-heading">Tool directory</h2><p>${tools.length} tools across ${disciplines.length} disciplines. Open a named tool to reach its official page. Details keeps the full reference and alternate downloads.</p></div><a href="#tools">Filter the library ${icon('filter')}</a></div><nav class="discipline-jump" aria-label="Jump to a tool discipline">${disciplines.map(({category, items}) => `<a href="#discipline-${esc(category)}">${esc(categoryLabel(category))}<span>${items.length}</span></a>`).join('')}</nav><div class="directory-grid">${disciplines.map(({category, items}) => `<section class="directory-section" id="discipline-${esc(category)}" data-discipline="${esc(category)}"><header class="directory-section-header"><h2>${esc(categoryLabel(category))}<span>${items.length} ${items.length === 1 ? 'tool' : 'tools'}</span></h2>${external(`${repo}/wiki/Tools-${encodeURIComponent(category)}`, `Discipline guide ${icon('external')}`)}</header>${toolTableHeader()}<div class="directory-tool-list tool-list" role="list">${items.map(tool => toolRow(tool, true)).join('')}</div></section>`).join('')}</div><p class="directory-footnote"><strong>Everyday</strong> is a practical starting point. <strong>Specialist</strong> serves a focused need. <strong>Evaluate first</strong> identifies a research candidate that needs local validation. Check the official release, compatibility, and terms for the intended job.</p></section>`;
  }
  const scenarioEmoji = {
    'intake-triage': '🧭', 'malware-removal': '🦠', 'ransomware-response': '🚨',
    'slow-pc': '🐢', 'windows-repair': '🛠️', 'windows-update': '🔄',
    'boot-recovery': '🚑', 'failing-drive': '💾', 'disk-migration': '💽',
    'backup-restore': '🗄️', 'network-dns': '🌐', 'outage-recovery': '⚡',
    'remote-support': '🖥️', 'ad-trust': '🔑', 'outlook-email': '📧',
    'printer-repair': '🖨️', 'hardware-bsod': '🌡️', 'new-pc': '🆕',
    'client-handoff': '📋', 'boot-usb': '🔌', 'ai-triage': '🧠',
    'preventive-maintenance': '🧹'
  };
  function quickStartSection() {
    if (!quickItems.length) return '';
    return `<section class="quick-start-section" aria-labelledby="quick-start-heading"><div class="section-title quick-start-title"><div><h2 id="quick-start-heading">Fix a problem</h2><p>Start with the first action, then choose the tool for the job. Each choice has its own role.</p></div>${external(`${repo}/wiki/Free-Tool-Picks`, `Selection notes ${icon('external')}`)}</div><div class="quick-start-list" role="list">${quickItems.map(item => {
      const tool = toolMap.get(item.toolId);
      return `<article class="quick-start-row" role="listitem" data-quick-start="${esc(item.id)}" data-quick-job="${esc(item.jobId)}"${tool ? ` data-quick-tool="${esc(tool.id)}"` : ' data-guide-only="true"'}><h3><a class="quick-scenario-link" href="${quickHref(item)}"><span class="scenario-emoji" aria-hidden="true">${esc(item.icon || scenarioEmoji[item.jobId] || '🧰')}</span><span>${esc(item.label || jobMap.get(item.jobId).title)}</span></a></h3><p class="quick-first-step">${formatted(item.firstStep || item.why)}</p>${quickChoiceRows(item)}${!tool ? `<a class="access-link" href="${quickHref(item)}">Open job guide ${icon('arrow')}</a>` : ''}</article>`;
    }).join('')}</div></section>`;
  }
  function jobAnswerStrip(job, quickId = '') {
    const picks = quickByJob.get(job.id) || [];
    if (!picks.length) return '';
    const selected = picks.find(item => item.id === quickId) || picks[0];
    const ordered = [selected, ...picks.filter(item => item !== selected)];
    return `<section class="job-answer-strip" aria-label="First actions"><h2>Do this first</h2>${ordered.map(item => {
      const actions = array(item.quickSteps);
      return `<article class="quick-answer" id="quick-${esc(item.id)}" data-quick-answer="${esc(item.id)}"${item === selected ? ' data-current-case="true"' : ''}><div class="quick-answer-steps"><h3 tabindex="-1">${esc(item.icon || scenarioEmoji[item.jobId] || '🧰')} ${esc(item.label)}${item === selected && picks.length > 1 ? '<span class="current-case">Selected case</span>' : ''}</h3><p class="answer-first-step">${formatted(item.firstStep || job.startHere)}</p>${actions.length ? `<ol>${actions.map(action => `<li>${formatted(action)}</li>`).join('')}</ol>` : ''}</div><div class="quick-answer-tool">${quickChoiceRows(item, 'answer-access')}</div></article>`;
    }).join('')}</section>`;
  }
  function serviceMap() {
    const groups = array(data.layout?.groups).filter(group => Array.isArray(group.jobs));
    if (!groups.length) return empty('route', 'Service map unavailable in this edition', 'The complete job playbooks are still available.', 'Browse job playbooks', '#playbooks');
    const coveredJobs = new Set(groups.flatMap(group => group.jobs).filter(id => jobMap.has(id)));
    return heading('INTA service coverage', 'Service map', 'Choose the service area, then open a job guide or a specific fix.') + `<div class="service-map-overview"><p>${groups.length} service areas · ${coveredJobs.size} job guides · ${fieldNotes.length} specific fixes</p>${external(`${repo}/wiki/Service-Map`, `Wiki service map ${icon('external')}`)}</div><nav class="service-area-jump" aria-label="Jump to a service area">${groups.map(group => `<a href="#services?area=${encodeURIComponent(group.id)}"><span aria-hidden="true">${esc(group.icon || '')}</span>${esc(group.navigationLabel || group.title)}<span class="service-area-count">${group.jobs.filter(id => jobMap.has(id)).length}</span></a>`).join('')}</nav><div class="service-map">${groups.map(group => {
      const entries = group.jobs.map(id => jobMap.get(id)).filter(Boolean);
      const noteCount = entries.reduce((total, job) => total + (fieldNotesByJob.get(job.id)?.length || 0), 0);
      return `<section class="service-area" id="service-${esc(group.id)}" data-service-area="${esc(group.id)}" aria-labelledby="service-heading-${esc(group.id)}"><header class="service-area-header"><div><h2 id="service-heading-${esc(group.id)}" tabindex="-1"><span aria-hidden="true">${esc(group.icon || '')}</span>${esc(group.title)}</h2><p>${formatted(group.summary)}</p></div><span>${entries.length} guides · ${noteCount} specific fixes</span></header><div class="service-job-list" role="list">${entries.map(job => {
        const pick = quickByJob.get(job.id)?.[0];
        const presentation = data.layout?.jobs?.[job.id] || {};
        const notes = fieldNotesByJob.get(job.id) || [];
        const resources = array(workspaceForJob(job)?.resources);
        return `<article class="service-job-row" role="listitem" data-service-job="${esc(job.id)}"><div><h3><a href="#job/${encodeURIComponent(job.id)}"><span aria-hidden="true">${esc(presentation.navigationIcon || presentation.icon || scenarioEmoji[job.id] || '')}</span>${esc(presentation.navigationLabel || pick?.label || job.title)}</a></h3><p>${formatted(pick?.firstStep || job.startHere || job.summary)}</p><span class="service-job-coverage">${workspaceTools(job).length} tools · ${resources.length} resources${job.estimatedTime ? ` · ${esc(job.estimatedTime)}` : ''}</span>${notes.length ? `<nav class="service-note-links" aria-label="Specific fixes for ${esc(presentation.navigationLabel || job.title)}"><span>Specific fixes</span>${notes.map(note => `<a data-service-fix="${esc(note.id)}" href="${fieldNoteHref(note)}">${esc(note.title)}</a>`).join('')}</nav>` : ''}</div><nav class="service-job-actions" aria-label="Open ${esc(presentation.navigationLabel || job.title)}">${pick ? `<a href="${quickHref(pick)}">First three actions ${icon('arrow')}</a>` : ''}${notes.length ? `<a href="${jobSectionHref(job, '', 'fixes')}">${notes.length} specific fixes ${icon('arrow')}</a>` : ''}<a href="#job/${encodeURIComponent(job.id)}">Complete guide ${icon('book')}</a></nav></article>`;
      }).join('')}</div></section>`;
    }).join('')}</div>`;
  }
  function home() {
    const featuredJobs = selectFeatured(jobs, [/malware|virus/i, /slow|performance/i, /network|internet|dns/i, /backup|restore/i, /boot|bitlocker/i, /remote/i], 6);
    const kit = selectFeatured(tools, [/sysinternals/i, /rustdesk/i, /crystaldiskinfo/i, /ventoy/i], 4);
    const intake = jobs.find(job => /intake|triage/i.test(job.id + ' ' + job.title));
    return heading('INTA Computer Services', 'Technician home', 'Tools and procedures for computer repair, maintenance, and support.') + onlineStartSection() + quickStartSection() + `<div class="home-actions"><a class="button primary" href="${intake ? '#job/' + encodeURIComponent(intake.id) : '#playbooks'}">${icon('file')}Start a service visit</a><a class="button ghost" href="#checklist">${icon('checkCircle')}Open checklist</a><a class="button primary" href="#command-center">${icon('terminal')}Open Command Center</a><a class="button ghost" href="#services">${icon('route')}Service map</a><a class="button ghost" href="#playbooks">${icon('book')}${jobs.length} job playbooks</a><a class="button ghost" href="#knowledge">${icon('layers')}Knowledge desk</a><a class="button ghost" href="#resources">${icon('download')}Offline copy &amp; resources</a></div>${compactToolDirectory()}<div class="home-support"><section><div class="section-title"><h2>Common service jobs</h2><a href="#playbooks">All playbooks ${icon('arrow')}</a></div><div class="job-list" role="list">${featuredJobs.map(jobCard).join('')}</div></section><section><h2>Everyday shortcuts</h2><div class="quick-tools">${kit.map(quickTool).join('')}</div>${knowledgeDesk(true)}</section></div>`;
  }
  function selectOptions(values, label, selected = '') {
    return `<option value="">${esc(label)}</option>${[...new Set(values.filter(Boolean))].sort((a, b) => a.localeCompare(b)).map(value => `<option value="${esc(value)}"${selected === value ? ' selected' : ''}>${esc(categoryLabel(value))}</option>`).join('')}`;
  }
  const toolSearchText = new Map(tools.map(tool => [tool.id, text([tool.name, tool.category, categoryLabel(tool.category), tool.summary, tool.bestFor, tool.reputation?.github?.url, tool.reputation?.sourceforge?.url, tool.reputation?.usage?.url, ...array(tool.tags), ...array(tool.platforms), ...(quickByTool.get(tool.id) || []).flatMap(item => [item.label, item.firstStep, ...array(item.aliases), ...quickChoices(item).filter(choice => choice.toolId === tool.id).flatMap(choiceTerms)])]).toLowerCase()]));
  const workspaceSearchText = new Map(jobs.map(job => [job.id, text(workspaceSearchTerms(job)).toLowerCase()]));
  const fieldNoteSearchText = new Map(fieldNotes.map(note => [note.id, text(fieldNoteTerms(note)).toLowerCase()]));
  const jobSearchText = new Map(jobs.map(job => [job.id, text([job.title, job.category, job.summary, job.startHere, ...array(job.steps).map(step => step.title), workspaceSearchText.get(job.id), ...(fieldNotesByJob.get(job.id) || []).flatMap(fieldNoteTerms), ...(quickByJob.get(job.id) || []).flatMap(quickTerms)]).toLowerCase()]));
  const quickSearchText = new Map(quickItems.map(item => [item.id, text(quickTerms(item)).toLowerCase()]));
  function matches(haystack, query) { return query.trim().toLowerCase().split(/\s+/).filter(Boolean).every(term => haystack.includes(term)); }
  const recommendationOrder = {core: 3, specialist: 2, evaluate: 1};
  const numberFormat = new Intl.NumberFormat();
  const toolSorts = {
    catalog: {label: 'Catalog order', description: 'Original library order.'},
    name: {label: 'Name · A to Z', description: 'Alphabetical by tool name.'},
    'name-desc': {label: 'Name · Z to A', description: 'Reverse alphabetical by tool name.'},
    recommendation: {label: 'INTA recommendation', description: 'Everyday picks first, then Specialist and Evaluate first. INTA tiers are editorial, not user ratings.', metric: t => recommendationOrder[t.tier], show: t => ({label: tierLabel(t.tier)})},
    reviewed: {label: 'Recently reviewed', description: 'Most recently reviewed INTA tool records first.', metric: t => Date.parse(t.reviewedAt || ''), show: t => ({label: 'Source reviewed', date: t.reviewedAt})},
    stars: {label: 'GitHub stars', description: 'Most stars on the matched project repository. Stars measure interest, not quality.', metric: t => githubEvidence(t)?.stars, show: (t, value) => githubDisplay(t, value, 'GitHub stars')},
    rating: {label: 'User rating', description: 'Highest SourceForge user rating, then more reviews for ties. Ratings from different providers are not mixed.', metric: t => sourceForgeEvidence(t)?.rating?.value, tie: t => sourceForgeEvidence(t)?.rating?.reviews || 0, show: (t, value) => { const sf = sourceForgeEvidence(t); return {label: `${value}/${sf.rating.scale} · ${numberFormat.format(sf.rating.reviews)} SourceForge ratings`, url: sf.url, date: sf.checkedAt}; }},
    reviews: {label: 'User review count', description: 'Most SourceForge user ratings. Sample size is not sentiment.', metric: t => sourceForgeEvidence(t)?.rating?.reviews, show: (t, value) => sourceForgeDisplay(t, value, 'SourceForge ratings')},
    popularity: {label: 'Popularity · downloads', description: 'Most downloads this week as reported by SourceForge. Downloads are not unique users or endorsements.', metric: t => sourceForgeEvidence(t)?.weeklyDownloads, show: (t, value) => sourceForgeDisplay(t, value, 'SourceForge downloads this week')},
    users: {label: 'Weekly active users', description: 'Publisher-reported weekly active users for the exact product, as of the linked report. Unmeasured tools stay visible.', metric: t => usageEvidence(t)?.weeklyActiveUsers, show: (t, value) => { const usage = usageEvidence(t); return {label: `${usage.approximate ? '≈' : ''}${numberFormat.format(value)} reported weekly active users`, url: usage.url, date: usage.reportedAt}; }},
    forks: {label: 'GitHub forks', description: 'Most forks on the matched project repository.', metric: t => githubEvidence(t)?.forks, show: (t, value) => githubDisplay(t, value, 'GitHub forks')},
  };
  function githubEvidence(tool) { return tool.reputation?.github?.status === 'observed' ? tool.reputation.github : null; }
  function sourceForgeEvidence(tool) { return tool.reputation?.sourceforge?.status === 'observed' ? tool.reputation.sourceforge : null; }
  function usageEvidence(tool) { return tool.reputation?.usage?.status === 'observed' ? tool.reputation.usage : null; }
  function githubDisplay(tool, value, label) {
    const gh = githubEvidence(tool);
    return {label: `${numberFormat.format(value)} ${label}`, url: gh.url, date: gh.checkedAt};
  }
  function sourceForgeDisplay(tool, value, label) {
    const sf = sourceForgeEvidence(tool);
    return {label: `${numberFormat.format(value)} ${label}`, url: sf.url, date: sf.checkedAt};
  }
  function toolSortKey(value) {
    if (value === 'tier') return 'recommendation'; // Preserve existing shared library URLs.
    return Object.hasOwn(toolSorts, value) ? value : 'catalog';
  }
  function sortToolResults(found, key) {
    const byName = (a, b) => a.name.localeCompare(b.name) || a.id.localeCompare(b.id);
    if (key === 'catalog') return found;
    if (key === 'name' || key === 'name-desc') return found.sort((a, b) => key === 'name' ? byName(a, b) : byName(b, a));
    const sort = toolSorts[key];
    return found.sort((a, b) => {
      const lhs = sort.metric(a), rhs = sort.metric(b);
      if (!Number.isFinite(lhs)) return Number.isFinite(rhs) ? 1 : byName(a, b);
      if (!Number.isFinite(rhs)) return -1;
      return rhs - lhs || (sort.tie ? sort.tie(b) - sort.tie(a) : 0) || byName(a, b);
    });
  }
  function toolSortLabel(observation, showDate = false) {
    const label = observation.url ? external(observation.url, `${esc(observation.label)} ${icon('external')}`) : esc(observation.label);
    return label + (showDate && observation.date ? ` <time datetime="${esc(observation.date)}">${esc(dateLabel(observation.date, true))}</time>` : '');
  }
  function amountAvailability(tool, key) {
    if (key === 'reviewed') return 'Review date unavailable';
    const provider = ['stars', 'forks'].includes(key) ? tool.reputation?.github : key === 'users' ? tool.reputation?.usage : tool.reputation?.sourceforge;
    return ({
      unavailable: 'Source unavailable',
      unreported: 'Not reported by source',
      'not-applicable': 'Not applicable',
      unverified: 'No verified data yet'
    })[provider?.status] || (provider?.status === 'observed' ? 'Not reported by source' : 'No verified data yet');
  }
  function toolSortSignal(tool, key) {
    if (!toolSorts[key]?.metric || key === 'recommendation') return '';
    const spec = toolSorts[key];
    const value = spec.metric(tool);
    return `<div class="tool-sort-signal${Number.isFinite(value) ? '' : ' unknown'}">${Number.isFinite(value) ? toolSortLabel(spec.show(tool, value), true) : esc(amountAvailability(tool, key))}</div>`;
  }
  function toolEvidence(tool) {
    const metrics = [['stars', 'GitHub stars'], ['forks', 'GitHub forks'], ['rating', 'User rating'], ['popularity', 'Weekly downloads'], ['users', 'Weekly active users']];
    return `<section class="tool-evidence" aria-labelledby="tool-evidence-heading"><h2 class="block-heading" id="tool-evidence-heading">${icon('activity')}Tool reputation</h2><dl>${metrics.map(([key, title]) => {
      const spec = toolSorts[key], value = spec.metric(tool);
      return `<div><dt>${esc(title)}</dt><dd>${Number.isFinite(value) ? toolSortLabel(spec.show(tool, value), true) : esc(amountAvailability(tool, key))}</dd></div>`;
    }).join('')}</dl><p>These signals are independent, dated snapshots. Review the source and the tool’s job fit before choosing.</p></section>`;
  }
  function library(savedOnly = false, params = new URLSearchParams()) {
    const query = params.get('q') || '';
    const category = params.get('category') || '';
    const platform = params.get('platform') || '';
    const tier = params.get('tier') || '';
    const sort = toolSortKey(params.get('sort'));
    const base = savedOnly ? tools.filter(tool => favorites.has(tool.id)) : tools;
    return heading('INTA tools', savedOnly ? 'Saved tools' : 'Tool library', savedOnly ? 'Your saved tools, kept in this browser for the next service visit.' : 'Search the complete catalog by capability, platform, or recommendation.', !savedOnly) + (savedOnly && !base.length ? empty('bookmark', 'No saved tools yet', 'Use the bookmark control beside any tool to keep it here for your next visit.', 'Explore the tool library', '#tools') : `<div class="filters" id="tool-filters"><label class="input-wrap">${icon('search')}<span class="sr-only">Search tools</span><input id="library-search" type="search" placeholder="Search tools, symptoms, or capabilities…" value="${esc(query)}"></label><label class="sr-only" for="category-filter">Tool category</label><select class="filter-select" id="category-filter">${selectOptions(tools.map(tool => tool.category), 'All categories', category)}</select><label class="sr-only" for="platform-filter">Platform</label><select class="filter-select" id="platform-filter">${selectOptions(tools.flatMap(tool => array(tool.platforms)), 'All platforms', platform)}</select><label class="sr-only" for="tier-filter">Recommendation level</label><select class="filter-select" id="tier-filter"><option value="">All recommendations</option><option value="core"${tier === 'core' ? ' selected' : ''}>Everyday picks</option><option value="specialist"${tier === 'specialist' ? ' selected' : ''}>Specialist tools</option><option value="evaluate"${tier === 'evaluate' ? ' selected' : ''}>Evaluate first</option></select></div><div class="filter-result"><span id="library-result-count" aria-live="polite"></span><label class="sort-control tool-sort-control" for="tool-sort"><span>Sort by</span><select id="tool-sort" aria-describedby="tool-sort-status">${Object.entries(toolSorts).map(([id, option]) => `<option value="${id}"${sort === id ? ' selected' : ''}>${esc(option.label)}</option>`).join('')}</select></label><button data-reset-filters>Clear filters</button></div><div id="active-tool-filters" class="active-tool-filters" aria-label="Active tool filters"></div><p id="tool-sort-status" class="tool-sort-status" aria-live="polite" hidden></p><p class="directory-action-hint">${icon('external')} Tool names open their official download or service page. Use Details for platform options and guidance.</p>${toolTableHeader()}<div class="tool-list" id="tool-results" role="list" data-saved-only="${savedOnly}"></div>`);
  }
  function filterTools(linkProse = true) {
    const container = $('#tool-results');
    if (!container) return;
    const query = $('#library-search').value;
    const category = $('#category-filter').value;
    const platform = $('#platform-filter').value;
    const tier = $('#tier-filter').value;
    const sort = toolSortKey($('#tool-sort').value);
    const base = container.dataset.savedOnly === 'true' ? tools.filter(tool => favorites.has(tool.id)) : tools;
    const found = base.filter(tool => matches(toolSearchText.get(tool.id), query) && (!category || tool.category === category) && (!platform || array(tool.platforms).includes(platform)) && (!tier || tool.tier === tier));
    const filterSummary = $('#active-tool-filters');
    const activeFilters = [['library-search', query ? `Search: ${query}` : ''], ['category-filter', category ? categoryLabel(category) : ''], ['platform-filter', platform], ['tier-filter', tier ? tierLabel(tier) : '']].filter(([, value]) => value);
    if (filterSummary) {
      filterSummary.hidden = !activeFilters.length;
      filterSummary.innerHTML = activeFilters.map(([field, label]) => `<button class="filter-chip" data-clear-tool-filter="${esc(field)}" aria-label="Remove filter ${esc(label)}">${esc(label)} ${icon('close')}</button>`).join('');
    }
    container.innerHTML = found.length ? sortToolResults(found, sort).map(tool => toolRow(tool, false, sort)).join('') : `<div style="grid-column:1/-1">${empty('search', 'No tools match this combination.', 'Try a different search or clear the filters to see the full library.', 'Clear filters', '', 'reset-filters')}</div>`;
    $('#library-result-count').innerHTML = `<strong>${found.length}</strong> of ${base.length} ${base.length === 1 ? 'tool' : 'tools'}`;
    const status = $('#tool-sort-status');
    status.hidden = sort === 'catalog';
    if (!status.hidden) {
      const spec = toolSorts[sort];
      const measured = spec.metric ? found.filter(tool => Number.isFinite(spec.metric(tool))).length : found.length;
      status.textContent = spec.description + (spec.metric && measured !== found.length ? ` ${measured} of ${found.length} matching tools have this data. Unknowns are last.` : '');
    } else status.textContent = '';
    rememberFilters({ q: query, category, platform, tier, sort: sort === 'catalog' ? '' : sort });
    if (linkProse) { linkifyProse(container); linkifyProse(status); }
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
    const focus = parseRoute().name === 'checklist';
    const all = focus ? closeoutRows(job).map(({step,index})=>checked(job,'step',index,step.title)) : [...array(job.steps).map((step, i) => checked(job, 'step', i, step.title)), ...array(job.verify).map((item, i) => checked(job, 'verify', i, text(item)))];
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
  function workspaceForJob(job) {
    const workspace = data.jobWorkspaces?.jobs?.[job.id];
    return workspace && typeof workspace === 'object' ? workspace : null;
  }
  function workspaceTools(job, workspace = workspaceForJob(job), quickId = '') {
    const entries = array(workspace?.tools).filter(entry => entry && toolMap.has(entry.toolId)).map(entry => ({ ...entry, tool: toolMap.get(entry.toolId) }));
    const present = new Set(entries.map(entry => entry.toolId));
    relatedTools(job.tools).forEach(tool => {
      if (!present.has(tool.id)) { entries.push({ toolId: tool.id, tool, why: tool.bestFor || tool.summary }); present.add(tool.id); }
    });
    const picks = quickByJob.get(job.id) || [];
    const selected = picks.find(item => item.id === quickId) || picks[0];
    const choices = selected ? quickChoices(selected) : [];
    const preferred = choices.map(choice => {
      let index = entries.findIndex(entry => choiceKey(entry) === choiceKey(choice));
      if (index < 0) index = entries.findIndex(entry => entry.toolId === choice.toolId);
      const entry = index >= 0 ? entries.splice(index, 1)[0] : { toolId: choice.toolId, tool: choice.tool, why: choice.why };
      return { ...entry, actionId: choice.actionId || '', selectedCase: selected.label, choice };
    });
    return [...preferred, ...entries];
  }
  function workspaceSearchTerms(job) {
    const workspace = workspaceForJob(job);
    if (!workspace) return [];
    return [workspace.goal, workspace.startHere, ...array(workspace.tools).map(entry => entry.why), ...array(workspace.resources).flatMap(resource => [resource.label, resource.description, resource.group, resource.url]), ...array(workspace.decisions).flatMap(decision => [decision.symptom, decision.action, decision.expected]), ...array(workspace.doneWhen), ...array(workspace.escalateWhen)];
  }
  function jobSectionHref(job, quickId, section) {
    return `#job/${encodeURIComponent(job.id)}${quickId ? `/quick/${encodeURIComponent(quickId)}` : ''}?section=${encodeURIComponent(section)}`;
  }
  function resourceGroupLabel(value) {
    const label = text(value || 'Official resources').replace(/[-_]/g, ' ');
    return label.charAt(0).toUpperCase() + label.slice(1);
  }
  function contextToolAccess(entry) {
    return `${accessActions(entry.tool, 'workspace-access', entry.actionId, true)}<a class="workspace-options-link" href="#tool/${encodeURIComponent(entry.toolId)}">All options ${icon('arrow')}</a>`;
  }
  function fieldNoteTools(note, job) {
    return array(note.toolIds).map(id => {
      const tool = toolMap.get(id);
      if (!tool) return null;
      const authored = array(workspaceForJob(job)?.tools).find(entry => entry.toolId === id);
      return { toolId: id, tool, actionId: authored?.actionId || '' };
    }).filter(Boolean);
  }
  function fieldNoteTerms(note) {
    return [note.title, note.symptom, note.firstAction, ...array(note.steps), note.expected, note.ifUnresolved, ...array(note.toolIds).map(id => toolMap.get(id)?.name), ...array(note.commands).flatMap(command => [command.label, command.shell, command.purpose, command.safety, command.code, command.sourceTitle])];
  }
  function fieldNoteHref(note, quickId = '') {
    return jobSectionHref(jobMap.get(note.jobId), quickId, `fix-${note.id}`);
  }
  function fieldNotesSection(job, quickId = '') {
    const notes = fieldNotesByJob.get(job.id) || [];
    if (!notes.length) return '';
    return `<section class="field-notes" id="job-fixes" aria-labelledby="job-fixes-heading"><header class="field-notes-header"><h2 class="block-heading" id="job-fixes-heading" tabindex="-1">${icon('wrench')}Specific fixes</h2><p>${notes.length} focused cases. Match the symptom, take the first action, and check the expected result.</p></header><nav class="field-note-jump" aria-label="Specific fixes in this guide">${notes.map(note => `<a href="${fieldNoteHref(note, quickId)}">${esc(note.title)} ${icon('arrow')}</a>`).join('')}</nav>${notes.map(note => `<article class="field-note" id="job-fix-${esc(note.id)}" data-field-note="${esc(note.id)}" aria-labelledby="fix-heading-${esc(note.id)}"><h3 id="fix-heading-${esc(note.id)}" tabindex="-1"><a href="${fieldNoteHref(note, quickId)}">${esc(note.title)} ${icon('link')}</a></h3><p class="field-note-symptom"><strong>Symptom</strong> ${formatted(note.symptom)}</p><p class="field-note-first"><strong>First action</strong> ${formatted(note.firstAction)}</p><ol class="field-note-steps">${array(note.steps).map(step => `<li>${formatted(step)}</li>`).join('')}</ol><dl class="field-note-outcomes"><div><dt>Expected result</dt><dd>${formatted(note.expected)}</dd></div><div><dt>If unresolved</dt><dd>${formatted(note.ifUnresolved)}</dd></div></dl>${array(note.commands).length ? `<section class="field-note-commands" aria-label="Copy-ready commands for ${esc(note.title)}"><h4>${icon('terminal')}Quick commands</h4>${array(note.commands).map((command,index) => `<article class="field-note-command"><div class="field-note-command-heading"><div><strong>${esc(command.label)}</strong><span>${esc(command.shell)} · ${esc(command.purpose)}</span></div><button class="copy-command" data-copy-field-command="${esc(note.id)}" data-command-index="${index}">${icon('copy')}Copy</button></div><pre><code>${esc(command.code)}</code></pre><p class="command-safety">${icon('info')}<span>${formatted(command.safety)}</span></p>${command.sourceUrl ? `<p class="command-source">${external(command.sourceUrl, `Official reference: ${esc(command.sourceTitle || 'Documentation')} ${icon('external')}`)}</p>` : ''}<p class="command-copy-note">Copy only · INTA never executes commands automatically.</p></article>`).join('')}</section>` : ''}${array(note.toolIds).length ? `<section class="field-note-tools" aria-label="Tools for ${esc(note.title)}">${fieldNoteTools(note, job).map(entry => `<div class="field-note-tool" data-field-tool="${esc(entry.toolId)}"><h4>${toolName(entry.tool)}</h4><div>${contextToolAccess(entry)}</div></div>`).join('')}</section>` : ''}<footer class="field-note-footer"><h4>Official sources</h4>${sourceList(array(note.sourceUrls).map((url, index) => ({ url, title: `Source ${index + 1} · ${safeUrl(url) ? new URL(url).hostname : ''}` })))}<p>Reviewed <time datetime="${esc(note.reviewedAt || data.fieldNotes.reviewedAt || reviewDate)}">${esc(dateLabel(note.reviewedAt || data.fieldNotes.reviewedAt || reviewDate, true))}</time></p></footer></article>`).join('')}</section>`;
  }
  function jobToolPanel(job, workspace, quickId) {
    const entries = workspaceTools(job, workspace, quickId);
    const toolCount = new Set(entries.map(entry => entry.toolId)).size;
    const resources = array(workspace?.resources);
    const authoredTools = new Set(array(workspace?.tools).map(entry => entry.toolId));
    const picks = quickByJob.get(job.id) || [];
    const selected = picks.find(item => item.id === quickId) || picks[0];
    const priority = selected ? entries.filter(entry => entry.choice || (authoredTools.has(entry.toolId) && array(entry.tool.tags).includes('discovery'))) : entries;
    const drivers = resources.filter(resource => resource.kind === 'driver' || /manufacturer|oem/i.test(resource.group || ''));
    const groups = new Map();
    resources.forEach(resource => {
      const group = resource.group || 'Official resources';
      if (!groups.has(group)) groups.set(group, []);
      groups.get(group).push(resource);
    });
    return `<aside class="job-tool-panel" id="job-tools" aria-labelledby="job-tools-heading"><header class="job-tool-panel-heading"><div><h2 id="job-tools-heading" tabindex="-1">${icon('wrench')}Tools &amp; resources</h2><p>${toolCount} ${toolCount === 1 ? 'tool' : 'tools'} · ${resources.length} ${resources.length === 1 ? 'resource' : 'resources'}</p></div><a class="panel-procedure-link" href="${jobSectionHref(job, quickId, 'procedure')}" aria-label="Go to the procedure">${icon('route')}Steps</a></header><div class="job-panel-content" tabindex="0" role="region" aria-label="All tools and official resources for this job">${priority.length ? `<section class="job-priority-section" aria-labelledby="job-priority-heading"><h3 class="panel-group-heading" id="job-priority-heading">${selected?.guideOnly ? 'Optional tools for this job' : 'Choose by role'}</h3>${selected?.guideOnly ? '<p class="no-extra-tool">No extra application is required.</p>' : ''}<div class="job-priority-tools${priority.length === 1 ? ' single-priority-tool' : ''}">${priority.map(entry => `<article class="job-priority-tool" data-priority-tool="${esc(entry.toolId)}" data-priority-action="${esc(entry.actionId || '')}"><h4>${toolName(entry.tool)}</h4>${entry.choice?.role ? `<p class="choice-role">${formatted(entry.choice.role)}</p>` : ''}${entry.choice?.freeUse ? `<p class="choice-use">${formatted(entry.choice.freeUse)}</p>` : ''}${accessActions(entry.tool, 'workspace-access', entry.actionId, true)}</article>`).join('')}</div></section>` : '<p class="no-extra-tool">No extra application is required for this job.</p>'}${drivers.length ? `<nav class="job-driver-shortcuts" aria-labelledby="job-driver-shortcuts-heading"><h3 class="panel-group-heading" id="job-driver-shortcuts-heading">Get the driver</h3><div>${drivers.map(resource => external(resource.url, `<span>${esc(resource.label)}</span>${icon('external')}`, 'driver-shortcut')).join('')}</div></nav>` : ''}${entries.length ? `<section class="job-panel-tools" aria-labelledby="job-tools-list-heading"><h3 class="panel-group-heading" id="job-tools-list-heading">Supporting tools &amp; usage</h3>${entries.map((entry, index) => `<article class="job-workspace-tool" data-workspace-tool="${esc(entry.toolId)}" data-workspace-action="${esc(entry.actionId || '')}"${entries.findIndex(other => other.toolId === entry.toolId) === index ? ` data-recommended-tool="${esc(entry.toolId)}"` : ''}><h4>${toolName(entry.tool)}</h4>${entry.why ? `<p>${formatted(entry.why)}</p>` : ''}${contextToolAccess(entry)}</article>`).join('')}</section>` : ''}${[...groups.entries()].map(([group, items], index) => `<section class="job-resource-group" data-resource-group="${esc(group)}" aria-labelledby="job-resource-group-${index}"><h3 class="panel-group-heading" id="job-resource-group-${index}">${esc(resourceGroupLabel(group))}</h3><div class="job-resource-list${/driver|manufacturer|oem/i.test(group) ? ' driver-resource-list' : ''}">${items.map(resource => `<article class="job-resource" id="job-resource-${esc(resource.id)}" data-workspace-resource="${esc(resource.id)}">${external(resource.url, `<span>${esc(resource.label)}</span>${icon('external')}`, 'job-resource-link')}<p>${formatted(resource.description)}</p><div class="job-resource-meta">${resource.kind ? `<span>${esc(resourceGroupLabel(resource.kind))}</span>` : ''}${resource.sourceUrl && resource.sourceUrl !== resource.url ? external(resource.sourceUrl, 'Source', 'resource-source') : ''}${resource.checkedAt ? `<time datetime="${esc(resource.checkedAt)}">Checked ${esc(dateLabel(resource.checkedAt, true))}</time>` : ''}</div></article>`).join('')}</div></section>`).join('')}</div></aside>`;
  }
  function jobNavigation(job, workspace, quickId) {
    const sections = [['tools', 'Tools & resources'], ...(array(workspace?.decisions).length ? [['decisions', 'Decisions']] : []), ['procedure', 'Procedure'], ['verify', 'Verify'], ...(fieldNotesByJob.get(job.id)?.length ? [['fixes', 'Specific fixes']] : []), ...(array(job.sources).length ? [['references', 'Sources']] : [])];
    return `<nav class="job-section-nav" aria-label="Sections in this job guide">${sections.map(([section, label]) => `<a href="${jobSectionHref(job, quickId, section)}">${esc(label)}</a>`).join('')}</nav><nav class="job-mobile-jump" aria-label="Quick navigation in this job guide"><a href="${jobSectionHref(job, quickId, 'tools')}">${icon('wrench')}${fieldNotesByJob.get(job.id)?.length ? 'Tools' : 'Tools &amp; links'}</a><a href="${jobSectionHref(job, quickId, 'procedure')}">${icon('route')}Steps</a><a href="${jobSectionHref(job, quickId, 'verify')}">${icon('checkCircle')}Verify</a>${fieldNotesByJob.get(job.id)?.length ? `<a href="${jobSectionHref(job, quickId, 'fixes')}" aria-label="Specific fixes">${icon('life')}Fixes</a>` : ''}</nav>`;
  }
  function workspacePlan(workspace) {
    if (!workspace) return '';
    return `<section class="workspace-start" aria-labelledby="workspace-start-heading"><h2 class="block-heading" id="workspace-start-heading">${icon('compass')}Plan the job</h2>${workspace.goal ? `<p class="workspace-goal">${formatted(workspace.goal)}</p>` : ''}${workspace.startHere ? `<p>${formatted(workspace.startHere)}</p>` : ''}</section>${array(workspace.decisions).length ? `<section class="workspace-decisions" id="job-decisions" aria-labelledby="job-decisions-heading"><h2 class="block-heading" id="job-decisions-heading" tabindex="-1">${icon('route')}Choose the next action</h2><div class="decision-table-wrap"><table class="decision-table"><thead><tr><th scope="col">What you find</th><th scope="col">What to do</th><th scope="col">What to expect</th></tr></thead><tbody>${array(workspace.decisions).map(decision => `<tr><th scope="row" data-label="What you find">${formatted(decision.symptom)}</th><td data-label="What to do">${formatted(decision.action)}</td><td data-label="What to expect">${formatted(decision.expected)}</td></tr>`).join('')}</tbody></table></div></section>` : ''}`;
  }
  function closeoutRows(job) {
    return array(job.steps).map((step,index)=>({step,index})).filter(({step})=>String(step.anchor || '').startsWith('step-closeout-'));
  }
  function closeoutShortcuts(step) {
    return array(step.shortcuts).map(item=>{
      const tool = toolMap.get(item.tool);
      if (tool) {
        const access = accessInfo(tool), primary = access.options[0];
        return `<a class="button ghost" href="#tool/${encodeURIComponent(tool.id)}">${esc(item.label)}</a>${primary ? external(primary.url,esc(accessLabel(primary)), 'button ghost') : ''}`;
      }
      if (jobMap.has(item.job)) return `<a class="button ghost" href="#job/${encodeURIComponent(item.job)}">${esc(item.label)}</a>`;
      return external(item.url,esc(item.label),'button ghost');
    }).join('');
  }
  function checklistPage() {
    const job = jobMap.get('client-handoff'), rows = closeoutRows(job);
    return heading('INTA fieldwork', 'Technician checklist', 'One pass from protecting the data to handing the computer back.') +
      `<div class="detail-actions"><button class="button primary" data-export-job="${job.id}">${icon('download')}Export checklist</button><button class="button ghost" data-print>${icon('printer')}Print</button><a class="button ghost" href="#job/client-handoff">Full handoff guide</a></div>
      <div class="detail-layout"><div><div class="callout">${icon('warning')}<div><h3>Protect data before routine maintenance</h3><p>If a drive may be failing, recover first. Isolate a suspected active compromise before routine patching or optimization.</p></div></div><div class="checklist closeout-rows">
      ${rows.map(({step,index},n)=>`<section class="step"><label class="step-label"><input type="checkbox" data-check-job="${job.id}" data-check-type="step" data-check-index="${index}" data-check-title="${esc(step.title)}"${checked(job,'step',index,step.title)?' checked':''}><span><h3><span class="step-number">${n+1}.</span> ${esc(step.title)}</h3></span></label><div class="step-detail">${formatted(step.detail)}</div><div class="closeout-shortcuts" aria-label="Step shortcuts">${closeoutShortcuts(step)}</div></section>`).join('')}</div></div>
      <aside class="detail-aside"><section class="aside-card" id="job-progress">${progressCard(job)}</section><section class="aside-card"><h3>Before closing</h3><p>Mark steps after you verify them. Record deferred work in the private ticket. Never mark an unresolved critical issue as passed.</p>${external(repo+'/wiki/Job-client-handoff','Open the Wiki guide','access-link')}</section></aside></div>`;
  }
  function jobQuickCommand(job) {
    const index = array(job.steps).findIndex(step => step && step.command);
    if (index < 0) return '';
    const step = job.steps[index];
    return `<section class="job-quick-command" aria-label="Quick command"><div class="job-quick-command-heading"><div><span class="page-eyebrow">⚡ QUICK ACTION</span><h2>${esc(step.commandShell || 'Command')} · ${esc(step.title)}</h2><p>${formatted(step.commandPurpose || 'Follow the relevant playbook step before running this command.')}</p></div><button class="copy-command" data-copy-command="${esc(job.id)}" data-command-index="${index}">${icon('copy')}Copy command</button></div><pre><code>${esc(text(step.command))}</code></pre>${step.commandSafety ? `<p class="command-safety">${icon('info')}<span>${formatted(step.commandSafety)}</span></p>` : ''}${step.commandUrl ? `<p class="command-source">${external(step.commandUrl, `Official reference: ${esc(step.commandName || 'Documentation')} ${icon('external')}`)}</p>` : ''}<p class="command-copy-note">Copy only · INTA never runs commands automatically · See the matching procedure step before execution.</p></section>`;
  }
  function jobDetail(id, quickId = '') {
    const job = jobMap.get(id);
    if (!job) return notFound('playbook');
    if (quickId && !(quickByJob.get(id) || []).some(item => item.id === quickId)) return notFound('scenario');
    const workspace = workspaceForJob(job);
    const related = array(job.related).map(id => jobMap.get(id)).filter(Boolean);
    const steps = array(job.steps);
    const verifies = array(job.verify);
    const stops = [...new Set([...array(job.stopIf), ...array(workspace?.escalateWhen)].map(text))];
    return `<div class="job-page" data-job-workspace="${esc(job.id)}"><a class="back-link" href="#playbooks">${icon('back')}All job playbooks</a><div class="job-work-layout"><header class="detail-header job-header"><div class="page-eyebrow">${icon(jobIcon(job))}${esc(job.category)} / Playbook</div><h1>${esc(job.title)}</h1><p class="summary">${esc(job.summary)}</p><div class="detail-meta"><span class="pill">${icon('clock')}${esc(job.estimatedTime || 'Work at the pace the job requires')}</span><span class="pill">${icon('layers')}${esc(job.difficulty || 'Technician guide')}</span><span class="pill">${icon('checkCircle')}${steps.length} steps · ${verifies.length} verification checks</span></div><div class="detail-actions"><button class="button primary" data-export-job="${esc(job.id)}">${icon('download')}Export checklist</button><button class="button ghost" data-print>${icon('printer')}Print guide</button><button class="button ghost" data-copy-link>${icon('link')}Copy link</button></div></header>${jobNavigation(job, workspace, quickId)}${jobToolPanel(job, workspace, quickId)}<div class="detail-main job-guide-main">${jobAnswerStrip(job, quickId)}${jobQuickCommand(job)}${workspacePlan(workspace)}${stops.length ? `<section class="job-stop-conditions" id="job-escalate" aria-labelledby="job-escalate-heading"><h2 class="block-heading" id="job-escalate-heading" tabindex="-1">${icon('warning')}Pause &amp; escalate when</h2><ul class="stop-list">${stops.map(item => `<li>${formatted(item)}</li>`).join('')}</ul></section>` : ''}<div class="callout job-original-start">${icon('compass')}<div><h3>Start here</h3><p>${formatted(job.startHere || job.summary)}</p></div></div><section class="aside-card job-progress-inline" id="job-progress">${progressCard(job)}</section><section id="job-procedure" aria-labelledby="job-procedure-heading"><h2 class="block-heading" id="job-procedure-heading" tabindex="-1">${icon('route')}Work through the job</h2><div class="checklist">${steps.map((step, index) => {
      const anchor = step.anchor || `step-${String(index + 1).padStart(2, '0')}`;
      return `<section class="step" id="job-${esc(anchor)}" data-step-anchor="${esc(anchor)}"><div class="step-heading"><label class="step-label"><input type="checkbox" data-check-job="${esc(job.id)}" data-check-type="step" data-check-index="${index}" data-check-title="${esc(step.title)}"${checked(job, 'step', index, step.title) ? ' checked' : ''}><span><h3 tabindex="-1"><span class="step-number">${index + 1}.</span>${esc(step.title)}</h3></span></label><a class="step-permalink" href="${jobSectionHref(job, quickId, anchor)}" aria-label="Link to step ${index + 1}: ${esc(step.title)}">${icon('link')}</a></div><div class="step-detail">${formatted(step.detail)}</div>${step.command ? `<div class="command-block"><div class="command-bar"><span>${esc(step.commandShell || 'Command')} · ${esc(step.commandPurpose || 'Follow the step above')}</span><button class="copy-command" data-copy-command="${esc(job.id)}" data-command-index="${index}">${icon('copy')}Copy</button></div><pre><code>${esc(text(step.command))}</code></pre>${step.commandSafety ? `<p class="command-safety">${icon('info')}<span>${formatted(step.commandSafety)}</span></p>` : ''}${step.commandUrl ? `<p class="command-source">${external(step.commandUrl, `Official reference: ${esc(step.commandName || new URL(step.commandUrl).hostname)} ${icon('external')}`)}</p>` : ''}<p class="command-copy-note">Copy only. INTA never executes commands automatically.</p></div>${commandLegend(step.command)}` : ''}</section>`;
    }).join('')}</div></section><section id="job-verify" aria-labelledby="job-verify-heading"><h2 class="block-heading" id="job-verify-heading" tabindex="-1">${icon('checkCircle')}Verify the result</h2><div class="verify-list">${verifies.map((item, index) => `<label class="verify-item"><input type="checkbox" data-check-job="${esc(job.id)}" data-check-type="verify" data-check-index="${index}" data-check-title="${esc(text(item))}"${checked(job, 'verify', index, text(item)) ? ' checked' : ''}><span>${formatted(item)}</span></label>`).join('')}</div>${array(workspace?.doneWhen).length ? `<div class="workspace-done"><h3>Ready to close when</h3><ul class="bullet-list">${array(workspace.doneWhen).map(item => `<li>${formatted(item)}</li>`).join('')}</ul></div>` : ''}</section>${fieldNotesSection(job, quickId)}${selectionNotes(quickByJob.get(job.id) || [])}${array(job.sources).length ? `<section id="job-references" aria-labelledby="job-references-heading"><h2 class="block-heading" id="job-references-heading" tabindex="-1">${icon('book')}Official references</h2>${sourceList(job.sources)}<p class="sources-note">Reviewed ${esc(dateLabel(job.reviewedAt || reviewDate, true))}. Follow the vendor's current instructions when the product or environment differs.</p></section>` : ''}${related.length ? `<h2 class="block-heading">${icon('route')}Keep going</h2><div class="detail-related">${related.map(jobCard).join('')}</div>` : ''}<section class="job-handoff"><h3>Leave a useful handoff</h3><p>Record the symptom, findings, changes, and verification in your team's private ticket. Keep passwords, customer files, and recovery keys out of the public wiki.</p></section></div></div></div>`;
  }

  function toolDetail(id) {
    const tool = toolMap.get(id);
    if (!tool) return notFound('tool');
    const related = jobs.filter(job => array(job.tools).includes(tool.id));
    const access = accessInfo(tool);
    const references = [{ title: `${tool.name} · official website`, url: tool.homepage }, { title: 'Documentation & usage', url: tool.docsUrl }, { title: 'Source or upstream reference', url: tool.sourceUrl }].filter((source, i, values) => source.url && values.findIndex(value => value.url === source.url) === i);
    return `<a class="back-link" href="#tools">${icon('back')}All tools</a><header class="detail-header"><div class="detail-tool-title"><div><div class="page-eyebrow">${esc(categoryLabel(tool.category))}</div><h1>${toolName(tool, 'detail-brand-link')}</h1></div></div><p class="summary">${esc(tool.summary)}</p><div class="detail-meta">${tierPill(tool.tier)}${array(tool.platforms).map(platform => `<span class="pill">${esc(platform)}</span>`).join('')}<span class="pill">${esc(modeLabel(tool.mode))}</span></div><div class="detail-actions">${accessActions(tool, 'detail-access')}${external(tool.docsUrl, `${icon('book')}Documentation`, 'button ghost')}<button class="button ghost" data-save="${esc(tool.id)}" aria-pressed="${favorites.has(tool.id)}">${icon('bookmark')}<span>${favorites.has(tool.id) ? 'Saved to your toolkit' : 'Save to your toolkit'}</span></button></div></header><div class="detail-layout"><div class="detail-main">${access.note ? `<p class="access-note">${formatted(access.note)}</p>` : ''}<div class="callout">${icon('compass')}<div><h3>When to reach for it</h3><p>${formatted(tool.bestFor || tool.summary)}</p></div></div><div class="tool-info-grid"><section class="info-block"><span class="small-label">Platforms</span><p>${esc(array(tool.platforms).join(' · '))}</p></section><section class="info-block"><span class="small-label">How you use it</span><p>${esc(modeLabel(tool.mode) || 'See official documentation')}</p></section><section class="info-block"><span class="small-label">Availability &amp; pricing</span><p>${formatted(tool.pricing || 'See the official source for current terms.')}</p></section><section class="info-block"><span class="small-label">Recommendation</span><p>${esc(tierLabel(tool.tier))}${tool.tier === 'evaluate' ? ' · Validate it against your own workflow before relying on it.' : tool.tier === 'specialist' ? ' · Best for a specific diagnostic or recovery need.' : ' · A practical starting point for routine service work.'}</p></section></div>${tool.cautions ? `<div class="callout warning">${icon('info')}<div><h3>Before you use it</h3><p>${formatted(tool.cautions)}</p></div></div>` : ''}${array(tool.tags).length ? `<div class="detail-tags">${array(tool.tags).map(tag => `<span class="pill">${esc(tag)}</span>`).join('')}</div>` : ''}${onlineDetail(tool.id)}${toolEvidence(tool)}${selectionNotes(quickByTool.get(tool.id) || [], 'Used in these jobs')}<h2 class="block-heading">${icon('book')}Go to the source</h2>${sourceList(references)}<p class="sources-note">Source review: ${esc(dateLabel(tool.reviewedAt || reviewDate, true))}. Official pages and acquisition links are checked separately. File versions and availability can change after review.</p>${array(tool.provenance).length ? `<h2 class="block-heading">${icon('layers')}Why it is in the toolkit</h2><ul class="bullet-list">${array(tool.provenance).map(item => `<li>${safeUrl(item) ? external(item, `${esc(item)} ${icon('external')}`) : formatted(item)}</li>`).join('')}</ul>` : ''}</div><aside class="detail-aside"><section class="aside-card"><h3>${external(`${repo}/wiki/Tool-Selection`, 'Tool selection')}</h3><p>Compare diagnostic roles, supported systems, required access, and operating requirements before choosing a tool for the job.</p></section>${related.length ? `<section class="aside-card"><h3>Use it in a playbook</h3><div class="aside-tools">${related.map(job => `<a href="#job/${encodeURIComponent(job.id)}"><span class="aside-link-icon">${icon(jobIcon(job))}</span><span>${esc(job.title)}</span>${icon('arrow')}</a>`).join('')}</div></section>` : ''}<section class="aside-card"><h3>Contribute a correction</h3><p>Bring the official source and a short explanation of what it improves to the INTA repository.</p>${external(`${repo}/issues/new`, `Suggest an update ${icon('arrow')}`, 'button text')}</section></aside></div>`;
  }
  const projectResources = [
    { icon: 'layers', title: 'The INTA wiki', body: 'Browse the complete technician field guide on GitHub. Read a procedure, follow its sources, or propose an improvement.', label: 'Open the wiki', url: `${repo}/wiki` },
    { icon: 'usb', title: 'Rescue & boot media', body: 'Find INTA Rescue releases and the Media Creator. Keep a recovery workspace ready for systems that need attention.', label: 'View releases', url: `${repo}/releases` },
    { icon: 'book', title: 'Athena research', body: 'Explore the research collection behind specialist additions to the toolkit. Bring proven findings into a practical workflow.', label: 'Explore Athena', url: 'https://github.com/Herbertofury/Athena' },
    { icon: 'github', title: 'Source repository', body: 'The repository holds the hub, guides, source catalog, and rescue tooling. Share a reproducible fix or suggest a useful tool.', label: 'Open the repository', url: repo },
    { icon: 'activity', title: 'Build & publication checks', body: 'Follow the repository workflows and review the result of each build. Release and source history remain easy to inspect.', label: 'View workflow runs', url: `${repo}/actions` },
    { icon: 'download', title: 'Offline workspace', body: 'Save a complete copy of this workspace. The catalog and playbooks work offline; vendor downloads and external references need a connection.', label: 'Save offline workspace', action: 'offline' }
  ];

  function commandRiskLabel(risk) {
    return ({ 'read-only': 'Read only', 'long-running': 'Long-running', 'downloads-file': 'Downloads file', 'installs-software': 'Installs software', 'changes-system': 'Changes system' })[risk] || 'Review first';
  }
  function commandCard(item) {
    const risk = item.risk || 'changes-system';
    return `<article class="command-card" data-command-item="${esc(item.id)}" data-command-category-value="${esc(item.category)}" data-command-shell-value="${esc(item.shell)}" data-command-risk-value="${esc(risk)}">
      <header class="command-card-header"><div><p class="command-card-kicker">${esc(item.shell)} · ${esc(item.privilege)}</p><h3>${esc(item.title)}</h3></div><span class="command-risk command-risk-${esc(risk)}">${esc(commandRiskLabel(risk))}</span></header>
      <p class="command-card-summary">${formatted(item.summary)}</p>
      <div class="command-code-toolbar"><span>${esc(item.shell)}</span><button class="copy-command" data-copy-center-command="${esc(item.id)}">${icon('copy')}Copy command</button></div>
      <pre class="command-center-code"><code>${esc(item.code)}</code></pre>
      <p class="command-safety">${icon('info')}<span>${formatted(item.safety)}</span></p>
      <footer class="command-card-footer">${external(item.sourceUrl, `Official reference: ${esc(item.sourceTitle)} ${icon('external')}`)}<span>${esc(item.category)}</span></footer>
    </article>`;
  }
  function commandCenterPage(params = new URLSearchParams()) {
    const selected = params.get('category') || 'all';
    const shell = params.get('shell') || '';
    const query = params.get('q') || '';
    const categories = array(commandCenter.categories);
    const workflows = array(commandCenter.workflows);
    const toolkits = array(commandCenter.toolkits);
    const commands = array(commandCenter.commands);
    return heading('Technician workspace', 'Command Center', 'One searchable command library for PowerShell, CMD and Linux — plus trusted technician toolkits, package managers, diagnostics, repairs and software maintenance.') +
      `<section class="command-center-hero"><div><p class="page-label">DIAGNOSE · INSTALL · UPDATE · REPAIR · VERIFY</p><h2>Your command bench</h2><p>Find the exact command, understand what it changes, copy it, then inspect the result. INTA never runs a command automatically.</p></div><div class="command-center-stats"><div><strong>${commands.length}</strong><span>commands</span></div><div><strong>${workflows.length}</strong><span>workflows</span></div><div><strong>${toolkits.length}</strong><span>toolkits</span></div></div></section>
      <section class="command-center-workflows"><div class="section-heading-row"><div><p class="page-label">GUIDED JOBS</p><h2>Ready-made workflows</h2><p>Run the steps in order; preview before applying changes.</p></div></div><div class="command-workflow-grid">${workflows.map(workflow => `<article class="command-workflow-card"><p class="command-card-kicker">${esc(workflow.category)}</p><h3>${esc(workflow.title)}</h3><p>${formatted(workflow.description)}</p><ol>${array(workflow.commandIds).map(id => { const item = commandCenterCommands.get(id); return item ? `<li><a href="#command-center?category=${encodeURIComponent(item.category)}&q=${encodeURIComponent(item.title)}">${esc(item.title)}</a></li>` : ''; }).join('')}</ol><p class="command-workflow-sequence">${formatted(workflow.sequence)}</p><p class="command-safety">${icon('alert-triangle')}<span>${formatted(workflow.warning)}</span></p><button class="secondary-button" data-copy-center-workflow="${esc(workflow.id)}">${icon('copy')}Copy workflow commands</button></article>`).join('')}</div></section>
      <section class="command-center-toolkit"><div class="section-heading-row"><div><p class="page-label">TRUSTED TECHNICIAN TOOLKITS</p><h2>Powerful tools other technicians already build</h2><p>Use their maintained utilities as tools, not as mystery scripts. Verify the release and read the changes before running.</p></div></div><div class="command-toolkit-grid">${toolkits.map(toolkit => { const install = toolkit.installCommandId && commandCenterCommands.get(toolkit.installCommandId); return `<article class="command-toolkit-card"><div class="command-toolkit-top"><span class="command-toolkit-kind">${esc(toolkit.kind.replace(/-/g,' '))}</span><span>${esc(toolkit.platform)}</span></div><h3>${external(toolkit.url, esc(toolkit.name))}</h3><p>${formatted(toolkit.summary)}</p><p class="command-safety">${icon('shield-alert')}<span>${formatted(toolkit.trust)}</span></p><div class="command-toolkit-actions">${external(toolkit.repository || toolkit.url, `Project / documentation ${icon('external')}`)}${install ? `<button class="copy-command" data-copy-center-command="${esc(install.id)}">${icon('copy')}Copy setup / launch</button>` : ''}</div></article>`; }).join('')}</div></section>
      <section class="command-center-library"><div class="section-heading-row"><div><p class="page-label">COMMAND LIBRARY</p><h2>Find a command</h2><p>Search by task, symptom, command name, shell or tool. Filter by platform and risk.</p></div></div>
        <div class="command-center-filters"><label class="input-wrap">${icon('search')}<span class="sr-only">Search commands</span><input id="command-center-search" type="search" placeholder="Search diagnostics, DISM, Defender, Chocolatey, updates…" value="${esc(query)}"></label><label class="sr-only" for="command-center-shell">Shell</label><select id="command-center-shell" class="filter-select"><option value="">All shells</option>${array(commandCenter.shells).map(x => `<option value="${esc(x)}"${shell===x?' selected':''}>${esc(x)}</option>`).join('')}</select><label class="sr-only" for="command-center-risk">Impact</label><select id="command-center-risk" class="filter-select"><option value="">All impact levels</option><option value="read-only">Read only</option><option value="long-running">Long-running</option><option value="downloads-file">Downloads file</option><option value="installs-software">Installs software</option><option value="changes-system">Changes system</option></select></div>
        <nav class="command-category-tabs" aria-label="Command categories"><button class="${selected==='all'?'active':''}" data-command-category="all" aria-pressed="${selected==='all'}">All commands</button>${categories.map(cat=>`<button class="${selected===cat.id?'active':''}" data-command-category="${esc(cat.id)}" aria-pressed="${selected===cat.id}">${esc(cat.label)}</button>`).join('')}</nav>
        <div class="command-center-result" id="command-center-count" aria-live="polite"></div>
        <div class="command-center-list" id="command-center-list">${commands.map(commandCard).join('')}</div>
      </section>
      <section class="command-center-maintenance"><p class="page-label">IMPORTANT DISTINCTIONS</p><h2>Application updates are not Windows updates</h2><div class="command-maintenance-grid"><div><h3>WinGet</h3><p>Microsoft package manager; preview with <code>winget upgrade</code>, then choose whether to update all identified apps.</p></div><div><h3>Chocolatey</h3><p>Separate package database and sources. Preview with <code>choco outdated</code> and <code>choco upgrade all --noop</code> before applying.</p></div><div><h3>Scoop</h3><p>User-oriented bucket manager. Only updates apps it manages.</p></div><div><h3>Windows Update / Store</h3><p>OS servicing and Store apps need their own update paths. A clean WinGet report does not mean Windows is fully patched.</p></div></div></section>`;
  }
  function filterCommandCenter() {
    const root = $('#command-center-list');
    if (!root) return;
    const query = ($('#command-center-search')?.value || '').trim().toLocaleLowerCase();
    const shell = $('#command-center-shell')?.value || '';
    const risk = $('#command-center-risk')?.value || '';
    const active = $('.command-category-tabs [data-command-category].active')?.dataset.commandCategory || 'all';
    let visible = 0;
    root.querySelectorAll('[data-command-item]').forEach(card => {
      const item = commandCenterCommands.get(card.dataset.commandItem);
      const haystack = commandCenterSearch.get(item?.id) || '';
      const show = (!query || haystack.includes(query)) && (!shell || item?.shell===shell) && (!risk || item?.risk===risk) && (active==='all' || item?.category===active);
      card.hidden = !show;
      if (show) visible++;
    });
    const count=$('#command-center-count');
    if(count) count.textContent = `${visible} of ${commandCenter.commands.length} commands`;
  }
  function resources() {
    return heading('INTA references', 'Resources', 'Official references, project releases, research, and an offline copy of the workspace.') + `<div class="resource-list" role="list">${projectResources.map(resource => `<article class="resource-row" role="listitem"><span class="reference-icon">${icon(resource.icon)}</span><div><h2>${esc(resource.title)}</h2><p>${esc(resource.body)}</p></div>${resource.action ? `<button class="button ghost" data-download-offline>${esc(resource.label)} ${icon('download')}</button>` : external(resource.url, `${esc(resource.label)} ${icon('external')}`, 'button ghost')}</article>`).join('')}</div><section class="resource-note"><h2>Current sources, honest expectations.</h2><p>The source review date tells you when this collection was checked. It does not mean every program was installed or tested on every system. Official download pages are used so you can check the current release and compatibility before a job.</p><p><strong>Everyday picks</strong> are practical first choices. <strong>Specialist</strong> tools serve a more focused need. <strong>Evaluate first</strong> entries are candidates to test before they become part of a routine workflow.</p><p>Favorites and checklist progress stay in this browser. Use your team's private ticketing system for client records. ${storageAvailable ? 'Export a checklist when you need a portable handoff.' : 'Browser storage is unavailable in this session; export a checklist to preserve it.'}</p></section><section class="section-space"><div class="section-title"><div><h2>Official reference desk</h2><p>The primary references used by the job playbooks.</p></div></div><label class="input-wrap">${icon('search')}<span class="sr-only">Filter official references</span><input type="search" id="source-search" placeholder="Find a vendor, document, or reference…"></label><div class="filter-result"><span id="source-count" aria-live="polite"></span></div><div class="source-directory" id="source-directory"></div></section>`;
  }
  const allSources = (() => {
    const unique = new Map();
    jobs.forEach(job => array(job.sources).forEach(source => { if (safeUrl(source.url)) unique.set(source.url, source); }));
    array(data.onlineTools?.tools).forEach(entry => array(entry.sources).forEach(source => { if (safeUrl(source.url)) unique.set(source.url, source); }));
    return [...unique.values()].sort((a, b) => String(a.title || a.url).localeCompare(String(b.title || b.url)));
  })();
  function filterSources(linkProse = true) {
    const directory = $('#source-directory');
    if (!directory) return;
    const query = $('#source-search').value;
    const found = allSources.filter(source => matches(`${source.title} ${source.url}`.toLowerCase(), query));
    directory.innerHTML = found.length ? found.map(source => `<div class="source-list">${external(source.url, `<span>${esc(source.title || source.url)}<span class="source-domain">${esc(new URL(source.url).hostname)}</span></span>${icon('external')}`)}</div>`).join('') : '<p class="prose">No references match. Try another search.</p>';
    if (linkProse) linkifyProse(directory);
    $('#source-count').textContent = `${found.length} of ${allSources.length} official references`;
  }
  const onlineData = data.onlineTools || { groups: [], tools: [], workflows: [], featured: [] };
  const onlineGroups = new Map(array(onlineData.groups).map(group => [group.id, group]));
  const onlineEntries = array(onlineData.tools).filter(entry => toolMap.has(entry.toolId));
  const onlineByTool = new Map(onlineEntries.map(entry => [entry.toolId, entry]));
  const onlineWorkflows = array(onlineData.workflows);
  const onlineWorkflowById = new Map(onlineWorkflows.map(flow => [flow.id, flow]));
  const onlineWorkflowTools = new Map(onlineWorkflows.map(flow => [flow.id, new Set(array(flow.toolIds))]));
  const onlineGroupCounts = new Map(array(onlineData.groups).map(group => [group.id, 0]));
  onlineEntries.forEach(entry => onlineGroupCounts.set(entry.group, (onlineGroupCounts.get(entry.group) || 0) + 1));
  const onlineIndex = new Map(onlineEntries.map(entry => {
    const tool = toolMap.get(entry.toolId);
    return [entry.toolId, text([tool.name, tool.summary, tool.bestFor, ...array(tool.tags),
      onlineGroups.get(entry.group)?.label, entry.firstAction, entry.expectedResult, entry.account,
      entry.sharing, tool.pricing]).toLowerCase()];
  }));

  function onlineTask(params) {
    return onlineWorkflowById.get(params.get('task')) || onlineWorkflows[0];
  }
  function onlineHref(params = new URLSearchParams(), changes = {}) {
    const next = new URLSearchParams(params);
    Object.entries(changes).forEach(([key, value]) => { if (value) next.set(key, value); else next.delete(key); });
    const query = next.toString();
    return `#online${query ? '?' + query : ''}`;
  }
  function onlineServiceHref(toolId) {
    return onlineHref(new URLSearchParams(), { tool: toolId });
  }

  function onlineStartSection() {
    if (!onlineEntries.length) return '';
    const featured = array(onlineData.featured).map(id => toolMap.get(id)).filter(Boolean);
    return `<section class="online-start" aria-labelledby="online-start-heading"><div class="section-title"><div><h2 id="online-start-heading">Online tools</h2><p>Check a file, investigate a link, or diagnose a website.</p></div><a class="button ghost" href="#online">Browse ${onlineEntries.length} online tools ${icon('external')}</a></div><div class="online-shortcuts">${featured.map(tool => `<div>${toolName(tool)}<span>${esc(onlineByTool.get(tool.id)?.shortRole || onlineGroups.get(onlineByTool.get(tool.id)?.group)?.label || tool.summary)}</span></div>`).join('')}</div><nav class="online-task-shortcuts" aria-label="Online tool tasks">${array(onlineData.workflows).map(flow => `<a href="#online?task=${encodeURIComponent(flow.id)}">${esc(flow.title)}</a>`).join('')}</nav></section>`;
  }

  function onlineWorkflow(workflow) {
    if (!workflow) return '';
    return `<section class="online-workflow" aria-labelledby="online-workflow-heading" data-online-workflow="${esc(workflow.id)}"><div><h2 id="online-workflow-heading">${esc(workflow.title)}</h2><p>${formatted(workflow.summary)}</p><ol>${array(workflow.steps).map(step => `<li>${formatted(step)}</li>`).join('')}</ol><a href="#job/${encodeURIComponent(workflow.jobId)}">Complete job guide ${icon('book')}</a></div><div class="online-workflow-kit" aria-label="Tools for this task">${array(workflow.toolIds).map(id => toolMap.get(id)).filter(Boolean).map(tool => `<div>${toolName(tool)}<p>${esc(onlineByTool.get(tool.id)?.shortRole || tool.bestFor)}</p>${accessActions(tool, 'online-access', '', true)}<a class="online-notes-link" href="${onlineServiceHref(tool.id)}" data-online-inspect="${esc(tool.id)}">Access &amp; submission behavior ${icon('arrow')}</a></div>`).join('')}</div></section>`;
  }

  function onlineServiceRow(entry) {
    const tool = toolMap.get(entry.toolId);
    return `<article class="online-tool-row" id="online-service-${esc(tool.id)}" role="listitem" data-online-tool="${esc(tool.id)}" aria-labelledby="online-heading-${esc(tool.id)}"><div class="online-tool-identity"><span class="row-meta">${esc(onlineGroups.get(entry.group)?.label || entry.group)}</span><h3 id="online-heading-${esc(tool.id)}" tabindex="-1">${toolName(tool)}</h3><p>${formatted(tool.summary)}</p><div class="online-service-actions">${accessActions(tool, 'online-access', '', true)}<a href="#tool/${encodeURIComponent(tool.id)}">Full reference</a><a href="${onlineServiceHref(tool.id)}" aria-label="Link to ${esc(tool.name)} in the online directory">Service link ${icon('link')}</a>${saveButton(tool)}</div></div><div class="online-tool-guidance"><p><strong>Start here</strong> ${formatted(entry.firstAction)}</p><p><strong>Read the result</strong> ${formatted(entry.expectedResult)}</p><dl class="online-access-notes"><div><dt>Access</dt><dd>${formatted(tool.pricing)} ${formatted(entry.account)}</dd></div><div><dt>Submission behavior</dt><dd>${formatted(entry.sharing)}</dd></div></dl><details class="online-evidence"><summary>Official evidence · reviewed ${esc(dateLabel(entry.reviewedAt))}</summary><ul>${array(entry.sources).map(source => `<li>${external(source.url, esc(source.title))}<span>${formatted(source.evidence)}</span></li>`).join('')}</ul></details></div></article>`;
  }

  function online(params = new URLSearchParams()) {
    const task = onlineTask(params);
    const group = onlineGroups.has(params.get('group')) ? params.get('group') : '';
    return heading('INTA web toolkit', 'Online tools', `${onlineEntries.length} source-reviewed services. Open the right site, follow the first action, and understand the result.`)
      + `<nav class="online-task-nav" aria-label="Choose an online task">${onlineWorkflows.map(flow => `<a class="${flow.id === task?.id ? 'selected' : ''}" data-online-task="${esc(flow.id)}" href="${esc(onlineHref(params, { task: flow.id, tool: '' }))}"${flow.id === task?.id ? ' aria-current="true"' : ''}>${esc(flow.title)}</a>`).join('')}</nav>`
      + onlineWorkflow(task)
      + `<section class="online-directory" aria-labelledby="online-directory-heading"><div class="section-title"><div><h2 id="online-directory-heading">All online services</h2><p>Search names, tasks, account requirements, or capabilities. Clear filters to return to the complete directory.</p></div>${external(`${repo}/wiki/Online-Tools`, 'Read the Wiki')}</div><div class="filters online-filters"><label class="input-wrap">${icon('search')}<span class="sr-only">Search online tools</span><input id="online-search" type="search" placeholder="Try hash, phishing, DMARC, TLS, CVE…" value="${esc(params.get('q') || '')}"></label><label class="sr-only" for="online-group">Online tool category</label><select class="filter-select" id="online-group"><option value="">All ${onlineGroups.size} groups</option>${array(onlineData.groups).map(item => `<option value="${esc(item.id)}"${item.id === group ? ' selected' : ''}>${esc(item.label)}</option>`).join('')}</select><button data-reset-online>Clear filters</button></div><div class="online-scope-filters" role="group" aria-label="Narrow the online directory"><label><input type="checkbox" id="online-task-only"${params.get('scope') === 'task' ? ' checked' : ''}> Tools for this task</label><label><input type="checkbox" id="online-saved-only"${params.get('saved') === '1' ? ' checked' : ''}> Saved only</label><span>Both options work with search and group filters.</span></div><nav class="online-group-nav" aria-label="Filter online tools by task"><button data-online-group="" aria-pressed="${!group}">All groups <span>${onlineEntries.length}</span></button>${array(onlineData.groups).map(item => `<button data-online-group="${esc(item.id)}" aria-pressed="${group === item.id}">${esc(item.label)} <span>${onlineGroupCounts.get(item.id)}</span></button>`).join('')}</nav><div class="filter-result"><span id="online-result-count" aria-live="polite"></span><span>Sources reviewed ${esc(dateLabel(onlineData.reviewedAt, true))}</span></div><div id="online-results" class="online-tool-list" role="list"></div></section>`;
  }

  function filterOnline(linkProse = true) {
    const container = $('#online-results');
    if (!container) return;
    const query = $('#online-search').value;
    const group = $('#online-group').value;
    const taskOnly = $('#online-task-only').checked;
    const savedOnly = $('#online-saved-only').checked;
    const route = parseRoute();
    const task = onlineTask(route.params);
    const taskTools = onlineWorkflowTools.get(task?.id);
    const entries = onlineEntries.filter(entry => (!group || entry.group === group)
      && (!taskOnly || taskTools?.has(entry.toolId)) && (!savedOnly || favorites.has(entry.toolId))
      && matches(onlineIndex.get(entry.toolId), query));
    // Keep expanded evidence, keyboard focus and decoded icons when the query
    // changes but the complete ordered result set is identical.
    const resultKey = entries.map(entry => entry.toolId).join('|');
    const changed = container.dataset.resultKey !== resultKey;
    if (changed) {
      container.innerHTML = entries.length ? entries.map(onlineServiceRow).join('') : `<div class="empty-state"><h3>No matching online tools</h3><p>Try a broader task or clear the filters to see every service.</p><button data-reset-online>Show all online tools</button></div>`;
      container.dataset.resultKey = resultKey;
    }
    $('#online-result-count').textContent = `${entries.length} of ${onlineEntries.length} online services${taskOnly ? ` · Task: ${task?.title || ''}` : ''}${savedOnly ? ' · Saved tools' : ''}`;
    $$('[data-online-group]').forEach(button => button.setAttribute('aria-pressed', button.dataset.onlineGroup === group));
    if (route.name === 'online') {
      const params = route.params;
      for (const [key, value] of [['q', query], ['group', group], ['scope', taskOnly ? 'task' : ''], ['saved', savedOnly ? '1' : '']]) { if (value) params.set(key, value); else params.delete(key); }
      if (linkProse) params.delete('tool');
      const next = onlineHref(params);
      if (location.hash !== next) history.replaceState(null, '', next);
      $$('[data-online-task]').forEach(link => { link.href = onlineHref(params, { task: link.dataset.onlineTask, tool: '' }); });
    }
    if (linkProse) { if (changed) linkifyProse(container); updateCounts(); }
  }

  function onlineDetail(toolId) {
    const entry = onlineByTool.get(toolId);
    if (!entry) return '';
    return `<section class="document-section online-detail"><h2>Use this online service</h2><p><strong>First action:</strong> ${formatted(entry.firstAction)}</p><p><strong>Expected result:</strong> ${formatted(entry.expectedResult)}</p><p><strong>Account:</strong> ${formatted(entry.account)}</p><p><strong>Submission behavior:</strong> ${formatted(entry.sharing)}</p><p><a href="#online?group=${encodeURIComponent(entry.group)}">Compare tools for this task</a></p>${sourceList(entry.sources)}</section>`;
  }

  const newsGroups = {
    advisory: ['Official advisories', 'Government alerts and exposure triage'],
    compliance: ['Standards & compliance', 'Frameworks, assessment and program references'],
    vendor: ['Vendor security', 'Product updates, mitigations and release guidance'],
    reporting: ['Independent reporting', 'Original reporting and technical analysis; verify before acting'],
    community: ['Community & social', 'Professional discussion and public accounts; not official alerts'],
    status: ['Service health', 'Publisher status and service incident pages']
  };
  function newsPage() {
    const catalog = data.news;
    const briefing = (catalog.briefing || []).map(item => {
      const src = catalog.sources.find(s => s.id === item.sourceId);
      return `<article class="news-brief"><span>${brandLink(brandFor({name: src.name, url: src.url}), src.name)} · <time datetime="${esc(item.date)}">${esc(dateLabel(item.date,true))}</time></span><h3>${external(item.url,esc(item.title))}</h3><p>${esc(item.description)}</p></article>`;
    }).join('');
    return heading('Technician intelligence', 'Technician News', 'Find official alerts first, then compare vendor updates, reporting, and community perspectives.', false)
      + `<div class="news-banner"><div><strong>Start with the source</strong><p>Look up the affected version in a vendor bulletin before changing a customer system. Treat social and reporting links as leads.</p></div><span>Catalog reviewed <time datetime="${esc(catalog.reviewedAt)}">${esc(dateLabel(catalog.reviewedAt,true))}</time></span></div>`
      + (briefing ? `<section aria-labelledby="news-brief-title"><h2 id="news-brief-title">Selected briefing</h2><p class="news-muted">Dated editorial snapshot, not a live feed. Open the publisher for the latest changes.</p><div class="news-grid">${briefing}</div></section>` : '')
      + `<section aria-labelledby="news-sources-title"><h2 id="news-sources-title">Source directory</h2><p class="news-muted">The directory works offline. Open publisher pages for current updates; saved sources are local to this browser.</p><div class="news-controls"><label>Search sources<input id="news-search" type="search" placeholder="CISA, Microsoft, NIST…"></label><label>Category<select id="news-category"><option value="">All categories</option>${Object.entries(newsGroups).map(([id,[label]])=>`<option value="${id}">${esc(label)}</option>`).join('')}</select></label><label class="news-saved"><input id="news-saved" type="checkbox"> Saved only</label></div><p id="news-count" aria-live="polite"></p><div id="news-directory"></div></section>`;
  }
  function bindNews() {
    const search = $('#news-search');
    if (!search) return;
    const directory = $('#news-directory');
    const category = $('#news-category');
    const onlySaved = $('#news-saved');
    const key = 'inta:news-follows:v1';
    const sources = data.news.sources;
    const validIds = new Set(sources.map(x => x.id));
    const previous = load(key, []);
    const saved = new Set(Array.isArray(previous) ? previous.filter(id => validIds.has(id)) : []);
    const normalize = value => value.toLocaleLowerCase().normalize('NFKD').replace(/[^a-z0-9]+/g,' ').trim();
    const indexes = sources.map(source => [source, normalize([source.name,source.description,source.url,newsGroups[source.category][0]].join(' '))]);
    const paint = () => {
      const words = normalize(search.value).split(' ').filter(Boolean);
      let count = 0;
      directory.innerHTML = Object.entries(newsGroups).map(([id,[label,description]]) => {
        if (category.value && category.value !== id) return '';
        const rows = indexes.filter(([s, searchable]) => s.category === id && (!onlySaved.checked || saved.has(s.id)) && words.every(w=>searchable.includes(w)));
        if (!rows.length) return '';
        count += rows.length;
        return `<section class="news-source-group" aria-labelledby="news-group-${id}"><h3 id="news-group-${id}">${esc(label)}</h3><p class="news-muted">${esc(description)}</p><div class="news-grid">${rows.map(([s]) => `<article class="news-card" data-news-id="${esc(s.id)}"><div><h4>${external(s.url,esc(s.name))}</h4><p>${esc(s.description)}</p></div><div class="news-actions"><button class="button" type="button" data-news-save="${esc(s.id)}" aria-label="Save ${esc(s.name)}" aria-pressed="${saved.has(s.id)}">${saved.has(s.id) ? 'Saved' : 'Save'}</button>${s.feed ? external(s.feed,'Feed') : ''}</div></article>`).join('')}</div></section>`;
      }).join('') || '<p>No sources match. Try a broader search or turn off Saved only.</p>';
      $('#news-count').textContent = `${count} of ${sources.length} sources`;
      linkifyProse(directory);
    };
    search.addEventListener('input',paint);
    category.addEventListener('change',paint);
    onlySaved.addEventListener('change',paint);
    directory.addEventListener('click',event => {
      const button = event.target.closest('[data-news-save]');
      const id = button?.dataset.newsSave;
      if (!validIds.has(id)) return;
      if (saved.has(id)) saved.delete(id); else saved.add(id);
      store(key,[...saved]);
      paint();
      (directory.querySelector(`[data-news-save="${id}"]`) || onlySaved).focus();
    });
    paint();
  }
  function notFound(type) { return empty('compass', `That ${type} is not in this edition.`, 'The link may have changed. The full library and playbooks are still available.', 'Go to the workspace', '#home'); }
  function parseRoute() {
    const raw = location.hash.slice(1) || 'home';
    const separator = raw.indexOf('?');
    const path = separator >= 0 ? raw.slice(0, separator) : raw;
    const query = separator >= 0 ? raw.slice(separator + 1) : '';
    const [name, ...parts] = path.split('/');
    let id = parts.join('/');
    let quickId = '';
    if (name === 'job' && parts.length === 3 && parts[1] === 'quick') { id = parts[0]; quickId = parts[2]; }
    try { id = decodeURIComponent(id); quickId = decodeURIComponent(quickId); } catch (_) { id = ''; quickId = ''; }
    const anchor = path.startsWith('discipline-') ? path : '';
    return { name: anchor ? 'home' : name, id, quickId, params: new URLSearchParams(query), raw, anchor };
  }
  function closeNavigation() {
    $('#sidebar').classList.remove('is-open');
    $('#mobile-menu').setAttribute('aria-expanded', 'false');
    $('#nav-backdrop').hidden = true;
  }
  function render() {
    const route = parseRoute();
    const names = { home: 'Technician home', online: 'Online tools', checklist: 'Technician checklist', news: 'Technician News', 'command-center': 'Command Center', services: 'Service map', tools: 'Tool library', playbooks: 'Job playbooks', saved: 'Saved tools', resources: 'Resources & wiki', knowledge: 'Knowledge desk', job: 'Job playbooks', tool: 'Tool library' };
    const active = route.name === 'job' ? 'playbooks' : route.name === 'tool' ? 'tools' : route.name;
    $$('#primary-nav [data-nav]').forEach(link => { link.classList.toggle('active', link.dataset.nav === active); if (link.dataset.nav === active) link.setAttribute('aria-current', 'page'); else link.removeAttribute('aria-current'); });
    $('#breadcrumb').innerHTML = `Workspace <span>/</span> <strong>${esc(names[route.name] || 'Technician home')}</strong>`;
    if (route.name === 'home') app.innerHTML = home();
    else if (route.name === 'online') app.innerHTML = online(route.params);
    else if (route.name === 'checklist') app.innerHTML = checklistPage();
    else if (route.name === 'news') app.innerHTML = newsPage();
    else if (route.name === 'command-center') app.innerHTML = commandCenterPage(route.params);
    else if (route.name === 'services') app.innerHTML = serviceMap();
    else if (route.name === 'tools') app.innerHTML = library(false, route.params);
    else if (route.name === 'saved') app.innerHTML = library(true, route.params);
    else if (route.name === 'playbooks') app.innerHTML = playbooks(route.params);
    else if (route.name === 'job') app.innerHTML = jobDetail(route.id, route.quickId);
    else if (route.name === 'tool') app.innerHTML = toolDetail(route.id);
    else if (route.name === 'resources') app.innerHTML = resources();
    else if (route.name === 'knowledge') app.innerHTML = knowledgePage();
    else app.innerHTML = notFound('page');
    const title = route.name === 'job' ? jobMap.get(route.id)?.title : route.name === 'tool' ? toolMap.get(route.id)?.name : names[route.name];
    document.title = `${title || 'Technician workspace'} | INTA Computer Services`;
    filterTools(false); filterJobs(false); filterSources(false); filterOnline(false); filterCommandCenter(); bindNews(); linkifyProse(app); updateCounts();
    closeNavigation(); closeSearch();
    if (route.anchor && document.getElementById(route.anchor)) document.getElementById(route.anchor).scrollIntoView({ block: 'start', behavior: 'instant' });
    else if (lastRoute !== route.raw) window.scrollTo({ top: 0, behavior: 'instant' });
    const serviceTarget = route.name === 'services' && document.getElementById(`service-${route.params.get('area')}`);
    const section = route.name === 'job' && route.params.get('section');
    const sectionTarget = section && document.getElementById(`job-${section}`);
    const selectedAnswer = route.quickId && document.getElementById(`quick-${route.quickId}`);
    const onlineTarget = route.name === 'online' && onlineByTool.has(route.params.get('tool'))
      && document.getElementById(`online-service-${route.params.get('tool')}`);
    if (onlineTarget) {
      onlineTarget.scrollIntoView({ block: 'start', behavior: 'instant' });
      onlineTarget.querySelector('h3[tabindex]')?.focus({ preventScroll: true });
    } else if (serviceTarget) {
      serviceTarget.scrollIntoView({ block: 'start', behavior: 'instant' });
      serviceTarget.querySelector('h2[tabindex]')?.focus({ preventScroll: true });
    } else if (sectionTarget) {
      sectionTarget.scrollIntoView({ block: 'start', behavior: 'instant' });
      sectionTarget.querySelector('h2[tabindex],h3[tabindex]')?.focus({ preventScroll: true });
    } else if (selectedAnswer) {
      selectedAnswer.scrollIntoView({ block: 'start', behavior: 'instant' });
      selectedAnswer.querySelector('h3')?.focus({ preventScroll: true });
    } else if (lastRoute) $('#main').focus({ preventScroll: true });
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
    linkifyProse(toast);
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
    if (parseRoute().name === 'online' && $('#online-saved-only')?.checked) {
      filterOnline();
      if (wasSaved) ($('#online-results [data-save]') || $('#online-saved-only')).focus({ preventScroll: true });
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
  function markdownBrandIcon(brand) {
    const url = brand && safeUrl(brandData.assetUrls?.[brand.assetId]);
    return url ? `<img src="${esc(url)}" width="18" height="18" alt="">` : '';
  }
  function markdownBrandName(brand, label) {
    const mark = markdownBrandIcon(brand);
    return brand ? `[${mark ? mark + ' ' : ''}${label}](${safeUrl(brand.downloadPage)})` : label;
  }
  function markdownBrandAside(brand) {
    const mark = markdownBrandIcon(brand);
    return mark ? `[${mark}](${safeUrl(brand.downloadPage)}) ` : '';
  }
  function decorateChecklistMarkdown(source) {
    // The checklist stays ordinary Markdown. Commands, code, URLs used as link
    // targets, reference definitions and existing image markup are not rewritten.
    function balanced(value, start, open, close) {
      let depth = 0;
      for (let i = start; i < value.length; i++) {
        if (value[i] === '\\') { i++; continue; }
        if (value[i] === open) depth++;
        else if (value[i] === close && --depth === 0) return i + 1;
      }
      return -1;
    }
    function prose(value) {
      if (!brandMatcher) return value;
      brandMatcher.lastIndex = 0;
      return value.replace(brandMatcher, match => {
        const key = match.toLocaleLowerCase('en-US'), brand = brandsByName.get(key);
        if (!brand || (sensitiveBrandNames.has(key) && !array(brand.names).includes(match))) return match;
        return markdownBrandName(brand, match);
      });
    }
    function inline(value) {
      let output = '', start = 0, i = 0;
      while (i < value.length) {
        let end = -1, replacement = null;
        if (value[i] === '`') {
          const run = /^`+/.exec(value.slice(i))[0], close = value.indexOf(run, i + run.length);
          if (close >= 0) end = close + run.length;
        } else if (value[i] === '<') {
          const token = /^(?:<a\b[^>]*>[\s\S]*?<\/a\s*>|<!--[^]*?-->|<[^>]*>)/i.exec(value.slice(i));
          if (token) end = i + token[0].length;
        } else if (value[i] === '[' || value.startsWith('![', i)) {
          const image = value[i] === '!', bracket = image ? i + 1 : i;
          const close = balanced(value, bracket, '[', ']');
          if (close >= 0 && value[close] === '(') {
            end = balanced(value, close, '(', ')');
            if (end >= 0 && !image) {
              const label = value.slice(bracket + 1, close - 1), href = value.slice(close + 1, end - 1).trim();
              if (!/<img\b/i.test(label)) {
                const brand = brandsByName.get(label.toLocaleLowerCase('en-US'));
                const mentions = mentionedBrands(label);
                if (brand) replacement = brandUrlKey(href) === brandUrlKey(brand.downloadPage)
                  ? `[${markdownBrandIcon(brand) ? markdownBrandIcon(brand) + ' ' : ''}${label}](${href})`
                  : markdownBrandAside(brand) + value.slice(i, end);
                else if (mentions.length) replacement = mentions.map(markdownBrandAside).join('') + value.slice(i, end);
                else if (/source|reference|https?:|\.[a-z]{2,}/i.test(label)) replacement = markdownBrandAside(brandFor({ url: href })) + value.slice(i, end);
              }
            }
          } else if (close >= 0 && value[close] === '[') end = balanced(value, close, '[', ']');
        } else if (value.startsWith('https://', i) || value.startsWith('http://', i)) {
          const raw = /^https?:\/\/[^\s<>]+/.exec(value.slice(i));
          if (raw) {
            end = i + raw[0].length;
            const brand = brandFor({ url: raw[0] });
            if (brand) replacement = markdownBrandAside(brand) + `[${raw[0]}](${raw[0]})`;
          }
        }
        if (end > i) {
          output += prose(value.slice(start, i)) + (replacement === null ? value.slice(i, end) : replacement);
          i = start = end;
        } else i++;
      }
      return output + prose(value.slice(start));
    }
    let fence = null;
    return source.split(/(?<=\n)/).map(line => {
      const marker = /^[ >\t]*(`{3,}|~{3,})(.*?)(?:\r?\n)?$/.exec(line);
      if (fence) {
        if (marker && marker[1][0] === fence[0] && marker[1].length >= fence.length && !marker[2].trim()) fence = null;
        return line;
      }
      if (marker) { fence = marker[1]; return line; }
      if (/^(?: {4}|\t)|^ {0,3}\[[^\]]+\]:/.test(line)) return line;
      return inline(line);
    }).join('');
  }
  function appendChecklistStep(lines, job, step, index, number = index + 1) {
    lines.push(`- [${checked(job, 'step', index, step.title) ? 'x' : ' '}] ${number}. ${step.title}`, '', text(step.detail), '');
    array(step.shortcuts).forEach(item=>{
      const tool = toolMap.get(item.tool);
      if (tool) {
        const access = accessInfo(tool);
        lines.push(`- [${item.label}](${safeUrl(access.officialUrl)})`);
        access.options.forEach(option=>lines.push(`  - [${accessLabel(option)}](${safeUrl(option.url)})`));
      } else {
        const target = item.job ? `${repo}/wiki/Job-${item.job}` : safeUrl(item.url);
        lines.push(`- [${item.label}](${target})`);
      }
    });
    if (array(step.shortcuts).length) lines.push('');

    if (step.command) {
      lines.push('```text', text(step.command), '```', '');
      const references = commandReferences(step.command);
      if (references.length) lines.push('Command references: ' + references.map(entry => `[${entry.name}](${entry.href})`).join(' · '), '');
    }
  }
  function exportTechnicianChecklist(job) {
    const state = progress(job);
    const lines = ['# INTA · Technician checklist', '', 'One pass from protecting the data to handing the computer back.', '', `Source review: ${job.reviewedAt || reviewDate}`, `Exported: ${new Date().toISOString()}`, `Checklist: ${state.done}/${state.total} checks`, '', '## Protect data before routine maintenance', '', 'If a drive may be failing, recover first. Isolate a suspected active compromise before routine patching or optimization.', '', '## Work through the checklist', ''];
    closeoutRows(job).forEach(({ step, index }, number) => appendChecklistStep(lines, job, step, index, number + 1));
    lines.push('## Before closing', '', 'Mark steps after you verify them. Record deferred work in the private ticket. Never mark an unresolved critical issue as passed.', '', `[Full handoff guide](${repo}/wiki/Job-${job.id})`, '', 'Record customer-specific details in your private service ticket.', '');
    download('INTA-technician-checklist.md', decorateChecklistMarkdown(lines.join('\n')));
    notify('Technician checklist exported with your current progress.');
  }
  function exportJob(id) {
    const job = jobMap.get(id);
    if (!job) return;
    if (parseRoute().name === 'checklist' && job.id === 'client-handoff') return exportTechnicianChecklist(job);
    const state = progress(job);
    const lines = [`# INTA · ${job.title}`, '', job.summary, '', `Source review: ${job.reviewedAt || reviewDate}`, `Exported: ${new Date().toISOString()}`, `Checklist: ${state.done}/${state.total} checks`, ''];
    const workspace = workspaceForJob(job);
    if (workspace) {
      lines.push('## Working plan', '', text(workspace.goal), '', text(workspace.startHere), '', '## Tools and resources for this job', '');
      workspaceTools(job, workspace, parseRoute().quickId).forEach(entry => {
        const access = accessInfo(entry.tool);
        const selectedOptions = entry.actionId ? access.options.filter(option => option.id === entry.actionId) : [];
        const options = entry.actionId ? [...selectedOptions, ...access.options.filter(option => option.id !== entry.actionId)] : access.options;
        lines.push(`### ${entry.tool.name}`, '', text(entry.why), '', `[Official page](${safeUrl(access.officialUrl)})`, ...options.map(option => `- [${accessLabel(option)}](${safeUrl(option.url)})`), '');
      });
      array(workspace.resources).forEach(resource => {
        lines.push(`- [${resource.label}](${safeUrl(resource.url)}) — ${text(resource.description)}`, `  - ${resourceGroupLabel(resource.group)} · ${resourceGroupLabel(resource.kind)} · Checked ${resource.checkedAt || data.jobWorkspaces.reviewedAt || reviewDate}`);
        if (resource.sourceUrl) lines.push(`  - [Source](${safeUrl(resource.sourceUrl)})`);
      });
      if (array(workspace.decisions).length) {
        lines.push('', '## Choose the next action', '');
        array(workspace.decisions).forEach(decision => lines.push(`### ${text(decision.symptom)}`, '', `Action: ${text(decision.action)}`, '', `Expected: ${text(decision.expected)}`, ''));
      }
    }
    const picks = quickByJob.get(job.id) || [];
    const selected = picks.find(item => item.id === parseRoute().quickId);
    const ordered = selected ? [selected, ...picks.filter(item => item !== selected)] : picks;
    if (ordered.length) {
      lines.push('## First actions', '');
      ordered.forEach(item => {
        lines.push(`### ${item.label}`, '', text(item.firstStep), '', ...array(item.quickSteps).map((step, i) => `${i + 1}. ${text(step)}`), '');
        if (item.guideOnly) lines.push('No extra download needed. Any tools below are optional aids.', '');
        quickChoices(item).forEach(choice => {
          const access = accessInfo(choice.tool);
          const options = choice.actionId ? access.options.filter(option => option.id === choice.actionId) : access.options.slice(0, 1);
          lines.push(`#### ${choice.tool.name}`, '', `Role: ${text(choice.role)}`, '', `Best fit: ${text(choice.why)}`, '', `Use and cost: ${text(choice.freeUse)}`, '', `[Official page](${safeUrl(access.officialUrl)})`, ...options.map(option => `- [${accessLabel(option)}](${safeUrl(option.url)})`), '');
          array(choice.evidenceUrls).forEach((url, index) => lines.push(`- [Selection source ${index + 1}](${safeUrl(url)})`));
          if (choice.reviewedAt) lines.push(`Reviewed: ${choice.reviewedAt}`);
          lines.push('');
        });
      });
    }
    lines.push('## Start here', '', text(job.startHere), '', '## Work through the job', '');
    array(job.steps).forEach((step, index) => appendChecklistStep(lines, job, step, index));
    lines.push('## Verify the result', '');
    array(job.verify).forEach((item, index) => lines.push(`- [${checked(job, 'verify', index, text(item)) ? 'x' : ' '}] ${text(item)}`));
    if (array(workspace?.doneWhen).length) lines.push('', '## Ready to close when', '', ...array(workspace.doneWhen).map(item => `- ${text(item)}`));
    lines.push('', '## Pause and escalate when', '', ...[...new Set([...array(job.stopIf), ...array(workspace?.escalateWhen)].map(text))].map(item => `- ${item}`), '', '## Tools', '');
    relatedTools(job.tools).forEach(tool => {
      const access = accessInfo(tool);
      lines.push(`- [${tool.name}](${safeUrl(access.officialUrl)}) — ${tool.summary}`);
      access.options.forEach(option => lines.push(`  - [${accessLabel(option)}](${safeUrl(option.url)})`));
    });
    const notes = fieldNotesByJob.get(job.id) || [];
    if (notes.length) {
      lines.push('', '## Specific fixes', '');
      notes.forEach(note => {
        lines.push(`### ${note.title}`, '', `Symptom: ${text(note.symptom)}`, '', `First action: ${text(note.firstAction)}`, '', ...array(note.steps).map((step, index) => `${index + 1}. ${text(step)}`), '', `Expected result: ${text(note.expected)}`, '', `If unresolved: ${text(note.ifUnresolved)}`, '');
        fieldNoteTools(note, job).forEach(entry => {
          const access = accessInfo(entry.tool);
          const chosen = entry.actionId ? access.options.filter(option => option.id === entry.actionId) : [];
          const options = entry.actionId ? [...chosen, ...access.options.filter(option => option.id !== entry.actionId)] : access.options;
          lines.push(`[${entry.tool.name} · official page](${safeUrl(access.officialUrl)})`, ...options.map(option => `- [${accessLabel(option)}](${safeUrl(option.url)})`), '');
        });
        lines.push('Official sources:', ...array(note.sourceUrls).map(url => `- ${safeUrl(url)}`), '', `Reviewed: ${note.reviewedAt || data.fieldNotes.reviewedAt || reviewDate}`, '');
      });
    }
    if (picks.length) {
      lines.push('', '## Selection notes', '');
      picks.forEach(item => lines.push(`### ${item.label}`, '', `Why: ${item.why}`, '', `Use and cost: ${item.freeUse}`, '', `Before you act: ${item.boundary}`, '', `Selection basis: ${item.qualityBasis}`, '', ...array(item.evidenceUrls).map(url => `- ${url}`), ''));
    }
    lines.push('', '## Official references', '', ...array(job.sources).map(source => `- [${source.title || source.url}](${safeUrl(source.url)})`), '', 'Record customer-specific details in your private service ticket.', '');
    download(`INTA-${job.id}-checklist.md`, decorateChecklistMarkdown(lines.join('\n')));
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
    const foundScenarios = quickItems.filter(item => matches(quickSearchText.get(item.id), query));
    const exactScenarioJobs = new Set(foundScenarios.map(item => item.jobId));
    jobs.forEach(job => {
      const firstAnswer = quickByJob.get(job.id)?.[0];
      if (firstAnswer && !exactScenarioJobs.has(job.id) && matches(workspaceSearchText.get(job.id), query)) foundScenarios.push(firstAnswer);
    });
    const foundNotes = fieldNotes.filter(note => matches(fieldNoteSearchText.get(note.id), query));
    const foundJobs = jobs.filter(job => matches(jobSearchText.get(job.id), query));
    const foundTools = tools.filter(tool => matches(toolSearchText.get(tool.id), query));
    const foundCommands = array(commandCenter.commands).filter(item => matches(commandCenterSearch.get(item.id) || '', query));
    let html = `<div class="search-summary">${foundScenarios.length + foundNotes.length + foundJobs.length + foundTools.length + foundCommands.length} results for “${esc(query)}”</div>`;
    if (foundScenarios.length) html += `<div class="search-label">First actions · ${foundScenarios.length}</div>${foundScenarios.map(item => `<a class="search-result search-scenario" data-search-scenario="${esc(item.id)}" href="${quickHref(item)}"><span class="result-icon" aria-hidden="true">${esc(item.icon || scenarioEmoji[item.jobId] || '🧰')}</span><span><strong>${esc(item.label)}</strong><small>${esc(item.firstStep)}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (foundNotes.length) html += `<div class="search-label">Specific fixes · ${foundNotes.length}</div>${foundNotes.map(note => `<a class="search-result" data-search-fix="${esc(note.id)}" href="${fieldNoteHref(note)}"><span class="result-icon">${icon('wrench')}</span><span><strong>${esc(note.title)}</strong><small>${esc(note.symptom)}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (foundJobs.length) html += `<div class="search-label">Job playbooks · ${foundJobs.length}</div>${foundJobs.map(job => `<a class="search-result" href="#job/${encodeURIComponent(job.id)}"><span class="result-icon">${icon(jobIcon(job))}</span><span><strong>${esc(job.title)}</strong><small>${esc(job.category)}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (foundTools.length) html += `<div class="search-label">Tools · ${foundTools.length}</div>${foundTools.map(tool => `<div class="search-tool-result" data-search-tool="${esc(tool.id)}"><div class="search-tool-main">${toolName(tool, 'search-tool-name')}<small>${esc(categoryLabel(tool.category))} · ${esc(array(tool.platforms).join(', '))}</small></div><a class="search-tool-details" href="#tool/${encodeURIComponent(tool.id)}" aria-label="Details and alternate downloads for ${esc(tool.name)}">Details ${icon('arrow')}</a></div>`).join('')}`;
    if (foundCommands.length) html += `<div class="search-label">Commands · ${foundCommands.length}</div>${foundCommands.map(item => `<a class="search-result" href="#command-center?category=${encodeURIComponent(item.category)}&q=${encodeURIComponent(item.title)}"><span class="result-icon">${icon('terminal')}</span><span><strong>${esc(item.title)}</strong><small>${esc(item.shell)} · ${esc(item.summary)}</small></span>${icon('arrow')}</a>`).join('')}`;
    if (!foundScenarios.length && !foundNotes.length && !foundJobs.length && !foundTools.length && !foundCommands.length) html += '<div class="search-empty">No results yet. Try a tool name or a symptom such as “network” or “malware”.</div>';
    searchResults.innerHTML = html;
    decorateBrandLinks(searchResults);
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
    if ($('#tool-sort')) $('#tool-sort').value = 'catalog';
    filterTools();
  }
  function resetJobFilters() { $('#jobs-search').value = ''; $('#jobs-category').value = ''; filterJobs(); }

  document.addEventListener('click', event => {
    const target = event.target instanceof Element ? event.target : null;
    if (!target) return;
    if (target.closest('.skip-link')) { event.preventDefault(); $('#main').focus(); return; }
    const jobCardTarget = target.closest('[data-job-route]');
    if (jobCardTarget && !target.closest('a,button,input,select,textarea') && !window.getSelection()?.toString()) { location.hash = jobCardTarget.dataset.jobRoute; return; }
    const onlineGroup = target.closest('[data-online-group]');
    if (onlineGroup) { $('#online-group').value = onlineGroup.dataset.onlineGroup; filterOnline(); return; }
    if (target.closest('[data-reset-online]')) {
      $('#online-search').value = ''; $('#online-group').value = '';
      $('#online-task-only').checked = false; $('#online-saved-only').checked = false;
      filterOnline(); $('#online-search').focus(); return;
    }
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
    const fieldCommand = target.closest('[data-copy-field-command]');
    if (fieldCommand) {
      const note = fieldNotes.find(item => item.id === fieldCommand.dataset.copyFieldCommand);
      const commandItem = note && array(note.commands)[Number(fieldCommand.dataset.commandIndex)];
      if (commandItem) copy(text(commandItem.code), 'Command copied. Review its safety note before running it.');
      return;
    }
    const centerCommand = target.closest('[data-copy-center-command]');
    if (centerCommand) {
      const item = commandCenterCommands.get(centerCommand.dataset.copyCenterCommand);
      if (item) copy(item.code, 'Command copied. Review its shell and safety notes before running.');
      return;
    }
    const centerWorkflow = target.closest('[data-copy-center-workflow]');
    if (centerWorkflow) {
      const workflow = array(commandCenter.workflows).find(item => item.id === centerWorkflow.dataset.copyCenterWorkflow);
      if (workflow) copy(workflow.commandIds.map(id => { const item=commandCenterCommands.get(id); return item ? `# ${item.title} [${item.shell}]
${item.code}
# Safety: ${item.safety}` : ''; }).filter(Boolean).join('\n\n'), 'Workflow commands copied. Review each step before running.');
      return;
    }
    const categoryTab = target.closest('[data-command-category]');
    if (categoryTab) {
      $$('.command-category-tabs [data-command-category]').forEach(button => { const active=button===categoryTab; button.classList.toggle('active',active); button.setAttribute('aria-pressed',String(active)); });
      filterCommandCenter();
      return;
    }
    const clearFilter = target.closest('[data-clear-tool-filter]');
    if (clearFilter) { const field = document.getElementById(clearFilter.dataset.clearToolFilter); if (field) { field.value = ''; filterTools(); field.focus(); } return; }
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
    if (hashLink && hashLink.getAttribute('href') === location.hash) {
      closeSearch(); closeNavigation();
      const route = parseRoute();
      const targetId = route.name === 'services' && route.params.get('area') ? `service-${route.params.get('area')}` : route.name === 'job' && route.params.get('section') ? `job-${route.params.get('section')}` : route.name === 'online' && onlineByTool.has(route.params.get('tool')) ? `online-service-${route.params.get('tool')}` : route.anchor;
      const destination = targetId && document.getElementById(targetId);
      if (destination) {
        event.preventDefault();
        destination.scrollIntoView({ block: 'start', behavior: 'instant' });
        destination.querySelector('h2[tabindex],h3[tabindex]')?.focus({ preventScroll: true });
      }
    }
  });
  document.addEventListener('input', event => {
    if (event.target.id === 'command-center-search') filterCommandCenter();
    if (event.target.id === 'online-search') filterOnline();
    else if (event.target.id === 'library-search') filterTools();
    else if (event.target.id === 'jobs-search') filterJobs();
    else if (event.target.id === 'source-search') filterSources();
  });
  document.addEventListener('change', event => {
    if (['command-center-shell', 'command-center-risk'].includes(event.target.id)) filterCommandCenter();
    else if (['online-group', 'online-task-only', 'online-saved-only'].includes(event.target.id)) filterOnline();
    else if (['category-filter', 'platform-filter', 'tier-filter', 'tool-sort'].includes(event.target.id)) filterTools();
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
    if (event.key === savedKey) { const next = load(savedKey, []); favorites = new Set(Array.isArray(next) ? next.filter(id => toolMap.has(id)) : []); updateCounts(); if (parseRoute().name === 'saved') render(); else if (parseRoute().name === 'online') filterOnline(); }
    if (event.key === checksKey) { const next = load(checksKey, {}); if (next && typeof next === 'object' && !Array.isArray(next)) checklists = next; migrateChecklistState(); if (['job','checklist'].includes(parseRoute().name)) render(); }
  });
  $$('[data-icon]').forEach(element => { element.innerHTML = icon(element.dataset.icon); });
  migrateChecklistState(); setTheme(); render();
  decorateBrandLinks(document.querySelector('.secondary-nav'));
  decorateBrandLinks(document.querySelector('.topbar'));
  decorateBrandLinks(document.querySelector('.main-footer'));
})();
