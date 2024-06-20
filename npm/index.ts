import { env, cwd } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { statSync as stat, existsSync as exists } from 'node:fs';
import { dirname, join, normalize } from 'node:path';

function isDir(path: string): boolean {
    try {
        return stat(path).isDirectory();
    } catch(err) {
        return false;
    }
}

function verifyHooksDir(dir: string) {
    const workingDir = cwd();
    let cur = workingDir;
    while (true) {
        const hooksDir = join(cur, dir);
        if (isDir(hooksDir) && exists(join(cur, '.git'))) {
            return; // Found
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
    verifyHooksDir(dir);

    const git = env['SET_GIT_HOOKS_DIR_GIT'] || 'git';
    const { status, stderr } = spawn(git, ['config', 'core.hooksPath', dir], { env: {} });
    if (status === null || status > 0) {
        throw new Error(`\`${git} config core.hooksPath ${dir}\` failed with status ${status}: ${stderr.toString()}`);
    }
}

try {
    setGitHooksDir('.git-hooks');
    console.log('Set .git-hooks directory to core.hooksPath');
} catch(err) {
    console.error(err); // Do not make this script fail
}
