#!/usr/bin/env python3
"""Refresh authentic, locally cached website icons. On demand, never on page load.

Publisher-declared icon -> publisher favicon -> recorded favicon cache. Bytes are
validated and normalized to small PNGs. Failed refreshes preserve prior assets.
"""
from __future__ import annotations
import argparse, concurrent.futures, hashlib, io, json, re, sys, threading, subprocess, tempfile
from pathlib import Path
from urllib.parse import urljoin, urlsplit, urlunsplit, quote
from urllib.request import Request, urlopen
from html.parser import HTMLParser
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'assets/brand-icons'
DATE = '2026-11-26'
HEADERS = {'User-Agent': 'Mozilla/5.0 (compatible; INTA-icon-cache/1.0; +https://github.com/Herbertofury/INTA)'}
LOCK=threading.Lock()
CACHE={}
FUTURES={}
FAILURES=[]

class IconLinks(HTMLParser):
    def __init__(self): super().__init__(); self.icons=[]
    def handle_starttag(self, tag, attrs):
        a=dict(attrs)
        if tag=='link' and 'icon' in a.get('rel','').lower() and a.get('href'):
            sizes=a.get('sizes','')
            score=3 if sizes in ['64x64','48x48','32x32'] else 2 if 'apple-touch' in a.get('rel','') else 1
            self.icons.append((score,a['href']))

def fetch(url, limit=3_000_000):
    if not url.startswith('https://'): raise ValueError('HTTPS required')
    # curl enforces a wall-clock deadline, including slow response bodies.
    command=['curl','--location','--fail','--silent','--show-error','--connect-timeout','4','--max-time','9','--max-filesize',str(limit),'--user-agent',HEADERS['User-Agent'],'--write-out','\n%{url_effective}\n%{content_type}',url]
    result=subprocess.run(command,capture_output=True,timeout=12)
    if result.returncode:raise ValueError(result.stderr.decode('utf-8',errors='replace')[:180])
    raw,final,ctype=result.stdout.rsplit(b'\n',2)
    if len(raw)>limit:raise ValueError('Oversized icon/page')
    return raw,final.decode(),ctype.decode()

def normalize(raw, ctype):
    if b'<svg' in raw[:1000].lower():
        if re.search(rb'<(?:script|foreignObject)\b|\bon\w+\s*=|(?:href|src)\s*=\s*["\']\s*(?:https?:|//|data:)', raw,re.I):raise ValueError('SVG has active/external content')
        with tempfile.TemporaryDirectory() as temp:
            src=Path(temp)/'icon.svg';dest=Path(temp)/'icon.png';src.write_bytes(raw)
            process=subprocess.run(['inkscape',str(src),'--export-type=png','--export-filename='+str(dest),'--export-width=64','--export-height=64'],capture_output=True,timeout=15)
            if process.returncode:raise ValueError('SVG rasterization failed')
            raw=dest.read_bytes()
    im=Image.open(io.BytesIO(raw))
    if im.format=='ICO':
        sizes=im.ico.sizes(); desired=min(sizes,key=lambda x:abs(x[0]-64));im=im.ico.getimage(desired)
    im.seek(0);im.load();im=im.convert('RGBA')
    if im.width<8 or im.height<8:raise ValueError('Tracking pixel, not icon')
    if im.width>64 or im.height>64:im.thumbnail((64,64),Image.Resampling.LANCZOS)
    if im.getbbox() is None:raise ValueError('Empty image')
    out=io.BytesIO();im.save(out,format='PNG',optimize=True)
    return out.getvalue(),im.width,im.height

