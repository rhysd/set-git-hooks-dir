import os
import os.path as path
import subprocess
import sys
import site
from setuptools.command.install import install

# XXX: Hacky setuptools command overrides. These are called when the package is built. They only work with sdist because
# the package is built in user local. Wheel package doesn't work because these are called when building .whl binary
# package.
# https://stackoverflow.com/questions/20288711/post-install-script-with-python-setuptools

class CustomInstall(install):
    def run(self):
        super().run()
        try:
            setup('.git-hooks')
        except Exception as exc:
            print('FAILED:', exc, file=sys.stderr)

def _find_dot_git(dirpath: str) -> str:
    # XXX: Locate a Git repository from site-packages directory. This works only when a virtual environment is created
    # within the repository.
    site_packages = site.getsitepackages()

    for cur in site_packages:
        while True:
            hooks_dir = path.join(cur, dirpath)
            dot_git = path.join(cur, '.git')
            if path.isdir(hooks_dir) and path.exists(dot_git):
                return dot_git
            parent = path.dirname(cur)
            if parent == cur:
                break
            cur = parent

    raise Exception(f'Git hooks directory {dirpath} was not found at any root of GitHub repository in {site_packages}')

def setup(dirpath: str) -> None:
    for name in ['SET_GIT_HOOKS_DIR_SKIP', 'GITHUB_ACTION', 'CI', 'JENKINS_URL']:
        if os.getenv(name):
            return

    dirpath = path.normpath(dirpath)
    dot_git = _find_dot_git(dirpath)
    if path.isdir(dot_git):
        with open(path.join(dot_git, 'config'), encoding='utf-8') as file:
            for line in file:
                if line.startswith('\thooksPath = '):
                    return  # core.hooksPath is already configured. Skip

    git = os.getenv('SET_GIT_HOOKS_DIR_GIT') or 'git'
    result = subprocess.run([git, 'config', 'core.hooksPath', dirpath], encoding='utf-8', cwd=path.dirname(dot_git))
    if result.returncode != 0:
        raise Exception(f'`{git} config core.hooksPath {dir} failed with status {result.returncode}: {result.stderr}')
