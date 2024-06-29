# frozen_string_literal: true

require 'rubygems/installer'
require_relative 'set_git_hooks_dir'

Gem.post_install do
  SetGitHooksDir.setup '.git-hooks'
end
