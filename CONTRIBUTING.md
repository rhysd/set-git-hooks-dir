Development Guide
=================

This document describes the development of this repository.

The behavior of packages in each language must be consistent to ensure that knowledge can be applied across multiple
programming languages.

## [Rust](./rust)

The [cargo](https://doc.rust-lang.org/cargo/) package manager manages the development.

```sh
cd ./rust

# Test
cargo test

# Lint with clippy
cargo clippy --all

# Format with rustfmt
cargo fmt

# Release after bumping the version in Cargo.toml
cargo publish
```

## [npm](./npm)

The [npm](https://www.npmjs.com/) package manager manages the development.

```sh
cd ./npm

# Install dependencies
npm install

# Build TypeScript sources into JavaScript
npm run build

# Run tests with Mocha
npm test

# Lint
npm run lint

# Format with prettier
npm run fmt

# Release after bumping the version in package.json
npm publish
```

## [Python](./python)

Create the Python virtual environment at first:

```sh
cd ./python

# On macOS
python3 -m venv venv
source ./venv/bin/activate

# On Windows
py -m venv venv
./venv/bin/activate

# On others
python -m venv venv
source ./venv/bin/activate
```

The virtual env contains all tools for developing Python package.

```sh
# Install all dependencies and install itself as develop mode
python -m pip install '.[dev]'

# Run tests
python ./test/test_setup.py

# Type checking with mypy
mypy

# Lint with flake8
flake8 ./set_git_hooks_dir ./test

# Format with black
black ./set_git_hooks_dir ./test
```

The [`publish.bash`](./python/scripts/publish.bash) script builds the Python package and uploads it to [PyPI](https://pypi.org/).

```sh
# Run this script after bumping the version in pyproject.toml
bash ./scripts/publish.bash
```

**Note:** Only a sdist package should be uploaded. Do not upload wheel package (`.whl`). The wheel package does not
allow running an arbitrary code on installing the package. We rely on the hook to configure Git hooks.

## [Ruby](./python)

All tasks are managed by `rake` command which is a part of the Ruby standard toolchain.

```sh
cd ./ruby

# Run tests
rake test

# Run checks
rake check

# Run code formtter
rake fmt

# Release this gem after bumping the version in the set_git_hooks_dir.gemspec file
rake release
```

This gem works as both bundler plugin and rubygems plugin. As the ['Details' section](./ruby/README.md) says, this is
because bundler does not run rubygems plugin's install hooks on `bundle install`. Please read the section for the
details.

The implementation level behavior of the package is as follows:

- `bundle install` : This gem works as a [bundler plugin][bundler-plugin]. bundler loads [`plugins.rb`](./ruby/plugins.rb)
  on the installation and the script runs `git config` while the plugin setup.
- `gem install` : This gem works as a [rubygems plugin][rubygems-plugin]. rubygems runs `Gem.post_install` hook in
  [`rubygems_plugin.rb`](./ruby/lib/rubygems_plugin.rb). The hook runs `git config`. Note that the hooks is called on
  each package installation. So the hook only runs the command only when `set_git_hooks_dir` package is installed.

[bundler-plugin]: https://bundler.io/guides/bundler_plugins.html
[rubygems-plugin]: https://guides.rubygems.org/plugins/

## Package versioning

Versioning of all packages in this project conforms [Semantic versioning 2.0.0][semver].

In addition,

- The major version and minor version must be synchronized among all packages.
- The patch version can differ among the packages. For example, when some fix is added to pip package, we bump the patch
  version of the pip package and publish the new version. Other packages' patch versions are not bumped.

This means that when new compatible or breaking change is added, all the packages should implement the same feature. The
major or minor version of all the packages must be bumped and the new packages must be published simultaneously.

This is important to provide the same functionalities in a language-agnostic manner.

[semver]: https://semver.org/

## CI

All CI jobs are defined in the [CI workflow](.github/workflows/ci). All the following combinations should be tested:

- Languages
  - Rust
  - Node.js
  - Python
  - Ruby
- OS
  - Linux
  - macOS
  - Windows
- Checks
  - Unit/Integration tests
  - Lints
  - Code formatting
