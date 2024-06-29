require 'set_git_hooks_dir'

puts 'Setting Git hooks from bundler plugin'
SetGitHooksDir.setup '.git-hooks'
