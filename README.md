Set Git hooks directory
=======================
[![CI][ci-badge]][ci]

This is a deadly simple Git hooks directory setup tool available as a tiny Rust crate or npm package.
This tool essentially runs the following command in your repository.

```sh
git config core.hooksPath path/to/dir
```

So why don't you run this simple command directly in your terminal? That's because you need to automate this setup.
Otherwise running the command is easily forgotten when checking out your repository. This is the same motivation as
[husky][].

This tool offers the way to automatically setup the Git hooks while preparing the development in your repository.
And it does nothing else.

This tool now supports the following language/tool, and maybe more languages/tools are supported in the future.

- Rust ([cargo][])
- Node.js ([npm][])
- ...more?

## Prerequisites

- Git 2.9 or later for `core.hooksPath` configuration

## Create your Git hooks directory

Create your favorite directory for putting Git hooks in your repository.

```sh
# The directory for your Git hooks
mkdir .git-hooks

# For Rust users
echo 'cargo test' > .git-hooks/pre-push

# For npm users
echo 'npm test' > .git-hooks/pre-push
```

In this example, `pre-push` hook is created. To know all hooks, see `.git/hooks/` directory.

```sh
ls .git/hooks/
```

## Setup the hooks directory

### Rust

Add set-git-hooks-dir crate as your dev dependencies.

```sh
cargo add set-git-hooks-dir --dev
```

Then write the following code in your [`build.rs` build script][cargo-build-script]. The `debug_assertions` gate avoids
configuring Git hooks when it is built in release mode (e.g. on `cargo install`).

```rust
fn main() {
    // This block runs only when debug build is enabled.
    #[cfg(debug_assertions)]
    {
        set_git_hooks_dir::install(".git-hooks").unwrap();
    }
}
```

Finally run `cargo check`, `cargo test`, or `cargo clippy`. It configures `core.hooksPath` in `.git/config`.

### npm

Add set-git-hooks-dir npm package as your dev dependencies.

```sh
npm install set-git-hooks-dir --save-dev

# When you use yarn
yarn add set-git-hooks-dir --dev
```

Then add the `prepare` npm hook in your package.json.

```json
{
    ...
    "scripts": {
        "prepare": "set-git-hooks-dir .git-hooks"
    }
}
```

Finally run `npm install`. It configures `core.hooksPath` in `.git/config`.

## OS-specific hooks

You may need OS-specific Git hooks.

For example, Windows sometimes needs Powershell so the same hooks are not available as Linux. Such complicated setup
is possible with this tool.

Prepare OS-specific hooks in different directory. Let's say Windows-specific hooks are in `.git-hooks/windows/` and
hooks for other OSes are in `.git-hooks/others/`.

### Rust

[`cfg` gate][cfg] is useful for the conditional setup.

```rust
fn main() {
    // Setup for Windows
    #[cfg(all(debug_assertions, windows))]
    {
        set_git_hooks_dir::install(r#".git-hooks\windows"#).unwrap();
    }
    // Setup for others
    #[cfg(all(debug_assertions, not(windows)))]
    {
        set_git_hooks_dir::install(".git-hooks/others").unwrap();
    }
}
```

### npm

Create a small script which does the conditional setup in JavaScript.

Let's say we have `scripts/setup-git-hooks.mjs`:

```javascript
import { setGitHooksDir } from 'set-git-hooks-dir';

if (process.platform === 'win32') {
    setGitHooksDir('.git-hooks\\windows');
} else {
    setGitHooksDir('.git-hooks/others');
}
```

Instead of running `set-git-hooks-dir` command, run the script in `prepare` hook:

```json
{
    ...
    "scripts": {
        "prepare": "node scripts/setup-git-hooks.mjs"
    }
}
```

## Custom Git command

This tool uses `git` command by default. However the command can be customized with the `SET_GIT_HOOKS_DIR_GIT`
environment variable.

```sh
export SET_GIT_HOOKS_DIR_GIT=/path/to/git
```

## License

This repository is distributed under [the MIT license](LICENSE).


[ci-badge]: https://github.com/rhysd/set-git-hooks-dir/actions/workflows/ci.yaml/badge.svg
[ci]: https://github.com/rhysd/set-git-hooks-dir/actions/workflows/ci.yaml
[cargo]: https://doc.rust-lang.org/cargo/
[npm]: https://www.npmjs.com/
[husky]: https://typicode.github.io/husky/
[cargo-build-script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html