# These URLs are publisher-owned site identity assets; sources are preserved.
OVERRIDES={
 'github.com':'https://github.com/favicon.ico',
 'www.chiark.greenend.org.uk':'https://www.chiark.greenend.org.uk/~sgtatham/putty/putty.ico',
 'github.github.com':'https://github.com/favicon.ico',
 'docs.github.com':'https://github.com/favicon.ico',
 'api.github.com':'https://github.com/favicon.ico',
 'herbertofury.github.io':'https://github.com/favicon.ico',
 'microsoft.com':'https://www.microsoft.com/favicon.ico',
 'google.com':'https://www.google.com/favicon.ico',
 'docs.python.org':'https://www.python.org/static/favicon.ico',
 'python.org':'https://www.python.org/static/favicon.ico',
 'claude.com':'https://claude.ai/favicon.ico',
 'claude.ai':'https://claude.ai/favicon.ico',
 'chatgpt.com':'https://chatgpt.com/favicon.ico',
 'openai.com':'https://openai.com/favicon.ico',
 'www.nist.gov':'https://www.nist.gov/themes/custom/nist_www/favicon.ico',
 'id-ransomware.malwarehunterteam.com':'https://id-ransomware.malwarehunterteam.com/assets/images/favicons/favicon-32x32.png',
 'dnssec-debugger.verisignlabs.com':'https://www.verisign.com/favicon.ico',
 'openphish.com':'https://openphish.com/static/openphish_logo2.png',
 'www.webpagetest.org':'https://raw.githubusercontent.com/catchpoint/WebPageTest/master/www/favicon.ico',
 # Twilio's main page declares this icon; the diagnostics subdomain has none.
 'networktest.twilio.com':'https://www.twilio.com/content/dam/twilio-com/core-assets/social/favicon-32x32.png',
}
# Personal/project directories can declare an icon that the shared host root
# does not have. These exact publisher routes were observed on their pages.
PAGE_ICONS={
 'https://www.chiark.greenend.org.uk/~sgtatham/putty/':'https://www.chiark.greenend.org.uk/~sgtatham/putty/putty.ico',
 'https://the.earth.li/~sgtatham/putty/':'https://www.chiark.greenend.org.uk/~sgtatham/putty/putty.ico',
}
# Consolidate known publisher subdomains; never blindly map shared hosting.
PARENTS=['microsoft.com','google.com','malwarebytes.com','sophos.com','bitdefender.com','eset.com','emsisoft.com','rclone.org','tailscale.com','teamviewer.com','anydesk.com','wireshark.org','wazuh.com','veeam.com','debian.org','ubuntu.com','samba.org','cpuid.com','sysinternals.com','gnu.org','syncthing.net','clamav.net','ollama.com','naps2.com','winmerge.org','osquery.io','restic.net','volatilityfoundation.org']
MAP={'github.github.com':'github.com','nvlpubs.nist.gov':'www.nist.gov','onyxweb.mykonicaminolta.com':'kmbs.konicaminolta.us','aka.ms':'microsoft.com','go.microsoft.com':'microsoft.com','download.sysinternals.com':'microsoft.com','live-team.pages.debian.net':'www.debian.org','restic.readthedocs.io':'restic.net','osquery.readthedocs.io':'osquery.io','volatility3.readthedocs.io':'volatilityfoundation.org','virustotal.github.io':'www.virustotal.com','the.earth.li':'www.chiark.greenend.org.uk','manual.winmerge.org':'winmerge.org','npp-user-manual.org':'notepad-plus-plus.org','downloads.es.net':'software.es.net','registry.npmjs.org':'www.npmjs.com','helpcenter.veeam.com':'www.veeam.com','code.claude.com':'claude.ai','knowledge.workspace.google.com':'google.com','dl.dell.com':'www.dell.com','docs.abuseipdb.com':'www.abuseipdb.com','downloads.mangodisk.app':'mangodisk.app','download.documentfoundation.org':'www.libreoffice.org','dl.emsisoft.com':'www.emsisoft.com','fastly-cdn.system-rescue.org':'www.system-rescue.org'}

def owner(host):
    h=host.lower().removeprefix('www.')
    if host in MAP:return MAP[host]
    for p in PARENTS:
        if h==p or h.endswith('.'+p):return p
    return host

