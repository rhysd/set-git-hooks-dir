use std::env;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    if env::args_os().skip(1).any(|a| a == "-h" || a == "--help") {
        eprintln!("Usage: set-git-hooks-dir [HOOKS_DIR [BASE_DIR]]");
        return Ok(());
    }

    let mut args = env::args_os().skip(1);
    let hooks_dir = args.next().unwrap_or_else(|| OsString::from(".git-hooks"));
    let base_dir = if let Some(a) = args.next() {
        PathBuf::from(a)
    } else {
        env::current_dir()?
    };

    eprintln!("Searching Git repository based on {base_dir:?} and configuring {hooks_dir:?} as a Git hooks directory...");
    set_git_hooks_dir::setup(hooks_dir, base_dir)?;
    eprintln!("Done.");
    Ok(())
}
