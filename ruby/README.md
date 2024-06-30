`set_git_hooks_dir` Ruby package
================================
[![gem][gem-badge]][gem]

## Usage

See https://github.com/rhysd/set-git-hooks-dir#readme for the usage.

## Details

This gem aims to be used as both bundler and rubygems plugins so that this gem can hook the package installation and
configure Git hooks properly.

So why can't this gem be a simple rubygems plugin? The answer is that bundler doesn't run rubygems plugin's install
hooks on `bundle install`. bundler downloads gems from rubygems.org and stores the downloaded gems to the
[`BUNDLE_PATH`][path] directory and makes them loadable. However, this doesn't mean *installing* the gems. It just
*stores*. It is very confusing that `gem install` *installs* a gem but `bundle install` does not *install* a gem. The
following issue mentions this behavior:

https://github.com/rubygems/bundler/issues/5429

> Running `bundle install` with `path` gems does not install those gems. The `path` feature is a shortcut to add the
> path to `$LOAD_PATH`, and does not do any of the usual gem installation procedures.

The detailed behavior of this gem in each cases are as follows:

- On `bundle install`, this gem works as a bundler plugin. bundler loads the plugin on installation and it configures
  Git hooks.
- On `gem install`, this gem works as a rubygems plugin. rubygems runs a install hook of the plugin and the hook
  configures Git hooks.

## License

This npm package is distributed under [the MIT license](LICENSE).

[gem-badge]: https://img.shields.io/gem/v/set_git_hooks_dir
[gem]: https://rubygems.org/gems/set_git_hooks_dir
[path]: https://bundler.io/v1.12/man/bundle-config.1.html#LIST-OF-AVAILABLE-KEYS
