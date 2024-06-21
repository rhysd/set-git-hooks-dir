use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{tempdir, TempDir};

// TODO: When adding more than one test cases, we need to add a static mutex and lock it before running
// each test (and unlock it after the test) otherwise all tests are run in parallel and tests will
// randomly fail because current directory is a global state.
struct Fixture {
    cwd: PathBuf,
    _tmp: TempDir,
}

impl Default for Fixture {
    fn default() -> Self {
        let cwd = env::current_dir().unwrap();
        let tmp = tempdir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();
        let out = Command::new("git").arg("init").output().unwrap();
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            panic!("`git init` failed: {stderr}");
        }
        for name in ["GITHUB_ACTION", "CI"] {
            env::remove_var(name);
        }
        Self { cwd, _tmp: tmp }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        env::set_current_dir(&self.cwd).unwrap();
    }
}

#[test]
fn test_setup_git_config() {
    let _fixture = Fixture::default();

    set_git_hooks_dir::setup("this-directory-does-not-exist").unwrap_err();

    fs::create_dir("this-is-test").unwrap();

    // Hooks are not set on CI
    env::set_var("GITHUB_ACTION", "true");
    set_git_hooks_dir::setup("this-is-test").unwrap();
    let content = fs::read_to_string(".git/config").unwrap();
    assert!(!content.contains("hooksPath = this-is-test"), "{content:?}");
    env::remove_var("GITHUB_ACTION");

    // Normal case
    set_git_hooks_dir::setup("this-is-test").unwrap();
    let content = fs::read_to_string(".git/config").unwrap();
    assert!(content.contains("hooksPath = this-is-test"), "{content:?}");

    // Do not override existing configuration
    fs::create_dir("second-this-is-test").unwrap();
    set_git_hooks_dir::setup("second-this-is-test").unwrap();
    let content = fs::read_to_string(".git/config").unwrap();
    assert!(content.contains("hooksPath = this-is-test"), "{content:?}");
}