def acquire(host,page):
    h=owner(host)
    # Single-flight every canonical host avoids repeated slow lookups.
    with LOCK:
        if h in CACHE:return CACHE[h]
        if h in FUTURES:ev=FUTURES[h];leader=False
        else:ev=FUTURES[h]=threading.Event();leader=True
    if not leader:
        ev.wait(100)
        return CACHE.get(h)
    attempts=[]; result=None
    try:
        root='https://'+h+'/'
        candidates=[]
        candidates.extend((icon,'publisher-declared project icon') for prefix,icon in PAGE_ICONS.items() if page.startswith(prefix))
        if h in OVERRIDES:candidates.append((OVERRIDES[h],'publisher website icon'))
        # Inspect the actual product page before the host root. A documentation
        # or personal-project subdirectory may own its own declared icon.
        if not candidates:
            pages=[root]
            if owner(host)==host and not re.search(r'\.(?:pdf|exe|msi|zip|gz|xz|7z|iso|dmg|deb|rpm)$',urlsplit(page).path,re.I):
                pages=list(dict.fromkeys([page,root]))
            for source_page in pages:
                try:
                    html,final,ctype=fetch(source_page,2_000_000)
                    if 'html' in ctype or html.lstrip().startswith(b'<'):
                        p=IconLinks();p.feed(html.decode('utf-8',errors='replace'))
                        candidates.extend((urljoin(final,url),'publisher-declared website icon') for _,url in sorted(p.icons,reverse=True)[:5])
                    if candidates:break
                except Exception as e:attempts.append({'url':source_page,'error':str(e)[:160]})
        candidates += [(urljoin(root,'favicon.ico'),'publisher website icon'),(urljoin(root,'favicon.png'),'publisher website icon')]
        candidates.append(('https://www.google.com/s2/favicons?domain_url='+quote(root,safe='')+'&sz=64','Google favicon cache of publisher website'))
        seen=set()
        for u,kind in candidates:
            if u in seen or not u.startswith('https://'):continue
            seen.add(u)
            try:
                raw,final,ctype=fetch(u);png,w,hgt=normalize(raw,ctype)
                digest=hashlib.sha256(png).hexdigest()
                if digest in GENERIC:raise ValueError('Generic favicon placeholder rejected')
                asset='i'+digest[:20];path=OUT/(digest[:20]+'.png');path.write_bytes(png)
                result={'assetId':asset,'asset':{'path':str(path.relative_to(ROOT)),'sourceUrl':u,'resolvedSourceUrl':final,'sourceKind':kind,'publisherHost':host,'sha256':digest,'mimeType':'image/png','width':w,'height':hgt,'checkedAt':DATE,'sourceSha256':hashlib.sha256(raw).hexdigest()}}
                break
            except Exception as e:attempts.append({'url':u,'error':str(e)[:160]})
        if not result:FAILURES.append({'host':host,'page':page,'attempts':attempts})
    finally:
        with LOCK:CACHE[owner(host)]=result;ev.set()
    return result

def urls_in(value):
    if isinstance(value,dict):
        for x in value.values():yield from urls_in(x)
    elif isinstance(value,list):
        for x in value:yield from urls_in(x)
    elif isinstance(value,str):
        yield from re.findall(r'https://[^\s"<>\]\)\}\,]+',value)

