use std::env;
use std::fs;
use std::process::Command;
use tempfile::{tempdir, TempDir};

// TODO: When adding more than one test cases, we need to add a static mutex and lock it before running
// each test (and unlock it after the test) otherwise all tests are run in parallel and tests will
// randomly fail because current directory is a global state.
struct Fixture {
    tmp: TempDir,
}

impl Default for Fixture {
    fn default() -> Self {
        let tmp = tempdir().unwrap();
        for name in ["GITHUB_ACTION", "CI"] {
            env::remove_var(name);
        }
        Self { tmp }
    }
}

#[test]
fn test_setup_hooks_in_git_config() {
    let fixture = Fixture::default();
    let dir = fixture.tmp.path();
    let config_path = dir.join(".git/config");

    // Create a hooks directory
    fs::create_dir(dir.join("test-hooks-dir")).unwrap();

    // The hooks directory exists but a Git repository has not been created yet
    let err = set_git_hooks_dir::setup("test-hooks-dir", dir).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains(r#"Directory "test-hooks-dir" is not found at any root of Git repository"#),
        "message={msg:?}",
    );

    // Create Git repository
    let out = Command::new("git")
        .arg("init")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr={:?}",
        String::from_utf8_lossy(&out.stderr),
    );

    // Git repository exists but the hooks directory does not exist
    let err = set_git_hooks_dir::setup("this-directory-does-not-exist", dir).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains(r#"Directory "this-directory-does-not-exist" is not found"#),
        "message={msg:?}",
    );

    // Hooks are not set on CI
    for name in [
        "GITHUB_ACTION",
        "CI",
        "SET_GIT_HOOKS_DIR_SKIP",
        "JENKINS_URL",
    ] {
        env::set_var(name, "true");
        set_git_hooks_dir::setup("test-hooks-dir", dir).unwrap();
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(
            !content.contains("\n\thooksPath = test-hooks-dir"),
            "env={name}, .git/config={content:?}",
        );
        env::remove_var(name);
    }

    // Normal case
    set_git_hooks_dir::setup("test-hooks-dir", dir).unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("\n\thooksPath = test-hooks-dir"),
        ".git/config={content:?}",
    );

    // Do not override existing configuration
    fs::create_dir(dir.join("second-test-hooks-dir")).unwrap();
    set_git_hooks_dir::setup("second-test-hooks-dir", dir).unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("\n\thooksPath = test-hooks-dir"),
        ".git/config={content:?}",
    );
}
