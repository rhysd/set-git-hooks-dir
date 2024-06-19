import { env, argv, exit } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { statSync as stat } from 'node:fs';
import { normalize } from 'node:path';

export function setGitHooksDir(dirPath: string) {
    const git = env['SET_GIT_HOOKS_DIR_GIT'] || 'git';
    const dir = normalize(dirPath);
    if (!stat(dir).isDirectory()) {
        throw new Error(`The path is not a directory: ${dir}`);
    }
    const { status, stderr } = spawn(git, ['config', 'core.hooksPath', dir], { env: {} });
    if (status === null || status > 0) {
        throw new Error(`\`${git} config core.hooksPath ${dir}\` failed with status ${status}: ${stderr.toString()}`);
    }
}

export function main() {
    if (argv.length <= 2 || argv.includes('--help') || argv.includes('-h')) {
        console.log('USAGE: npx set-git-hooks-dir path/to/hooks/dir\n\nSee https://github.com/rhysd/set-git-hooks-dir#readme');
        exit(1);
    }
    setGitHooksDir(argv[2]);
}
