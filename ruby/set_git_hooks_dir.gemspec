# frozen_string_literal: true

Gem::Specification.new do |s|
  s.name = 'set_git_hooks_dir'
  s.version = '0.0.7'
  s.summary = 'Deadly simple Git hooks directory setup tool'
  s.description = 'set_git_hooks_dir is a deadly simple gem to configure your Git hooks on package installation'
  s.authors = ['rhysd']
  s.email = ['lin90162@yahoo.co.jp']
  s.files = ['lib/set_git_hooks_dir.rb', 'lib/rubygems_plugin.rb', 'plugins.rb', 'LICENSE']
  s.homepage = 'https://github.com/rhysd/set-git-hooks-dir'
  s.license = 'MIT'
  s.required_ruby_version = '>= 2.6'
  s.require_paths = ['lib']
  s.metadata['homepage_uri'] = s.homepage
  s.metadata['source_code_uri'] = 'https://github.com/rhysd/set-git-hooks-dir/tree/main/ruby'
end
