## Usage

See https://github.com/rhysd/set-git-hooks-dir#readme for the usage.

## Advanced usage

If you want to configure the Git hooks by yourself, it is still possible by disabling automatic setup by
disabling the feature,

```sh
cargo add set-git-hooks-dir --dev --no-default-features
```

and writing your own [build script][build-script].

```rust
fn main() {
    #[cfg(debug_assertions)]
    let _ = set_git_hooks_dir::setup(".git-hooks");
}
```

## License

This crate is distributed under [the MIT license](LICENSE).

[build-script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
