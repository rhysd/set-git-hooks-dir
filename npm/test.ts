import * as assert from 'node:assert';
import { chdir, cwd } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { mkdirSync as mkdir, readFileSync as read } from 'node:fs';
import { join } from 'node:path';
import { dirSync as tmpdir, type DirResult } from 'tmp';
import { setGitHooksDir } from './index.js';

describe('setGitHooksDir', function () {
    const origCwd = cwd();
    let tmpDir: DirResult;

    before(function () {
        tmpDir = tmpdir({ unsafeCleanup: true });
        chdir(tmpDir.name);
        const { status, stderr } = spawn('git', ['init']);
        assert.ok(status !== null && status === 0, stderr.toString());
        mkdir('this-is-test');
    });

    after(function () {
        chdir(origCwd);
        if (tmpDir) {
            tmpDir.removeCallback();
        }
    });

    it('configures hooks directory', function () {
        setGitHooksDir('this-is-test');
        const gitconfig = read(join('.git', 'config'), 'utf-8');
        assert.ok(gitconfig.includes('hooksPath = this-is-test'), gitconfig);
    });
});

