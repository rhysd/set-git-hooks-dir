# frozen_string_literal: true

# bundler adds ./lib to load paths before loading this file so `require` works
require 'set_git_hooks_dir'

# XXX: `bundle install` does not run rubygems install hooks when it does *path* installs. This is because bundler's
# *path* install only *puts* dependencies in some loadable path and does *not* install them as gem. This means that
# `bundle install` does not mean `gem install` in terms of gem installation.
#
# https://github.com/rubygems/bundler/issues/5429
#
# To enable automatic hooks configuration via `bundle install`, this package needs to be installed as a bundler plugin.
# For the case of `gem install`, `lib/rubygems_plugin.rb` remains (though I don't know the use case).

SetGitHooksDir.setup '.git-hooks'
