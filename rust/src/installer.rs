use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

fn verify_hooks_dir(dir: &Path) -> io::Result<()> {
    let cwd = env::current_dir()?;
    let mut cur = cwd.as_path();
    loop {
        let hooks_dir = cur.join(dir);
        if hooks_dir.is_dir() && cur.join(".git").exists() {
            return Ok(());
        }
        let Some(parent) = cur.parent() else {
            let msg =
                format!("Directory {dir:?} is not found at any root of Git repository in {cwd:?}");
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        };
        cur = parent;
    }
}

pub fn setup(dir: impl AsRef<Path>) -> io::Result<()> {
    let dir = dir.as_ref();
    verify_hooks_dir(dir)?;

    let git_var = env::var("SET_GIT_HOOKS_DIR_GIT");
    let git = match &git_var {
        Ok(var) if !var.is_empty() => var.as_str(),
        _ => "git",
    };

    let output = Command::new(git)
        .arg("config")
        .arg("core.hooksPath")
        .arg(dir)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{git} config core.hooksPath {dir:?}` failed: {stderr}"),
        ));
    }

    Ok(())
}
