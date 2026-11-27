"""Exercise the built native CLI against temporary fixtures only."""
from contextlib import suppress
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

DLL = Path(sys.argv.pop(1)).resolve()


class StoragePreviewTests(unittest.TestCase):
    def setUp(self):
        self.fixture = tempfile.TemporaryDirectory()
        self.addCleanup(self.fixture.cleanup)
        self.base = Path(self.fixture.name).resolve()
        self.root = self.base / 'scoped'
        self.root.mkdir()
        self.sub = self.root / 'nested'
        self.sub.mkdir()
        self.old = self.sub / 'old-private-name'
        self.old.write_bytes(b'1234')
        past = time.time() - 86400 * 8
        os.utime(self.old, (past, past))
        self.recent = self.root / 'recent-private-name'
        self.recent.write_bytes(b'123456789')
        self.outside = self.base / 'outside'
        self.outside.mkdir()
        (self.outside / 'not-in-scope').write_bytes(bytes(100))
        self.links = {}
        for name, target, is_dir in [('linkdir', self.outside, True), ('linkfile', self.old, False)]:
            with suppress(OSError, NotImplementedError):
                (self.root / name).symlink_to(target, target_is_directory=is_dir)
                self.links[name] = self.root / name

    def run_cli(self, *args):
        return subprocess.run(
            ['dotnet', str(DLL), '--storage-preview', *map(str, args)],
            capture_output=True, encoding='utf-8', timeout=20, check=False
        )

    def test_counts_and_lifecycle(self):
        before = (self.old.stat().st_mtime_ns, self.old.read_bytes(), self.recent.read_bytes())
        result = self.run_cli(self.root)
        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertEqual((data['schemaVersion'], data['resultType']), (1, 'storage-preview'))
        self.assertEqual((data['filesScanned'], data['logicalBytes']), (2, 13))
        self.assertEqual((data['olderFiles'], data['olderLogicalBytes']), (1, 4))
        self.assertEqual(data['specialEntriesSkipped'], len(self.links))
        self.assertFalse(data['partial'])
        self.assertNotIn('private-name', result.stdout)
        self.assertNotIn(str(self.root), result.stdout)
        self.assertEqual((self.old.stat().st_mtime_ns, self.old.read_bytes(), self.recent.read_bytes()), before)

    def test_age_and_exact_budget(self):
        result = self.run_cli(self.root, 30, 3 + len(self.links))
        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertEqual(data['olderFiles'], 0)
        self.assertFalse(data['partial'])
        limited = self.run_cli(self.root, 1, 1)
        self.assertEqual(limited.returncode, 0, limited.stderr)
        data = json.loads(limited.stdout)
        self.assertEqual(data['entriesScanned'], 1)
        self.assertTrue(data['entryLimitReached'])
        self.assertTrue(data['partial'])

    def test_invalid_targets_and_ranges(self):
        for args in [(), (self.root.anchor,), (self.root / 'missing',), (self.old,), (self.root, '0'),
                     (self.root, '7', '1000001'), (self.root, '7', '1', 'extra'), ('--unknown',)]:
            with self.subTest(args=args):
                result = self.run_cli(*args)
                self.assertEqual(result.returncode, 2, result.stdout)
                self.assertEqual(result.stdout, '')
                self.assertTrue(result.stderr.strip())

    def test_refuses_link_root_and_link_ancestor(self):
        if 'linkdir' not in self.links:
            self.skipTest('Symlink creation is unavailable on this runner.')
        nested_target = self.outside / 'child'
        nested_target.mkdir()
        for folder in [self.links['linkdir'], self.links['linkdir'] / nested_target.name]:
            with self.subTest(folder=folder):
                result = self.run_cli(folder)
                self.assertEqual(result.returncode, 2)
                self.assertEqual(result.stdout, '')

    def test_depth_cap_reports_partial(self):
        if os.name == 'nt':
            self.skipTest('Use Linux runner for deep fixtures without relying on Windows long-path policy.')
        path = self.root
        for _ in range(130):
            path /= 'd'
            path.mkdir()
        (path / 'outside-depth').write_bytes(b'skip')
        result = self.run_cli(self.root)
        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertTrue(data['partial'])
        self.assertEqual(data['depthLimitedDirectories'], 1)
        self.assertEqual(data['logicalBytes'], 13)


if __name__ == '__main__':
    unittest.main()
