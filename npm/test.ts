import * as assert from 'node:assert';
import { chdir, cwd } from 'node:process';
import { spawnSync as spawn } from 'node:child_process';
import { mkdirSync as mkdir, readFileSync as read } from 'node:fs';
import { join } from 'node:path';
import { env } from 'node:process';
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
        for (const name of ['GITHUB_ACTION', 'CI']) {
            delete env[name];
        }
    });

    after(function () {
        chdir(origCwd);
        if (tmpDir) {
            tmpDir.removeCallback();
        }
    });

    it('configures hooks directory', function () {
        assert.throws(
            () => setGitHooksDir('this-directory-does-not-exist'),
            /Git hooks directory this-directory-does-not-exist was not found/,
        );

        mkdir('this-is-test');

        // Hooks are not set on CI
        {
            env['GITHUB_ACTION'] = 'true';
            setGitHooksDir('this-is-test');
            const gitconfig = read(join('.git', 'config'), 'utf-8');
            assert.ok(!gitconfig.includes('hooksPath = this-is-test'), gitconfig);
            delete env['GITHUB_ACTION'];
        }

        {
            setGitHooksDir('this-is-test');
            const gitconfig = read(join('.git', 'config'), 'utf-8');
            assert.ok(gitconfig.includes('hooksPath = this-is-test'), gitconfig);
        }

        {
            mkdir('this-is-test-2');
            setGitHooksDir('this-is-test-2');
            const gitconfig = read(join('.git', 'config'), 'utf-8');
            // Do not override existing configuration
            assert.ok(gitconfig.includes('hooksPath = this-is-test'), gitconfig);
        }
    });
});
