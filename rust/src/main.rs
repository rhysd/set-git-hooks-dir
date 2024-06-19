use std::env;
use std::io;
use std::process::exit;

fn main() -> io::Result<()> {
    let arg = env::args_os()
        .nth(1)
        .filter(|arg| arg != "-h" && arg != "--help");
    let Some(dir) = arg else {
        eprintln!("USAGE: set-git-hooks-dir path/to/dir\n\nSee https://github.com/rhysd/set-git-hooks-dir#readme");
        exit(1);
    };
    set_git_hooks_dir::setup(dir)
}
