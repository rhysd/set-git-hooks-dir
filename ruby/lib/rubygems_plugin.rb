# frozen_string_literal: true

require 'rubygems/installer'
require_relative 'set_git_hooks_dir'

Gem.post_install do |installer|
  begin
    SetGitHooksDir.setup '.git-hooks' if installer.spec.name == 'set_git_hooks_dir'
  rescue StandardError => err
    STDERR.puts "set_git_hooks_dir: #{err}"
  end
end
