"""Windows, attended integration check using a previously prepared Ollama model store."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import socket
import subprocess
import time


def sha(path):
    with path.open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def listening():
    with socket.socket() as probe:
        probe.settimeout(.25)
        return probe.connect_ex(('127.0.0.1', 11434)) == 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('workbench', 'capabilities', 'model-store', 'new-evidence-directory'):
        parser.add_argument(name.replace('-', '_'), type=Path)
    args = parser.parse_args()
    qa = args.new_evidence_directory.resolve()
    qa.mkdir(parents=True, exist_ok=False)
    state = qa / 'state'
    model = 'qwen2.5:0.5b'
    digest = 'a8b0c51577010a279d933d14c2a8ab4b268079d44c5c8830c0a93900f1827c67'
    ollama = Path(os.environ['LOCALAPPDATA']) / 'Programs/Ollama/ollama.exe'
    shell = Path(os.environ['SystemRoot']) / 'System32/WindowsPowerShell/v1.0/powershell.exe'
    # Validate the fixed local publisher binary before starting it. No model or runtime is downloaded.
    script = r"""$s=Get-AuthenticodeSignature -LiteralPath "$env:LOCALAPPDATA/Programs/Ollama/ollama.exe"
if ($s.Status -ne 'Valid' -or $s.SignerCertificate.Subject -notmatch '^CN=Ollama Inc\.,') {exit 1}
$s.SignerCertificate.Subject"""
    shell_env = dict(os.environ, PSModulePath=str(shell.parent / 'Modules'))
    signer = subprocess.check_output([str(shell), '-NoProfile', '-NonInteractive', '-Command', script], env=shell_env,text=True).strip()
    env = dict(os.environ, INTA_DATA_DIR=str(state), OLLAMA_HOST='127.0.0.1:11434',
               OLLAMA_NO_CLOUD='1', OLLAMA_MODELS=str(args.model_store.resolve()), OLLAMA_KEEP_ALIVE='0')

    def execute(binary, words, success=True):
        result = subprocess.run([str(binary), *map(str, words)], env=env, capture_output=True,
                                text=True, encoding='utf-8', timeout=210)
        if success:
            if result.returncode:
                raise RuntimeError(f'{words[0:2]}: {result.stderr.strip()}')
            return json.loads(result.stdout)
        assert result.returncode, 'Refusal expected'

    def work(*words, **kwargs):
        return execute(args.workbench, words, **kwargs)

    def forge(*words, **kwargs):
        result = execute(args.capabilities, ['forge', *words], **kwargs)
        if result:
            assert result['ok']
            return result['result']

    assert not listening(), 'The test requires an unused loopback port 11434'
    verified = work('assistant', 'prepare', args.model_store.resolve(), '--approve')
    assert verified['digest'] == digest
    assert work('assistant', 'server-plan')['digest'] == digest
    request = 'Record the installed memory capacity and the active Windows power plan.'
    destination = qa / 'baseline.json'
    with (qa/'ollama.log').open('xb') as log:
        service = subprocess.Popen([str(ollama), 'serve'], env=env, cwd=qa, stdin=subprocess.DEVNULL,
                                   stdout=log, stderr=subprocess.STDOUT, creationflags=subprocess.CREATE_NO_WINDOW)
        try:
            for _ in range(120):
                if service.poll() is not None:
                    raise RuntimeError('The owned server exited before readiness')
                if listening():
                    break
                time.sleep(.25)
            # Prevent testing against another server that happened to occupy the port.
            owner = subprocess.check_output([str(shell),'-NoProfile','-NonInteractive','-Command',
                '(Get-NetTCPConnection -LocalAddress 127.0.0.1 -LocalPort 11434 -State Listen -ErrorAction Stop).OwningProcess'],env=shell_env,text=True)
            assert int(owner.strip()) == service.pid
            result = work('ai','forge',model,digest,request,destination,'--approve')
            assert result['selectedOperations'] == ['system.memory','system.power'], result
            assert result['state']=='exported' and result['testsPassed']==3 and result['modelDigest']==digest
            raw = destination.read_bytes()
            assert request.encode() not in raw
            actual = json.loads(raw)
            assert all(step['input']=={} for step in actual['steps'])
            check = forge('verify', destination)
            installed = forge('import', destination, '--store', qa/'recipes')
            observation = forge('run', result['id'], '--store',qa/'recipes')
            assert all(v['manifestSha256'] == result['manifestSha256'] for v in (check,installed,observation))
            assert installed['state']=='imported' and observation['testsPassed']==3
            assert observation['observations'][0]['totalPhysicalBytes'] > 0
            assert observation['observations'][1]['activeId'] in observation['observations'][1]['planIds']
            assert forge('import',destination,'--store',qa/'recipes')['state']=='already-imported'
            work('ai','forge',model,digest,request,destination,'--approve',success=False)
            assert destination.read_bytes()==raw
            for words in [(model,'0'*64,request,qa/'wrong.json','--approve'),
                          (model,digest,request,qa/'unapproved.json')]:
                work('ai','forge',*words,success=False)
                assert not words[3].exists()
            other = work('ai','forge',model,digest,'Count Windows startup registrations.',qa/'startup.json','--approve')
            assert other['selectedOperations']==['startup.counts'] and other['testsPassed']==6
        finally:
            service.terminate()
            service.wait(timeout=30)
    assert not listening()
    report = {'schemaVersion':1,'platform':'Windows x64','transport':'WinHTTP 127.0.0.1, cloud disabled',
              'ollamaSigner':signer,'workbenchSha256':sha(args.workbench),'capabilitiesSha256':sha(args.capabilities),
              'model':model,'modelManifestSha256':digest,'modelBytesVerified':verified['verifiedBytes'],
              'export':result,'secondSelection':other['selectedOperations'],'secondSelectionVectors':other['testsPassed'],
              'verified':check,'imported':installed['state'],'storedRunAssertionsPassed':observation['testsPassed'],
              'duplicateImport':'already-imported','checks':['RAM and power selected from natural language',
              'startup separately selected from natural language','cross-process Forge v1 verify/import/run',
              'baseline matches independent subsequent observations','original prompt excluded from recipe',
              'existing file preserved','changed model digest rejected','missing consent rejected','owned service returned to idle']}
    (qa/'receipt.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report,indent=2))


if __name__ == '__main__':
    main()
