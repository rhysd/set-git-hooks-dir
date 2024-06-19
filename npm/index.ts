import { env, cwd } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { statSync as stat, existsSync as exists } from 'node:fs';
import { dirname, join } from 'node:path';

function isDir(path: string): boolean {
    try {
        return stat(path).isDirectory();
    } catch(err) {
        return false;
    }
}

export function setGitHooksDir(dirName: string) {
    const workingDir = cwd();
    let cur = workingDir;
    while (true) {
        const hooksDir = join(cur, dirName);
        if (isDir(hooksDir) && exists(join(cur, '.git'))) {
            break; // Found
        }
        const parent = dirname(cur);
        if (parent === cur) {
            throw new Error(`Git hooks directory ${dirName} was not found in ${workingDir}`);
        }
        cur = parent;
    }

    const git = env['SET_GIT_HOOKS_DIR_GIT'] || 'git';
    const { status, stderr } = spawn(git, ['config', 'core.hooksPath', dirName], { env: {} });
    if (status === null || status > 0) {
        throw new Error(`\`${git} config core.hooksPath ${dirName}\` failed with status ${status}: ${stderr.toString()}`);
    }
}

try {
    setGitHooksDir('.git-hooks');
    console.log('Set .git-hooks directory to core.hooksPath');
} catch(err) {
    console.error(err); // Do not make this script fail
}
