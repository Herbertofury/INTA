#!/usr/bin/env python3
"""Independent evidence for the separately named INTA Recovery-only image."""
import argparse,hashlib,json,os,re,runpy,subprocess,tempfile
from email.parser import Parser
from pathlib import Path

def checksum(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()

def product(iso,sha):
    assert iso.is_file() and iso.name=='inta-rescue-recovery-amd64.iso' and re.fullmatch('[0-9a-f]{64}',sha)
    assert checksum(iso)==sha
    with iso.open('rb') as inp:
        inp.seek(16*2048); header=inp.read(2048)
    assert header[:7]==b'\x01CD001\x01'
    assert header[40:72].rstrip()==b'INTA_RECOVERY',header[40:72]
    assert header[574:702].rstrip()==b'INTA RESCUE RECOVERY',header[574:702]

def absence(iso,sha,rev,folder):
    with tempfile.TemporaryDirectory(prefix='inta-recovery-absence-') as tmp:
        fs=Path(tmp)/'live.squashfs'
        subprocess.run(['xorriso','-osirrox','on','-indev',f'stdio:{iso}','-extract','/live/filesystem.squashfs',fs],check=True,stderr=subprocess.DEVNULL,stdout=subprocess.DEVNULL)
        def take(name): return subprocess.check_output(['unsquashfs','-processors','1','-cat',fs,name])
        assert take('etc/inta-source-revision').decode().strip()==rev
        assert take('etc/inta-edition').decode().strip()=='recovery-only-v1'
        assert 'VERSION_CODENAME=trixie' in take('usr/lib/os-release').decode()
        listing=subprocess.check_output(['unsquashfs','-ls',fs],text=True).splitlines()
        names=[n.removeprefix('squashfs-root/') for n in listing if n.startswith('squashfs-root/')]
        parser=Parser(); packages=[]
        for record in take('var/lib/dpkg/status').decode().split('\n\n'):
            fields=parser.parsestr(record)
            if fields.get('Status')=='install ok installed':
                packages.append((fields['Package'],(fields.get('Source') or fields['Package']).split()[0]))
        forbidden=re.compile(r'^(?:clamav(?:$|-)|libclamav|libclamunrar|libfreshclam|freshclam|clamdscan|clamtk|yara(?:$|-)|libyara|yara-x)')
        forbidden_names=re.compile(r'(?:^|/)(?:clamscan|clamd|clamdscan|clamconf|freshclam|sigtool|yara|yarac|yr|claude|codex|libclamav[^/]*\.so(?:\..*)?|libfreshclam[^/]*\.so(?:\..*)?|libyara[^/]*\.so(?:\..*)?)$')
        bad_pkg=[p for p in packages if any(forbidden.match(part) for part in p)]
        bad_files=[p for p in names if forbidden_names.search(p) or p.startswith(('opt/inta-ai/','usr/local/lib/node_modules/@anthropic-ai/','usr/local/lib/node_modules/@openai/'))]
        assert not bad_pkg and not bad_files,(bad_pkg,bad_files)
        assert {'gddrescue','dosfstools','testdisk','e2fsprogs','gparted','clonezilla'} <= {p for p,_ in packages}
        assert len(packages)>250 and 'usr/local/bin/inta-rescue' in names
        evidence={'scope':'exact Debian packages and source packages, executable and library filenames, shipped AI payload directories, edition and build marker, PVD application+volume, required recovery tools',
            'sha256':sha,'bytes':iso.stat().st_size,'revision':rev,'suite':'trixie','volumeID':'INTA_RECOVERY','applicationID':'INTA RESCUE RECOVERY','installedPackages':len(packages),'squashfsPaths':len(names),'scannerPackages':bad_pkg,'scannerFilesOrAIClients':bad_files}
    assert checksum(iso)==sha
    (folder/'absence.json').write_text(json.dumps(evidence,indent=2)+'\n')
    print(json.dumps(evidence))

def repair(repo,iso,sha,rev,folder):
    helpers=runpy.run_path(str(repo/'native/workbench/scripts/smoke-rescue-repair.py'))
    with tempfile.TemporaryDirectory(prefix='inta-recovery-fatrepair-') as temp:
        work=Path(temp);source,target,control,old,file_hash=helpers['prepare_fixture'](work)
        kernel,initrd=helpers['extract_boot_files'](iso,work)
        cmdline=('boot=live components username=inta hostname=inta quiet console=ttyS0 live-media=/dev/vda '
                 'systemd.mount-extra=LABEL=INTALAB:/run/inta-lab:iso9660:ro '
                 'systemd.run="/bin/bash /run/inta-lab/repair.sh" systemd.run_success_action=poweroff systemd.run_failure_action=poweroff')
        serial=folder/'repair-guest-serial.log'
        kvm=os.access('/dev/kvm',os.R_OK|os.W_OK)
        cmd=['qemu-system-x86_64','-machine','q35,accel='+('kvm' if kvm else 'tcg'),'-cpu','host' if kvm else 'max','-smp','2','-m','2048','-no-reboot','-nic','none','-display','none','-monitor','none','-serial',f'file:{serial}','-kernel',kernel,'-initrd',initrd,'-append',cmdline]
        for p,ro in [(iso,True),(control,True),(source,True),(target,False)]:
            assert not any(c in str(p) for c in ',\r\n')
            cmd.extend(['-drive',f'if=virtio,format=raw,readonly={"on" if ro else "off"},file={p}'])
        subprocess.run(cmd,check=True,timeout=220 if kvm else 600)
        log=serial.read_text(errors='replace').splitlines()
        assert 'INTA_REPAIR_PASS' in log and f'INTA_RECOVERED_SHA256={file_hash}' in log,log[-45:]
        assert checksum(source)==old and checksum(iso)==sha
        recovered=work/'from-destination.img';reread=work/'restored.txt'
        subprocess.run(['mcopy','-i',target,'::recovered.img',recovered],check=True)
        subprocess.run(['mcopy','-i',recovered,'::RECOVER.TXT',reread],check=True)
        assert checksum(reread)==file_hash
        assert subprocess.run(['fsck.fat','-n',recovered],check=False,stdout=subprocess.DEVNULL).returncode==0
        evidence={'imageSHA256':sha,'revision':rev,'sourceImageBeforeAfter':old,'sourceReadOnly':True,'originalFileAndRestoredFileSHA256':file_hash,'restoredBytes':reread.stat().st_size,'copyFsckReadOnlyExit':0,'QEMU':'KVM' if kvm else 'TCG','guestNetworkDevices':0,'writableGuestDisks':1,'writableDisk':'new synthetic FAT destination only'}
        print(json.dumps(evidence))
        (folder/'repair-proof.json').write_text(json.dumps(evidence,indent=2)+'\n')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('action',choices=['absence','repair']);p.add_argument('iso',type=Path);p.add_argument('sha256');p.add_argument('revision');p.add_argument('folder',type=Path);p.add_argument('--repo',type=Path,default=Path(__file__).resolve().parents[1])
    a=p.parse_args()
    if not __debug__:
        raise SystemExit('Verification requires Python assertions.')
    a.iso=a.iso.resolve(strict=True)
    if not re.fullmatch('[0-9a-f]{40}',a.revision):
        raise ValueError('Invalid product source revision')
    product(a.iso,a.sha256)
    a.folder.mkdir(parents=True,exist_ok=False)
    if a.action=='absence':absence(a.iso,a.sha256,a.revision,a.folder)
    else:repair(a.repo,a.iso,a.sha256,a.revision,a.folder)
