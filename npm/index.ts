import { env, cwd } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { statSync as stat, existsSync as exists, readFileSync as read } from 'node:fs';
import { dirname, join, normalize } from 'node:path';

function isDir(path: string): boolean {
    try {
        return stat(path).isDirectory();
    } catch(err) {
        return false;
    }
}

function findDotGitPath(dir: string): string {
    const workingDir = cwd();
    let cur = workingDir;
    while (true) {
        const hooksDir = join(cur, dir);
        const dotGitPath = join(cur, '.git');
        if (isDir(hooksDir) && exists(dotGitPath)) {
            return dotGitPath;
        }
        const parent = dirname(cur);
        if (parent === cur) {
            throw new Error(`Git hooks directory ${dir} was not found at any root of GitHub repository in ${workingDir}`);
        }
        cur = parent;
    }
}

export function setGitHooksDir(dir: string) {
    dir = normalize(dir);
    const dotGit = findDotGitPath(dir);
    if (isDir(dotGit)) {
        const config = read(join(dotGit, 'config'), 'utf-8');
        if (/\n\thooksPath = /.test(config)) {
            return; // core.hooksPath is already configured. Skip
        }
    }

    const git = env['SET_GIT_HOOKS_DIR_GIT'] || 'git';
    const { status, stderr } = spawn(git, ['config', 'core.hooksPath', dir], { env: {} });
    if (status === null || status > 0) {
        throw new Error(`\`${git} config core.hooksPath ${dir}\` failed with status ${status}: ${stderr.toString()}`);
    }
}

setGitHooksDir('.git-hooks');
