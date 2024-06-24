Development Guide
=================

This document describes the development of this repository.

The behavior of packages in each language must be consistent to ensure that knowledge can be applied across multiple
programming languages.

## [Rust](./rust)

The [cargo](https://doc.rust-lang.org/cargo/) package manager manages the development.

```sh
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
