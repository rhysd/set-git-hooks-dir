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

# Release
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

# Release
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

# Release gem. You need to run `gem signin` in advance
rake release
```

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
