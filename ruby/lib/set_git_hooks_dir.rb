require 'pathname'

class SetGitHooksDir
  class << self
    SKIP_ENV_VARS = ['SET_GIT_HOOKS_DIR_SKIP', 'GITHUB_ACTION', 'CI', 'JENKINS_URL']

    def setup(hooks_dir)
      return if SKIP_ENV_VARS.lazy.filter_map { |n| ENV[n] }.any? { |v| !v.empty? }

      hooks_dir = Pathname.new hooks_dir
      cwd = Pathname.pwd
      dotgit = cwd.ascend
        .filter_map { |dir| dir + '.git' if (dir + hooks_dir).directory? }
        .find { |dotgit| dotgit.exist? }

      raise Exception.new("Git hooks directory #{hooks_dir} was not found at any root of GitHub repository in #{cwd}") if !dotgit
      return if dotgit.directory? && File.read(dotgit + 'config').include?("\n\thooksPath = ")

      git = ENV['SET_GIT_HOOKS_DIR_GIT']
      git = 'git' if git.nil? || git.empty?
      out = `#{git} config core.hooksPath #{hooks_dir}`
      raise Exception.new("`#{git} config core.hooksPath #{hooks_dir}` failed with status #{$?.exitstatus}: #{out}") unless $?.success?
    end
  end
end
