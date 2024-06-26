import subprocess
import unittest
import os.path as path
import os
from glob import glob
from shutil import which
from tempfile import TemporaryDirectory
from set_git_hooks_dir import setup

global_python = 'python3' if which('python3') else 'python'

class TestSetGitHooksDir(unittest.TestCase):
    def setUp(self):
        self.temp = TemporaryDirectory()
        self.dir = self.temp.name
        self.package_dir = path.dirname(path.dirname(__file__))
        self._run(['git', 'init'])
        for name in ['GITHUB_ACTION', 'CI', 'JENKINS_URL']:
            if name in os.environ:
                del os.environ[name]

    def tearDown(self):
        self.temp.cleanup()

    def _run(self, cmdline: list[str]) -> None:
        subprocess.run(cmdline, cwd=self.dir, check=True)

    def _find_sdist(self) -> str:
        found = []
        for pat in ['set_git_hooks_dir-*.tar.gz', 'set_git_hooks_dir-*.zip']:
            found.extend(glob(pat, root_dir=self.dir))
        assert len(found) == 1
        return found[0]

    def test_configure_git_hooks(self):
        os.mkdir(path.join(self.dir, '.git-hooks'))

        self._run([global_python, '-m', 'venv', 'venv'])

        local_python = path.join(self.dir, 'venv', 'bin', 'python')
        self._run([local_python, '-m', 'pip', 'install', 'build', '--no-cache-dir', '--no-input', '--verbose'])
        self._run([local_python, '-m', 'build', self.package_dir, '--outdir', '.', '--sdist'])
        self._run([local_python, '-m', 'pip', 'install', self._find_sdist(), '--no-input', '--verbose'])

        gitconfig = path.join(self.dir, '.git', 'config')
        with open(gitconfig, 'r', encoding='utf-8') as file:
            content = file.read()
            assert '\thooksPath = .git-hooks' in content, content

        with self.assertRaises(Exception):
            setup('this-directory-does-not-exist')


if __name__ == '__main__':
    unittest.main()
