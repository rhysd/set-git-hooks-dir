# frozen_string_literal: true

require 'pathname'

class SetGitHooksDir
  class << self
    SKIP_ENV_VARS = %w[SET_GIT_HOOKS_DIR_SKIP GITHUB_ACTION CI JENKINS_URL].freeze

    def setup(hooks_dir)
      if SKIP_ENV_VARS.lazy.filter_map { |n| ENV[n] }.any? { |v| !v.empty? }
        puts 'CI environment is detected. Skip setting Git hooks'
        return
      end

      puts "Start setting hooks directory: #{hooks_dir}"

      hooks_dir = Pathname.new hooks_dir
      cwd = Pathname.pwd
      dotgit = cwd.ascend
                  .filter_map { |dir| dir + '.git' if (dir + hooks_dir).directory? }
                  .find { |dotgit| dotgit.exist? }

      puts "Repository config directory: #{dotgit}"

      raise StandardError.new("Git hooks directory #{hooks_dir} was not found at any root of GitHub repository in #{cwd}") unless dotgit
      if dotgit.directory? && File.read(dotgit + 'config').include?("\n\thooksPath = ")
        puts 'core.hooksPath is already set. Skip setting Git hooks'
        return
      end

      git = ENV['SET_GIT_HOOKS_DIR_GIT']
      git = 'git' if git.nil? || git.empty?
      puts "Running `#{git} config core.hooksPath #{hooks_dir}`"
      out = `#{git} config core.hooksPath #{hooks_dir}`
      raise StandardError.new("`#{git} config core.hooksPath #{hooks_dir}` failed with status #{$?.exitstatus}: #{out}") unless $?.success?
      puts 'Done!'
    end
  end
end
