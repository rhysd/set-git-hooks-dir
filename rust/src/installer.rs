use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

const CI_ENV_VARS: &[&str] = &[
    "SET_GIT_HOOKS_DIR_SKIP",
    "GITHUB_ACTION",
    "CI",
    "JENKINS_URL",
];

fn find_dot_git(hooks_dir: &Path, base_dir: &Path) -> io::Result<PathBuf> {
    let mut cur = base_dir;
    loop {
        let dir = cur.join(hooks_dir);
        if dir.is_dir() {
            let dot_git = cur.join(".git");
            if dot_git.exists() {
                return Ok(dot_git);
            }
        }
        let Some(parent) = cur.parent() else {
            let msg = format!(
                "Directory {hooks_dir:?} is not found at any root of Git repository in {base_dir:?}",
            );
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        };
        cur = parent;
    }
}

/// Setup the Git hooks directory path specified by `dir` argument.
pub fn setup(hooks_dir: impl AsRef<Path>, base_dir: impl AsRef<Path>) -> io::Result<()> {
    for var in CI_ENV_VARS {
        if matches!(env::var(var), Ok(v) if !v.is_empty()) {
            return Ok(());
        }
    }

    let hooks_dir = hooks_dir.as_ref();

    let dot_git = find_dot_git(hooks_dir, base_dir.as_ref())?;
    if dot_git.is_dir() {
        let config = File::open(dot_git.join("config"))?;
        for line in BufReader::new(config).lines() {
            if line?.starts_with("\thooksPath = ") {
                return Ok(());
            }
        }
    }

    let git_var = env::var("SET_GIT_HOOKS_DIR_GIT");
    let git = match &git_var {
        Ok(var) if !var.is_empty() => var.as_str(),
        _ => "git",
    };

    let mut cmd = Command::new(git);
    cmd.arg("config").arg("core.hooksPath").arg(hooks_dir);
    if let Some(root) = dot_git.parent() {
        cmd.current_dir(root);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{git} config core.hooksPath {hooks_dir:?}` failed: {stderr}"),
        ));
    }

    Ok(())
}