def norm_url(u):
    p=urlsplit(u);return urlunsplit((p.scheme,p.netloc,p.path.rstrip('/'),p.query,''))

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--only',nargs='*');ap.add_argument('--force',action='store_true',help='Reacquire selected publisher icons instead of reusing verified cached assets');args=ap.parse_args()
    OUT.mkdir(parents=True,exist_ok=True)
    tools=json.loads((ROOT/'data/tools.json').read_text());access=json.loads((ROOT/'data/tool-access.json').read_text())['tools'];aliases=json.loads((ROOT/'data/link-aliases.json').read_text())['links']
    previous=json.loads((ROOT/'data/brand-icons.json').read_text()) if (ROOT/'data/brand-icons.json').exists() else {'assets':{},'brands':[]}
    failure_history=list(previous.get('failureHistory',[]))
    for failure in previous.get('failures',[]):
        if failure not in failure_history:failure_history.append(failure)
    for aid,a in previous['assets'].items():
        # Cache by the image's publisher identity only. A tool's release/docs
        # hosts may be shared providers and must never acquire that tool's logo.
        if not args.force:CACHE.setdefault(owner(a['publisherHost']),{'assetId':aid,'asset':a})
    # A failed atomic refresh still contains useful, verified acquisitions.
    # Resume only intact local assets; never publish its incomplete registry.
    report_path=ROOT/'reports/brand-icon-refresh.json'
    if not args.force and report_path.exists():
        try:
            report=json.loads(report_path.read_text())
            for aid,a in report.get('assets',{}).items():
                path=(ROOT/a['path']).resolve()
                if (path.is_relative_to(OUT.resolve()) and path.is_file()
                        and hashlib.sha256(path.read_bytes()).hexdigest()==a.get('sha256')):
                    CACHE.setdefault(owner(a['publisherHost']),{'assetId':aid,'asset':a})
            for failure in report.get('failures',[]):
                if failure not in failure_history:failure_history.append(failure)
        except (OSError,ValueError,KeyError):
            pass
    prevby={b['id']:b for b in previous['brands']}
    global GENERIC
    GENERIC=set()
    # Detect the cache provider's generic globe once; never accept it as a brand.
    try:
        b,_,c=fetch('https://www.google.com/s2/favicons?domain_url=https%3A%2F%2Fno-icon.invalid%2F&sz=64');GENERIC.add(hashlib.sha256(normalize(b,c)[0]).hexdigest())
    except Exception:pass
    brands=[]
    for t in tools:
        a=access[t['id']];route=a['officialUrl'];p=a.get('primary',{})
        hosted=t.get('downloadUrl','')
        if hosted.startswith('https://') and not re.search(r'\.(?:exe|msi|zip|dmg|pkg|deb|rpm|iso|tgz|gz|xz|lz)(?:[?#]|$)',hosted):route=hosted
        if p.get('kind') in ('web','store') and p.get('url','').startswith('https://'):route=p['url']
        names=[t['name'],*t.get('aliases',[])]
        for x in aliases:
            if x.get('toolId')==t['id']:names.extend([x['name'],*x.get('aliases',[])])
        urls=list(dict.fromkeys([t['homepage'],t.get('downloadUrl',''),t.get('docsUrl',''),a['officialUrl'],*list(urls_in(a))]));urls=[u for u in urls if u.startswith('https://')]
        brands.append({'id':t['id'],'toolId':t['id'],'name':t['name'],'names':list(dict.fromkeys(names)),'urls':urls,'hosts':sorted(set(urlsplit(u).hostname for u in urls)),'downloadPage':route,'kind':'tool'})
    for a in aliases:
        if 'toolId' in a or a.get('kind') in ('method','reference'):continue
        brand = {'id':'app-'+a['id'],'name':a['name'],'names':[a['name'],*a.get('aliases',[])],'urls':[a['url']],'hosts':[urlsplit(a['url']).hostname],'downloadPage':a['url'],'kind':'application'}
        if 'inlineIcon' in a: brand['inlineIcon'] = a['inlineIcon']
        brands.append(brand)
    brands.append({'id':'app-inta','name':'INTA','names':['INTA','INTA Computer Services'],'urls':['https://herbertofury.github.io/INTA/','https://github.com/Herbertofury/INTA'],'hosts':['herbertofury.github.io','github.com'],'downloadPage':'https://herbertofury.github.io/INTA/','kind':'application'})
    for brand in brands:
        if brand['id']=='app-inta-rescue':brand['downloadPage']='https://github.com/Herbertofury/INTA/releases'
        if brand['id']=='app-media-creator':brand['names'].append('INTA Media Creator')
    brands.extend([
        {'id':'app-yara','name':'YARA','names':['YARA'],'urls':['https://virustotal.github.io/yara/','https://github.com/VirusTotal/yara','https://github.com/VirusTotal/yara/releases/latest'],'hosts':['virustotal.github.io','github.com'],'downloadPage':'https://github.com/VirusTotal/yara/releases/latest','kind':'application'},
        {'id':'app-sleuth-kit','name':'The Sleuth Kit','names':['The Sleuth Kit','Sleuth Kit'],'urls':['https://www.sleuthkit.org/sleuthkit/','https://www.sleuthkit.org/sleuthkit/download.php','https://github.com/sleuthkit/sleuthkit'],'hosts':['www.sleuthkit.org','github.com'],'downloadPage':'https://www.sleuthkit.org/sleuthkit/download.php','kind':'application'},
    ])
    extra=[('dell','Dell','https://www.dell.com/support/'),('gmail','Gmail','https://mail.google.com/'),('google-workspace','Google Workspace','https://workspace.google.com/'),('hp','HP','https://support.hp.com/us-en/drivers'),('brother','Brother','https://support.brother.com/'),('canon','Canon','https://www.usa.canon.com/support/software-and-drivers'),('epson','Epson','https://epson.com/Support/sl/s'),('ricoh','Ricoh','https://www.ricoh-usa.com/en/support-and-download'),('xerox','Xerox','https://www.support.xerox.com/en-us'),('lexmark','Lexmark','https://support.lexmark.com/en_us/drivers-downloads.html'),('zebra','Zebra','https://www.zebra.com/us/en/support-downloads.html'),('kyocera','Kyocera','https://www.kyoceradocumentsolutions.us/en/support/downloads.html'),('konica-minolta','Konica Minolta','https://kmbs.konicaminolta.us/support-downloads/'),('sharp','Sharp','https://business.sharpusa.com/product-downloads')]
    for id,n,u in extra:brands.append({'id':'web-'+id,'name':n,'names':[n],'urls':[u],'hosts':[urlsplit(u).hostname],'downloadPage':u,'kind':'website'})
    hosturls={}
    for p in list((ROOT/'data').glob('*.json'))+list((ROOT/'docs').rglob('*.md'))+[ROOT/'README.md']:
        if p.name=='brand-icons.json':continue
        raw=p.read_text()
        for u in re.findall(r'https://[^\s"<>\]\)\}\,]+',raw):
            h=urlsplit(u).hostname
            if h and not any(x in h for x in ['localhost','127.0.0.1','example.']):hosturls.setdefault(h,u)
    hosturls['knowledge.workspace.google.com']='https://knowledge.workspace.google.com/'
    hosts={h for b in brands for h in b['hosts']}
    for h,u in sorted(hosturls.items()):
        shared={'github.com':'https://github.com/','api.github.com':'https://github.com/','raw.githubusercontent.com':'https://github.com/','downloads.sourceforge.net':'https://sourceforge.net/','sourceforge.net':'https://sourceforge.net/','registry.npmjs.org':'https://www.npmjs.com/','aka.ms':'https://www.microsoft.com/download/','go.microsoft.com':'https://www.microsoft.com/download/'}
        match=next((b for b in brands if b.get('toolId') and any(norm_url(v)==norm_url(u) for v in b['urls'])),None)
        route=match['downloadPage'] if match and h not in ('github.com','api.github.com','raw.githubusercontent.com','sourceforge.net','downloads.sourceforge.net') else shared.get(h,'https://'+h+'/')
        # Download/CDN hosts are transport, not meaningful landing pages.
        if route=='https://'+h+'/' and re.match(r'^(?:dl[.]|download[.]|downloads[.]|ftp[.]|packages[.]|cdimage[.]|releases[.]|fastly-cdn[.])',h):
            own=owner(h);candidate=next((b for b in brands if b['kind']!='website' and any(owner(z)==own for z in b['hosts'])),None)
            route=candidate['downloadPage'] if candidate else 'https://'+own+'/'
        if h=='dl.dell.com':route='https://www.dell.com/support/'
        if h=='herbertofury.github.io':route='https://herbertofury.github.io/INTA/'
        if h=='www.sleuthkit.org':route='https://www.sleuthkit.org/sleuthkit/download.php'
        brands.append({'id':'site-'+re.sub('[^a-z0-9]+','-',h),'name':h.removeprefix('www.'),'names':[h.removeprefix('www.')],'urls':[u],'hosts':[h],'downloadPage':route,'kind':'website'})
    # The manifest is the canonical owner of explicit, reviewed supplemental
    # identities. Preserve those records when refreshing source-derived rows.
    generated={b['id'] for b in brands}
    for old in previous['brands']:
        if old['id'] not in generated and old.get('kind') in ('application','website'):
            brands.append(dict(old));generated.add(old['id'])
    # Reviewed names and hosted routes survive source-derived refreshes.
    for brand in brands:
        old=prevby.get(brand['id'],{})
        for key in ('names','urls','hosts'):
            brand[key]=list(dict.fromkeys([*brand.get(key,[]),*old.get(key,[])]))
        if old.get('downloadPage'):brand['downloadPage']=old['downloadPage']
    by_id={b['id']:b for b in brands}
    known={name.casefold() for b in brands for name in b['names']}
    provider_path=ROOT/'data/reputation-sources.json'
    providers=json.loads(provider_path.read_text()).get('providers',[]) if provider_path.exists() else []
    explicit={'github':'app-github','sigmahq':'sigma','yara':'app-yara','wireshark':'wireshark','debian-docs':'app-debian'}
    for provider in providers:
        host=urlsplit(provider['url']).hostname
        brand=by_id.get(explicit.get(provider['id'])) or next((b for b in brands if b['kind']=='website' and host in b['hosts']),None)
        if not brand:continue
        if provider['name'].casefold() not in known:
            brand['names'].append(provider['name']);known.add(provider['name'].casefold())
        if provider['url'] not in brand['urls']:brand['urls'].append(provider['url'])
    for identity,names in {'site-nvd-nist-gov':['NIST NVD'],'site-www-cisa-gov':['CISA KEV'],'site-docs-redhat-com':['Red Hat']}.items():
        if identity not in by_id:continue
        for name in names:
            if name.casefold() not in known:by_id[identity]['names'].append(name);known.add(name.casefold())
    for brand in brands:
        if 'mangodisk.app' in brand['hosts'] or 'downloads.mangodisk.app' in brand['hosts']:brand['downloadPage']='https://mangodisk.app/download'
        if 'www.kimai.org' in brand['hosts']:brand['downloadPage']='https://www.kimai.org/en/download.html'
        if 'www.libreoffice.org' in brand['hosts'] or 'download.documentfoundation.org' in brand['hosts']:brand['downloadPage']='https://www.libreoffice.org/download/'
    def update(b):
        if args.only and b['id'] not in args.only:
            old=prevby.get(b['id'])
            if old and 'inlineIcon' in b:return {**old,'inlineIcon':b['inlineIcon']},None
            return old or b,None
        if b['id'] in ('app-inta','app-inta-rescue','app-media-creator','site-herbertofury-github-io'):
            source=ROOT/'docs/assets/inta-mark.svg'
            raw=source.read_bytes();png,w,h=normalize(raw,'image/svg+xml');digest=hashlib.sha256(png).hexdigest();aid='i'+digest[:20];path=OUT/(digest[:20]+'.png');path.write_bytes(png)
            r={'assetId':aid,'asset':{'path':str(path.relative_to(ROOT)),'sourceUrl':'https://raw.githubusercontent.com/Herbertofury/INTA/main/docs/assets/inta-mark.svg','resolvedSourceUrl':'https://raw.githubusercontent.com/Herbertofury/INTA/main/docs/assets/inta-mark.svg','sourceKind':'Existing first-party INTA interface brand mark','publisherHost':'herbertofury.github.io','sha256':digest,'sourceSha256':hashlib.sha256(raw).hexdigest(),'mimeType':'image/png','width':w,'height':h,'checkedAt':DATE}}
        elif b['id']=='app-yara':r=acquire('github.com','https://github.com/VirusTotal/yara')
        else:
            icon_page=b['urls'][0]
            primary=access.get(b.get('toolId'),{}).get('primary',{})
            if primary.get('kind')=='web' and primary.get('url','').startswith('https://'):
                icon_page=primary['url']
            r=acquire(urlsplit(icon_page).hostname,icon_page)
        if r:b['assetId']=r['assetId'];return b,r
        old=prevby.get(b['id'])
        if old and old.get('assetId'):b['assetId']=old['assetId'];return b,None
        return b,None
    assets=dict(previous['assets']);out=[{**b, **({'assetId':prevby[b['id']]['assetId']} if prevby.get(b['id'],{}).get('assetId') else {})} for b in brands]
    def checkpoint():
        used={b.get('assetId') for b in out}
        partial={'schemaVersion':1,'checkedAt':DATE,'assets':{k:v for k,v in assets.items() if k in used},'brands':out,'coverage':{'tools':len(tools),'brands':len(out),'assets':len(assets),'unresolved':[b['id'] for b in out if not b.get('assetId')]},'failures':list(FAILURES),'failureHistory':failure_history}
        # Keep the complete last-known-good public registry throughout refresh.
        # Partial progress is a diagnostic, never a published replacement.
        if not partial['coverage']['unresolved']:
            temp=ROOT/'data/brand-icons.json.tmp';temp.write_text(json.dumps(partial,indent=2,ensure_ascii=False)+'\n');temp.replace(ROOT/'data/brand-icons.json')
    with concurrent.futures.ThreadPoolExecutor(max_workers=20) as ex:
        pending={ex.submit(update,b):i for i,b in enumerate(brands)}
        for n,future in enumerate(concurrent.futures.as_completed(pending),1):
            b,r=future.result();out[pending[future]]=b
            # Identical bytes can be declared by multiple publisher sites.
            # Preserve the established source instead of racing to replace it.
            if r:assets.setdefault(r['assetId'],r['asset'])
            if n%25==0:checkpoint();print('Resolved',n,'/',len(brands),flush=True)
    used={b.get('assetId') for b in out};assets={k:v for k,v in assets.items() if k in used}
    unresolved=[b['id'] for b in out if not b.get('assetId')]
    manifest={'schemaVersion':1,'checkedAt':DATE,'policy':'Authentic publisher website/app icons, cached locally; publisher favicon cache only when recorded; no generated placeholders. Downloads remain separate from evidence links.','assets':assets,'brands':out,'coverage':{'tools':len(tools),'brands':len(out),'assets':len(assets),'unresolved':unresolved},'failures':FAILURES,'failureHistory':failure_history}
    if unresolved:
        report=ROOT/'reports/brand-icon-refresh.json';report.parent.mkdir(exist_ok=True)
        report.write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n')
    else:
        temp=ROOT/'data/brand-icons.json.tmp';temp.write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n');temp.replace(ROOT/'data/brand-icons.json')
    print(json.dumps(manifest['coverage']),flush=True)
    return bool(unresolved)
GENERIC=set()
if __name__=='__main__':sys.exit(main())
