use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn install(dir: impl AsRef<Path>) -> io::Result<()> {
    let git_var = env::var("GIT_SET_HOOKS_DIR_GIT");
    let git = match &git_var {
        Ok(var) if !var.is_empty() => var.as_str(),
        _ => "git",
    };
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Path {dir:?} is not a directory"),
        ));
    }
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
