# frozen_string_literal: true

require 'pathname'

class SetGitHooksDir
  class GitHooksDirNotFound < StandardError
    def initialize(dir, cwd)
      super "Git hooks directory #{dir} was not found at any root of GitHub repository in #{cwd}"
    end
  end

  class GitConfigHooksFailed < StandardError
    def initialize(git, dir, status, output)
      super "`#{git} config core.hooksPath #{dir}` failed with status #{status}: #{output}"
    end
  end

  SKIP_ENV_VARS = %w[SET_GIT_HOOKS_DIR_SKIP GITHUB_ACTION CI JENKINS_URL].freeze

  class << self
    def setup(hooks_dir)
      return if SKIP_ENV_VARS.lazy.filter_map { |n| ENV[n] }.any? { |v| !v.empty? }

      hooks_dir = Pathname.new hooks_dir
      cwd = Pathname.pwd
      dotgit = cwd.ascend
                  .filter_map { |dir| dir + '.git' if (dir + hooks_dir).directory? }
                  .find { |dotgit| dotgit.exist? }

      raise GitHooksDirNotFound.new(hooks_dir, cwd) unless dotgit
      return if dotgit.directory? && File.foreach(dotgit + 'config').any? { |i| i.start_with? "\thooksPath = " }

      git = ENV['SET_GIT_HOOKS_DIR_GIT']
      git = 'git' if git.nil? || git.empty?
      out = `#{git} config core.hooksPath #{hooks_dir}`
      raise GitConfigHooksFailed.new(git, hooks_dir, $?.exitstatus, out) unless $?.success?
    end
  end
end
