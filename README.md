Set Git hooks directory
=======================
[![CI][ci-badge]][ci]

This is a deadly simple tool to manage your [Git hooks][hooks] in your repository and automate the setup for Rust and
Node.js projects. This tool essentially runs the following command in your repository.

```sh
git config core.hooksPath .git-hooks
```

So why don't you run this simple command directly in your terminal? That's because you need to automate this setup.
Otherwise running the command is easily forgotten when checking out your repository. This is the same motivation as
[husky][].

This tool offers the way to automatically setup the Git hooks while preparing for the development in your repository.
And it does nothing else. Note that this automatic setup is skipped when it is run on CI.

This tool now supports the following language/tool, and maybe more languages/tools are supported in the future.

- Rust ([cargo][])
- Node.js ([npm][])
- ...more?

## Prerequisites

- Git 2.9 or later for `core.hooksPath` configuration

## Create your Git hooks directory

Create .git-hooks directory at the root of your repository and put your favorite Git hooks.

```sh
cd path/to/your/repository

# The directory for your Git hooks
mkdir .git-hooks

# For Rust users
echo 'cargo test' > .git-hooks/pre-push

# For npm users
echo 'npm test' > .git-hooks/pre-push

# Manage hooks by Git
git add .git-hooks
```

In this example, `pre-push` hook is created for running tests before `git push`. To know all hooks, see `.git/hooks/`
directory.

```sh
ls .git/hooks/
```

## Setup the hooks directory

### Rust

Add set-git-hooks-dir crate as your dev dependencies and run `cargo check` to do the initial setup.

```sh
cargo add set-git-hooks-dir --dev
cargo check
```

And everything you need to do has been done. When your project's dev-dependencies (e.g. `cargo test`, `cargo check`,
`cargo clippy`) are built at first, `core.hooksPath` is configured.

### npm

Add set-git-hooks-dir npm package as your project's dev dependency.

```sh
npm install set-git-hooks-dir --save-dev

# For yarn users
yarn add set-git-hooks-dir --dev
```

And everything you need to do has been done. The `postinstall` hook of the package automatically configures
`.git-hooks` directory in `.git/config`.

## Customization

Some environment variables can customize the behavior of this tool.

### Git command

This tool uses `git` command by default. However the command can be customized with the `SET_GIT_HOOKS_DIR_GIT`
environment variable.

```sh
export SET_GIT_HOOKS_DIR_GIT=/path/to/git
```

### Skip automatically configure Git hooks directory

Setting `SET_GIT_HOOKS_DIR_SKIP` environment variable skips automatic Git hooks directory configuration.

```sh
export SET_GIT_HOOKS_DIR_SKIP=true
```

## License

This repository is distributed under [the MIT license](LICENSE).


[ci-badge]: https://github.com/rhysd/set-git-hooks-dir/actions/workflows/ci.yaml/badge.svg
[ci]: https://github.com/rhysd/set-git-hooks-dir/actions/workflows/ci.yaml
[hooks]: https://git-scm.com/docs/githooks
[cargo]: https://doc.rust-lang.org/cargo/
[npm]: https://www.npmjs.com/
[husky]: https://typicode.github.io/husky/
[build-script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html
