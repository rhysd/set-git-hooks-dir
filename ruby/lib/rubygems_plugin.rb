# frozen_string_literal: true

require 'rubygems/installer'
require_relative 'set_git_hooks_dir'

Gem.post_install do |installer|
  puts "Setting Git hooks from rubygems plugin #{installer.spec.name}"
  SetGitHooksDir.setup '.git-hooks' if installer.spec.name == 'set_git_hooks_dir'
end
