require 'test/unit'
require 'pathname'
require 'tmpdir'
require 'fileutils'
require_relative '../lib/set_git_hooks_dir'

class TestSetGitHooks < Test::Unit::TestCase
  def setup
    @tmp_path = Pathname.new Dir.mktmpdir
    @prev_cwd = Pathname.pwd
    Dir.chdir @tmp_path
    `git init`
    assert $?.success?
    ['GITHUB_ACTION', 'CI'].each { |n| ENV.delete n }
  end

  def teardown
    Dir.chdir @prev_cwd
    FileUtils.remove_entry @tmp_path
  end

  def test_set_git_hooks_directory
    Dir.mkdir(@tmp_path + 'this-is-test')

    ENV['GITHUB_ACTION'] = 'true'
    SetGitHooksDir::setup 'this-is-test'
    config = File.read(@tmp_path + '.git/config')
    assert_no_match(/\thooksPath = /, config)
    ENV.delete 'GITHUB_ACTION'

    SetGitHooksDir::setup 'this-is-test'
    config = File.read(@tmp_path + '.git/config')
    assert_match(/\thooksPath = /, config)

    Dir.mkdir(@tmp_path + '2-this-is-test')
    SetGitHooksDir::setup '2-this-is-test'
    config = File.read(@tmp_path + '.git/config')
    assert_no_match(/\thooksPath = 2-this-is-test/, config)

    assert_raise { SetGitHooksDir::setup 'this-directory-does-not-exist' }
  end
end
