use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

fn find_dot_git(dir: &Path) -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    let mut cur = cwd.as_path();
    loop {
        let hooks_dir = cur.join(dir);
        if hooks_dir.is_dir() {
            let dot_git = cur.join(".git");
            if dot_git.exists() {
                return Ok(dot_git);
            }
        }
        let Some(parent) = cur.parent() else {
            let msg =
                format!("Directory {dir:?} is not found at any root of Git repository in {cwd:?}");
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        };
        cur = parent;
    }
}

/// Setup the Git hooks directory path specified by `dir` argument.
pub fn setup(dir: impl AsRef<Path>) -> io::Result<()> {
    for var in ["SET_GIT_HOOKS_DIR_SKIP", "GITHUB_ACTION", "CI"] {
        if matches!(env::var(var), Ok(v) if !v.is_empty()) {
            return Ok(());
        }
    }

    let dir = dir.as_ref();

    let dot_git = find_dot_git(dir)?;
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
    cmd.arg("config").arg("core.hooksPath").arg(dir);
    if let Some(root) = dot_git.parent() {
        cmd.current_dir(root);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{git} config core.hooksPath {dir:?}` failed: {stderr}"),
        ));
    }

    Ok(())
}
